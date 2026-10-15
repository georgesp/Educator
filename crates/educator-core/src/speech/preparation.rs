//! Preparation of the words of a series ahead of the child (SUB026705): when a series that reads
//! words starts, a background worker synthesizes each word then its sentence, in order, while the
//! instruction is being read. When the child asks for a word:
//!
//! - prepared: it is served at once, from memory;
//! - being synthesized: the request waits for that synthesis (never a second one);
//! - still waiting in the queue: it is synthesized at once on the caller's thread, and the worker
//!   pauses until it is done (the child's request always goes first).
//!
//! A child plays one series at a time: a new series takes the place of the others in the queue
//! (their words prepared stay served). A series released (abandoned, over) drops what it still had
//! to prepare and what was prepared. The engine given is usually the disk cache, so what was
//! prepared also stays there.

use super::{speed, SpeechSynthesizer};
use crate::exercises::Exercise;
use crate::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use uuid::Uuid;

/// Series whose prepared words are kept in memory at most; the oldest is dropped beyond (a series
/// left without being released).
pub const MAX_PREPARED_SERIES: usize = 2;

/// Words of a series prepared ahead of the child (a timed challenge has 90): the next ones are
/// added as the child goes ([`readings_ahead`]).
pub const WORDS_AHEAD: usize = 12;

/// What to prepare for the words of a series from exercise `from`: each word, then its sentence,
/// for at most [`WORDS_AHEAD`] words.
pub fn readings_ahead(exercises: &[Exercise], from: usize) -> Vec<Reading> {
    exercises
        .iter()
        .skip(from)
        .filter_map(|e| e.word.as_ref())
        .take(WORDS_AHEAD)
        .flat_map(|word| [word.speech(false, false), word.speech(true, false)])
        .flatten()
        .collect()
}

/// A text to read and its speed.
#[derive(Debug, Clone)]
pub struct Reading {
    pub text: String,
    pub speed: f64,
    /// Only what follows the first pause is heard: a word read in its carrier, « Le mot : jaune. »
    /// ([`super::wav::keep_after_first_pause`]).
    pub after_pause: bool,
}

impl Reading {
    pub fn new(text: impl Into<String>, speed: f64) -> Self {
        let speed = if speed.is_finite() {
            speed
        } else {
            speed::NORMAL
        };
        Self {
            text: text.into(),
            speed,
            after_pause: false,
        }
    }

    /// A reading of which only what follows the first pause is heard.
    pub fn after_pause(text: impl Into<String>, speed: f64) -> Self {
        Self {
            after_pause: true,
            ..Self::new(text, speed)
        }
    }

    /// The WAV of the reading: synthesized by `engine`, cut at its first pause when it asks for it.
    pub fn synthesize(&self, engine: &dyn SpeechSynthesizer) -> Result<Vec<u8>> {
        let wav = engine.synthesize(&self.text, self.speed)?;
        Ok(if self.after_pause {
            super::wav::keep_after_first_pause(&wav)
        } else {
            wav
        })
    }
}

impl PartialEq for Reading {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
            && self.speed.to_bits() == other.speed.to_bits()
            && self.after_pause == other.after_pause
    }
}

impl Eq for Reading {}

impl Hash for Reading {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text.hash(state);
        self.speed.to_bits().hash(state);
        self.after_pause.hash(state);
    }
}

#[derive(Default)]
struct State {
    /// What the worker still has to prepare, in order.
    queue: VecDeque<(Uuid, Reading)>,
    /// Being synthesized now (by the worker or for a child).
    in_progress: HashSet<Reading>,
    /// Prepared readings of each series still known.
    ready: HashMap<Uuid, HashMap<Reading, Arc<Vec<u8>>>>,
    /// The series in the order they were prepared (oldest first).
    series: VecDeque<Uuid>,
    /// Children's requests being synthesized: the worker waits for them.
    demands: usize,
    stopped: bool,
}

impl State {
    fn ready(&self, series: Uuid, reading: &Reading) -> Option<Arc<Vec<u8>>> {
        self.ready.get(&series)?.get(reading).cloned()
    }

    fn store(&mut self, series: Uuid, reading: Reading, wav: Vec<u8>) {
        if let Some(ready) = self.ready.get_mut(&series) {
            ready.insert(reading, Arc::new(wav));
        }
    }

    fn release(&mut self, series: Uuid) {
        self.queue.retain(|(owner, _)| *owner != series);
        self.ready.remove(&series);
        self.series.retain(|s| *s != series);
    }
}

struct Shared {
    state: Mutex<State>,
    changed: Condvar,
}

impl Shared {
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn wait<'a>(&self, guard: MutexGuard<'a, State>) -> MutexGuard<'a, State> {
        self.changed.wait(guard).unwrap_or_else(|e| e.into_inner())
    }
}

/// Prepares the readings of the series in the background. See the module documentation.
pub struct SpeechPreparer {
    engine: Arc<dyn SpeechSynthesizer>,
    shared: Arc<Shared>,
}

impl fmt::Debug for SpeechPreparer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.shared.lock();
        f.debug_struct("SpeechPreparer")
            .field("queued", &state.queue.len())
            .field("in_progress", &state.in_progress.len())
            .finish_non_exhaustive()
    }
}

