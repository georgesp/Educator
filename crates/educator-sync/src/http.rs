//! A minimal HTTP/1.1 over TLS (ADR 0007 section 7): one request per connection, a JSON body of
//! known length (`Content-Length`), bounded sizes. Enough for four routes between our two
//! applications, and readable with `curl --insecure` for a diagnosis.

use std::fmt;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::IpAddr;

/// Largest head (request line and headers).
const MAX_HEAD: usize = 8 * 1024;

/// Largest body accepted (a batch is far smaller).
pub const MAX_BODY: usize = 4 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    /// Header names in lower case.
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v.as_str())
    }

    /// The token of `Authorization: Bearer …`.
    pub fn bearer(&self) -> Option<&str> {
        self.header("authorization")?
            .strip_prefix("Bearer ")
            .map(str::trim)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub status: u16,
    pub body: Vec<u8>,
}

impl Response {
    pub fn json(status: u16, body: &impl serde::Serialize) -> Self {
        Self {
            status,
            body: serde_json::to_vec(body).unwrap_or_default(),
        }
    }
}

/// A request or response that cannot be read.
#[derive(Debug)]
pub enum HttpError {
    Io(std::io::Error),
    Malformed(&'static str),
    TooLarge,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::Io(e) => write!(f, "{e}"),
            HttpError::Malformed(what) => write!(f, "Malformed HTTP: {what}"),
            HttpError::TooLarge => f.write_str("Message too large"),
        }
    }
}

impl std::error::Error for HttpError {}

impl From<std::io::Error> for HttpError {
    fn from(e: std::io::Error) -> Self {
        HttpError::Io(e)
    }
}

/// The start line and the headers.
fn read_head(reader: &mut impl BufRead) -> Result<(String, Vec<(String, String)>), HttpError> {
    let mut size = 0;
    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        let read = reader
            .by_ref()
            .take((MAX_HEAD - size + 1) as u64)
            .read_line(&mut line)?;
        if read == 0 {
            return Err(HttpError::Malformed("connection closed"));
        }
        size += read;
        if size > MAX_HEAD {
            return Err(HttpError::TooLarge);
        }
        let line = line.trim_end_matches(['\r', '\n']).to_owned();
        if line.is_empty() {
            break;
        }
        lines.push(line);
    }
    let mut lines = lines.into_iter();
    let start = lines.next().ok_or(HttpError::Malformed("empty"))?;
    let headers = lines
        .map(|l| {
            let (name, value) = l.split_once(':').ok_or(HttpError::Malformed("header"))?;
            Ok((name.trim().to_ascii_lowercase(), value.trim().to_owned()))
        })
        .collect::<Result<Vec<_>, HttpError>>()?;
    Ok((start, headers))
}

fn read_body(
    reader: &mut impl Read,
    headers: &[(String, String)],
    max_body: usize,
) -> Result<Vec<u8>, HttpError> {
    let length = match headers.iter().find(|(n, _)| n == "content-length") {
        None => 0,
        Some((_, v)) => v
            .parse::<usize>()
            .map_err(|_| HttpError::Malformed("content-length"))?,
    };
    if length > max_body {
        return Err(HttpError::TooLarge);
    }
    let mut body = vec![0; length];
    reader.read_exact(&mut body)?;
    Ok(body)
}

pub fn read_request(stream: &mut impl Read, max_body: usize) -> Result<Request, HttpError> {
    let mut reader = BufReader::new(stream);
    let (start, headers) = read_head(&mut reader)?;
    let mut parts = start.split(' ');
    let (Some(method), Some(path), Some(version)) = (parts.next(), parts.next(), parts.next())
    else {
        return Err(HttpError::Malformed("request line"));
    };
    if !version.starts_with("HTTP/1.") {
        return Err(HttpError::Malformed("version"));
    }
    let body = read_body(&mut reader, &headers, max_body)?;
    Ok(Request {
        method: method.to_owned(),
        path: path.to_owned(),
        headers,
        body,
    })
}

fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        409 => "Conflict",
        413 => "Payload Too Large",
        429 => "Too Many Requests",
        503 => "Service Unavailable",
        _ => "Error",
    }
}

