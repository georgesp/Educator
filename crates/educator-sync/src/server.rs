//! The HTTPS server of the computer (SUB027610): it listens only while the application asks it to
//! (ADR 0007, section 1), refuses any connection that does not come from the local network,
//! bounds the connections, the sizes and the time of each request, and hands every request to a
//! [`Handler`]. Desktop only.

pub use crate::http::{is_local_address, Handler};
use crate::http::{read_request, write_response, HttpError, Response, MAX_BODY};
use crate::tls::{TlsError, TlsIdentity};
use std::fmt;
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// The private IPv4 addresses of this computer, the home network first (for the QR code).
pub fn local_addresses() -> Vec<String> {
    let Ok(interfaces) = if_addrs::get_if_addrs() else {
        return Vec::new();
    };
    let mut addresses: Vec<std::net::Ipv4Addr> = interfaces
        .iter()
        .filter(|i| !i.is_loopback())
        .filter_map(|i| match i.ip() {
            IpAddr::V4(v4) if v4.is_private() => Some(v4),
            _ => None,
        })
        .collect();
    // 192.168.x.x (boxes) first, then 10.x and 172.16-31.x (often a VPN or a virtual machine).
    addresses.sort_by_key(|a| (a.octets()[0] != 192, *a));
    addresses.dedup();
    addresses.iter().map(ToString::to_string).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerOptions {
    /// The port wished (0: any); another one is taken when it is busy.
    pub port: u16,
    pub max_connections: usize,
    /// Longest wait for one read or write.
    pub timeout: Duration,
    /// Longest time for a whole request and its response: a peer sending a byte now and then
    /// cannot keep a connection.
    pub deadline: Duration,
    pub max_body: usize,
}

impl Default for ServerOptions {
    fn default() -> Self {
        Self {
            port: 0,
            max_connections: 8,
            timeout: Duration::from_secs(20),
            deadline: Duration::from_secs(90),
            max_body: MAX_BODY,
        }
    }
}

/// The server could not start (port, firewall, certificate).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerError(pub String);

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Synchronisation server: {}", self.0)
    }
}

impl std::error::Error for ServerError {}

impl From<TlsError> for ServerError {
    fn from(e: TlsError) -> Self {
        ServerError(e.to_string())
    }
}

/// A server listening; stopped by [`RunningServer::stop`] or when dropped.
pub struct RunningServer {
    port: u16,
    stop: Arc<AtomicBool>,
    accept: Option<JoinHandle<()>>,
}

impl fmt::Debug for RunningServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunningServer")
            .field("port", &self.port)
            .finish_non_exhaustive()
    }
}

fn bind(port: u16) -> std::io::Result<TcpListener> {
    TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port)))
}

impl RunningServer {
    pub fn start(
        identity: &TlsIdentity,
        options: ServerOptions,
        handler: Arc<dyn Handler>,
    ) -> Result<Self, ServerError> {
        let config = identity.server_config()?;
        let listener = match bind(options.port) {
            Ok(listener) => listener,
            Err(_) if options.port != 0 => bind(0).map_err(|e| ServerError(e.to_string()))?,
            Err(e) => return Err(ServerError(e.to_string())),
        };
        listener
            .set_nonblocking(true)
            .map_err(|e| ServerError(e.to_string()))?;
        let port = listener
            .local_addr()
            .map_err(|e| ServerError(e.to_string()))?
            .port();
        let stop = Arc::new(AtomicBool::new(false));
        let active = Arc::new(AtomicUsize::new(0));
        let flag = Arc::clone(&stop);
        let accept = std::thread::Builder::new()
            .name("educator-sync-server".to_owned())
            .spawn(move || {
                while !flag.load(Ordering::SeqCst) {
                    match listener.accept() {
                        Ok((stream, peer)) => {
                            if !is_local_address(peer.ip()) {
                                log::warn!("Synchronisation: connection from outside the local network refused.");
                                continue;
                            }
                            if active.load(Ordering::SeqCst) >= options.max_connections {
                                log::warn!("Synchronisation: too many connections, one refused.");
                                continue;
                            }
                            active.fetch_add(1, Ordering::SeqCst);
                            let (config, handler, done) =
                                (Arc::clone(&config), Arc::clone(&handler), Arc::clone(&active));
                            let spawned = std::thread::Builder::new()
                                .name("educator-sync-connection".to_owned())
                                .spawn(move || {
                                    serve(stream, peer.ip(), config, handler.as_ref(), options);
                                    done.fetch_sub(1, Ordering::SeqCst);
                                });
                            if spawned.is_err() {
                                active.fetch_sub(1, Ordering::SeqCst);
                                log::error!("Synchronisation: no thread for a connection.");
                            }
                        }
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            std::thread::sleep(Duration::from_millis(50));
                        }
                        Err(e) => {
                            log::warn!("Synchronisation: accept failed: {e}");
                            std::thread::sleep(Duration::from_millis(200));
                        }
                    }
                }
            })
            .map_err(|e| ServerError(e.to_string()))?;
        log::info!("Synchronisation server listening on port {port}.");
        Ok(Self {
            port,
            stop,
            accept: Some(accept),
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(thread) = self.accept.take() {
            let _ = thread.join();
            log::info!("Synchronisation server stopped.");
        }
    }
}

impl Drop for RunningServer {
    fn drop(&mut self) {
        self.stop();
    }
}

/// A connection that gives up past its deadline.
struct Deadline {
    stream: TcpStream,
    until: Instant,
    timeout: Duration,
}

impl Deadline {
    fn left(&self) -> std::io::Result<Duration> {
        self.until
            .checked_duration_since(Instant::now())
            .filter(|d| !d.is_zero())
            .map(|d| d.min(self.timeout))
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::TimedOut, "deadline passed"))
    }
}

impl Read for Deadline {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let left = self.left()?;
        self.stream.set_read_timeout(Some(left))?;
        self.stream.read(buf)
    }
}

impl Write for Deadline {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let left = self.left()?;
        self.stream.set_write_timeout(Some(left))?;
        self.stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
}

fn serve(
    stream: TcpStream,
    from: IpAddr,
    config: Arc<rustls::ServerConfig>,
    handler: &dyn Handler,
    options: ServerOptions,
) {
    if stream.set_nonblocking(false).is_err() {
        return;
    }
    let stream = Deadline {
        stream,
        until: Instant::now() + options.deadline,
        timeout: options.timeout,
    };
    let Ok(connection) = rustls::ServerConnection::new(config) else {
        return;
    };
    let mut tls = rustls::StreamOwned::new(connection, stream);
    let response = match read_request(&mut tls, options.max_body) {
        Ok(request) => handler.handle(&request, from),
        Err(HttpError::TooLarge) => Response::json(
            413,
            &crate::wire::ErrorResponse {
                protocol: crate::wire::PROTOCOL_VERSION,
                code: "invalid".to_owned(),
                message: "Message too large.".to_owned(),
            },
        ),
        Err(e) => {
            log::debug!("Synchronisation: request not read: {e}");
            return;
        }
    };
    if let Err(e) = write_response(&mut tls, &response) {
        log::debug!("Synchronisation: response not written: {e}");
        return;
    }
    tls.conn.send_close_notify();
    let _ = std::io::Write::flush(&mut tls);
}