impl SpeechPreparer {
    /// Starts the worker thread; it ends with the preparer.
    pub fn new(engine: Arc<dyn SpeechSynthesizer>) -> Self {
        let shared = Arc::new(Shared {
            state: Mutex::new(State::default()),
            changed: Condvar::new(),
        });
        let worker = (Arc::clone(&engine), Arc::clone(&shared));
        // Without the worker (thread refused), every reading is simply synthesized when asked.
        let _ = std::thread::Builder::new()
            .name("speech-preparation".to_owned())
            .spawn(move || work(worker.0.as_ref(), &worker.1));

        Self { engine, shared }
    }

    /// Queues the readings of a series, in order, after those of the same series already queued;
    /// the readings other series still had to prepare are dropped. Nothing is done when the voice
    /// is unavailable.
    pub fn prepare(&self, series: Uuid, readings: Vec<Reading>) {
        if readings.is_empty() || !self.engine.status().is_available {
            return;
        }

        let mut state = self.shared.lock();
        state.queue.retain(|(owner, _)| *owner == series);
        if !state.series.contains(&series) {
            state.series.push_back(series);
            state.ready.entry(series).or_default();
            while state.series.len() > MAX_PREPARED_SERIES {
                if let Some(oldest) = state.series.front().copied() {
                    state.release(oldest);
                }
            }
        }

        for reading in readings {
            let queued = state
                .queue
                .iter()
                .any(|(owner, r)| *owner == series && *r == reading);
            if !queued && state.ready(series, &reading).is_none() {
                state.queue.push_back((series, reading));
            }
        }

        drop(state);
        self.shared.changed.notify_all();
    }

    /// A reading of a series, prepared or not (see the module documentation).
    pub fn read(&self, series: Uuid, reading: &Reading) -> Result<Vec<u8>> {
        let mut state = self.shared.lock();
        loop {
            if let Some(wav) = state.ready(series, reading) {
                return Ok(wav.as_ref().clone());
            }
            if !state.in_progress.contains(reading) {
                break;
            }
            // Being synthesized: wait for it, then look again.
            state = self.shared.wait(state);
        }

        state
            .queue
            .retain(|(owner, r)| !(*owner == series && r == reading));
        let synthesis = Synthesis::start(&self.shared, state, reading, true);

        let outcome = reading.synthesize(self.engine.as_ref());
        if let Ok(wav) = &outcome {
            self.shared
                .lock()
                .store(series, reading.clone(), wav.clone());
        }
        drop(synthesis);
        outcome
    }

    /// Forgets a series: what it still had to prepare and what was prepared.
    pub fn release(&self, series: Uuid) {
        self.shared.lock().release(series);
        self.shared.changed.notify_all();
    }

    /// Nothing queued nor being synthesized (tests, diagnostics).
    pub fn is_idle(&self) -> bool {
        let state = self.shared.lock();
        state.queue.is_empty() && state.in_progress.is_empty()
    }

    /// How many readings of a series are prepared (tests, diagnostics).
    pub fn prepared_count(&self, series: Uuid) -> usize {
        self.shared
            .lock()
            .ready
            .get(&series)
            .map_or(0, HashMap::len)
    }
}

impl Drop for SpeechPreparer {
    fn drop(&mut self) {
        self.shared.lock().stopped = true;
        self.shared.changed.notify_all();
    }
}

/// The worker: the next reading of the queue, once no child's request is being synthesized.
fn work(engine: &dyn SpeechSynthesizer, shared: &Shared) {
    loop {
        let mut state = shared.lock();
        let (series, reading) = loop {
            if state.stopped {
                return;
            }
            if state.demands == 0 {
                if let Some(next) = state.queue.pop_front() {
                    if state.in_progress.contains(&next.1) || state.ready(next.0, &next.1).is_some()
                    {
                        continue;
                    }
                    break next;
                }
            }
            state = shared.wait(state);
        };
        let synthesis = Synthesis::start(shared, state, &reading, false);

        let outcome = reading.synthesize(engine);
        // A failure is not kept: the child's request tries again and reports it.
        if let Ok(wav) = outcome {
            shared.lock().store(series, reading.clone(), wav);
        }
        drop(synthesis);
    }
}

/// A reading being synthesized: marked in progress (and, for a child's request, pausing the
/// worker) until dropped, even if the engine panics, so that nobody waits for it forever.
struct Synthesis<'a> {
    shared: &'a Shared,
    reading: Reading,
    demand: bool,
}

impl<'a> Synthesis<'a> {
    fn start(
        shared: &'a Shared,
        mut state: MutexGuard<'_, State>,
        reading: &Reading,
        demand: bool,
    ) -> Self {
        state.in_progress.insert(reading.clone());
        if demand {
            state.demands += 1;
        }
        Self {
            shared,
            reading: reading.clone(),
            demand,
        }
    }
}

impl Drop for Synthesis<'_> {
    fn drop(&mut self) {
        let mut state = self.shared.lock();
        state.in_progress.remove(&self.reading);
        if self.demand {
            state.demands = state.demands.saturating_sub(1);
        }
        drop(state);
        self.shared.changed.notify_all();
    }
}