pub fn write_response(stream: &mut impl Write, response: &Response) -> std::io::Result<()> {
    let head = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        response.status,
        reason(response.status),
        response.body.len()
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(&response.body)?;
    stream.flush()
}

pub fn write_request(
    stream: &mut impl Write,
    path: &str,
    bearer: Option<&str>,
    body: &[u8],
) -> std::io::Result<()> {
    let authorization = bearer
        .map(|t| format!("Authorization: Bearer {t}\r\n"))
        .unwrap_or_default();
    let head = format!(
        "POST {path} HTTP/1.1\r\nHost: educator.local\r\n{authorization}Content-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()
}

pub fn read_response(stream: &mut impl Read, max_body: usize) -> Result<Response, HttpError> {
    let mut reader = BufReader::new(stream);
    let (start, headers) = read_head(&mut reader)?;
    let status = start
        .split(' ')
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .ok_or(HttpError::Malformed("status line"))?;
    let body = read_body(&mut reader, &headers, max_body)?;
    Ok(Response { status, body })
}

/// What answers the requests (the routes of `service::ServerHandler`, served by `server`).
pub trait Handler: Send + Sync {
    fn handle(&self, request: &Request, from: IpAddr) -> Response;
}

/// Whether an address belongs to the local network (or this machine): private IPv4 ranges,
/// link-local, loopback, unique local and link-local IPv6.
pub fn is_local_address(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_private() || v4.is_link_local() || v4.is_loopback(),
        IpAddr::V6(v6) => {
            let first = v6.segments()[0];
            v6.is_loopback()
                || (first & 0xfe00) == 0xfc00
                || (first & 0xffc0) == 0xfe80
                || v6
                    .to_ipv4_mapped()
                    .is_some_and(|v4| is_local_address(IpAddr::V4(v4)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_local_network_is_local() {
        for local in [
            "192.168.1.10",
            "10.0.0.2",
            "172.20.1.1",
            "169.254.3.4",
            "127.0.0.1",
            "::1",
            "fd12::1",
            "fe80::1",
            "::ffff:192.168.0.3",
        ] {
            assert!(is_local_address(local.parse().unwrap()), "{local}");
        }
        for outside in ["8.8.8.8", "172.32.0.1", "2001:db8::1", "::ffff:1.1.1.1"] {
            assert!(!is_local_address(outside.parse().unwrap()), "{outside}");
        }
    }

    #[test]
    fn a_request_round_trips() {
        let mut wire = Vec::new();
        write_request(&mut wire, "/v1/pull", Some("abc"), br#"{"a":1}"#).unwrap();

        let request = read_request(&mut wire.as_slice(), MAX_BODY).unwrap();

        assert_eq!(
            ("POST", "/v1/pull"),
            (request.method.as_str(), request.path.as_str())
        );
        assert_eq!(Some("abc"), request.bearer());
        assert_eq!(br#"{"a":1}"#.to_vec(), request.body);
    }

    #[test]
    fn a_response_round_trips() {
        let mut wire = Vec::new();
        write_response(
            &mut wire,
            &Response::json(409, &serde_json::json!({"code": "x"})),
        )
        .unwrap();

        let response = read_response(&mut wire.as_slice(), MAX_BODY).unwrap();

        assert_eq!(409, response.status);
        assert_eq!(br#"{"code":"x"}"#.to_vec(), response.body);
    }

    #[test]
    fn oversized_or_malformed_messages_are_refused() {
        let big = format!(
            "POST / HTTP/1.1\r\nContent-Length: {}\r\n\r\n",
            MAX_BODY + 1
        );
        assert!(matches!(
            read_request(&mut big.as_bytes(), MAX_BODY),
            Err(HttpError::TooLarge)
        ));
        let long_head = format!("POST / HTTP/1.1\r\nX: {}\r\n\r\n", "a".repeat(MAX_HEAD));
        assert!(matches!(
            read_request(&mut long_head.as_bytes(), MAX_BODY),
            Err(HttpError::TooLarge)
        ));
        for text in [
            "",
            "GARBAGE\r\n\r\n",
            "POST / SMTP\r\n\r\n",
            "POST / HTTP/1.1\r\nbad\r\n\r\n",
        ] {
            assert!(
                read_request(&mut text.as_bytes(), MAX_BODY).is_err(),
                "{text}"
            );
        }
    }
}
