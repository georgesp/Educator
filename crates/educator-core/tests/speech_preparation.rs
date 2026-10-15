//! SUB026705: the words of a series are prepared in order ahead of the child; the child's request
//! goes first, a reading is never synthesized twice, a released series stops its preparation.

use educator_core::speech::{
    speed, Reading, SpeechEngineStatus, SpeechPreparer, SpeechSynthesizer, MAX_PREPARED_SERIES,
};
use educator_core::{Error, Result};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;

/// An engine that writes down what it is asked and waits while its gate is closed.
#[derive(Default)]
struct GatedEngine {
    state: Mutex<(bool, Vec<String>)>,
    changed: Condvar,
    unavailable: bool,
}

impl GatedEngine {
    fn closed() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn open() -> Arc<Self> {
        let engine = Self::default();
        engine.state.lock().unwrap().0 = true;
        Arc::new(engine)
    }

    fn open_gate(&self) {
        self.state.lock().unwrap().0 = true;
        self.changed.notify_all();
    }

    fn asked(&self) -> Vec<String> {
        self.state.lock().unwrap().1.clone()
    }

    /// Waits until the engine was asked these texts.
    fn wait_asked(&self, expected: &[&str]) {
        let deadline = Instant::now() + Duration::from_secs(5);
        while self.asked() != expected {
            assert!(
                Instant::now() < deadline,
                "asked {:?}, expected {expected:?}",
                self.asked()
            );
            std::thread::sleep(Duration::from_millis(5));
        }
    }
}

impl SpeechSynthesizer for GatedEngine {
    fn status(&self) -> SpeechEngineStatus {
        if self.unavailable {
            SpeechEngineStatus::unavailable("tests")
        } else {
            SpeechEngineStatus::available("test-voice")
        }
    }

    fn synthesize(&self, text: &str, _speed: f64) -> Result<Vec<u8>> {
        let mut state = self.state.lock().unwrap();
        state.1.push(text.to_owned());
        while !state.0 {
            state = self.changed.wait(state).unwrap();
        }
        Ok(text.as_bytes().to_vec())
    }

    fn warm_up(&self, _speed: f64) {}
}

fn readings(texts: &[&str]) -> Vec<Reading> {
    texts
        .iter()
        .map(|t| Reading::new(*t, speed::WORDS))
        .collect()
}

fn word(text: &str) -> Reading {
    Reading::new(text, speed::WORDS)
}

fn wait_idle(preparer: &SpeechPreparer) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !preparer.is_idle() {
        assert!(Instant::now() < deadline, "the preparation never ended");
        std::thread::sleep(Duration::from_millis(5));
    }
}

const SERIES: &[&str] = &[
    "un stylo.",
    "J'écris avec un stylo.",
    "très,",
    "Il est très grand.",
    "une école.",
    "Je vais à l'école.",
];

#[test]
fn the_words_are_prepared_in_order_then_served_without_the_engine() {
    let engine = GatedEngine::open();
    let preparer = SpeechPreparer::new(engine.clone());
    let series = Uuid::now_v7();

    preparer.prepare(series, readings(SERIES));
    wait_idle(&preparer);

    assert_eq!(SERIES, engine.asked());
    assert_eq!(SERIES.len(), preparer.prepared_count(series));
    for text in SERIES {
        assert_eq!(text.as_bytes(), preparer.read(series, &word(text)).unwrap());
    }
    assert_eq!(SERIES.len(), engine.asked().len(), "served from memory");
}

#[test]
fn the_childs_request_goes_before_the_preparation() {
    let engine = GatedEngine::closed();
    let preparer = Arc::new(SpeechPreparer::new(engine.clone()));
    let series = Uuid::now_v7();
    preparer.prepare(series, readings(SERIES));
    engine.wait_asked(&SERIES[..1]);

    // The child goes straight to the third word while the first one is being synthesized.
    let child = {
        let preparer = Arc::clone(&preparer);
        std::thread::spawn(move || preparer.read(series, &word("une école.")))
    };
    engine.wait_asked(&[SERIES[0], "une école."]);
    engine.open_gate();

    assert_eq!(
        b"une \xc3\xa9cole.".to_vec(),
        child.join().unwrap().unwrap()
    );
    wait_idle(&preparer);
    assert_eq!(
        vec![
            "un stylo.",
            "une école.",
            "J'écris avec un stylo.",
            "très,",
            "Il est très grand.",
            "Je vais à l'école.",
        ],
        engine.asked(),
        "the word asked is synthesized once, before the rest of the preparation"
    );
}

#[test]
fn a_reading_being_prepared_is_awaited_never_synthesized_twice() {
    let engine = GatedEngine::closed();
    let preparer = Arc::new(SpeechPreparer::new(engine.clone()));
    let series = Uuid::now_v7();
    preparer.prepare(series, readings(&SERIES[..1]));
    engine.wait_asked(&SERIES[..1]);

    let child = {
        let preparer = Arc::clone(&preparer);
        std::thread::spawn(move || preparer.read(series, &word("un stylo.")))
    };
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(SERIES[..1], engine.asked(), "the child waits for it");
    engine.open_gate();

    assert_eq!(b"un stylo.".to_vec(), child.join().unwrap().unwrap());
    assert_eq!(SERIES[..1], engine.asked());
}

#[test]
fn a_released_series_stops_its_preparation() {
    let engine = GatedEngine::closed();
    let preparer = SpeechPreparer::new(engine.clone());
    let series = Uuid::now_v7();
    preparer.prepare(series, readings(SERIES));
    engine.wait_asked(&SERIES[..1]);

    preparer.release(series);
    engine.open_gate();
    wait_idle(&preparer);

    assert_eq!(SERIES[..1], engine.asked());
    assert_eq!(0, preparer.prepared_count(series));
}

#[test]
fn a_reading_never_prepared_is_synthesized_when_asked() {
    let engine = GatedEngine::open();
    let preparer = SpeechPreparer::new(engine.clone());

    let wav = preparer.read(Uuid::now_v7(), &word("hibou.")).unwrap();

    assert_eq!(b"hibou.".to_vec(), wav);
    assert_eq!(vec!["hibou."], engine.asked());
}

#[test]
fn only_the_last_series_are_kept() {
    let engine = GatedEngine::open();
    let preparer = SpeechPreparer::new(engine.clone());
    let series: Vec<Uuid> = (0..=MAX_PREPARED_SERIES).map(|_| Uuid::now_v7()).collect();

    for (i, id) in series.iter().enumerate() {
        preparer.prepare(*id, readings(&[&format!("mot {i}.")]));
        wait_idle(&preparer);
    }

    assert_eq!(0, preparer.prepared_count(series[0]));
    assert_eq!(1, preparer.prepared_count(series[MAX_PREPARED_SERIES]));
}

#[test]
fn nothing_is_prepared_without_a_voice() {
    let engine = Arc::new(GatedEngine {
        unavailable: true,
        ..GatedEngine::default()
    });
    let preparer = SpeechPreparer::new(engine.clone());

    preparer.prepare(Uuid::now_v7(), readings(SERIES));
    std::thread::sleep(Duration::from_millis(20));

    assert!(engine.asked().is_empty());
}

#[test]
fn a_failed_preparation_is_tried_again_when_asked() {
    struct Failing(Mutex<usize>);
    impl SpeechSynthesizer for Failing {
        fn status(&self) -> SpeechEngineStatus {
            SpeechEngineStatus::available("test-voice")
        }
        fn synthesize(&self, _text: &str, _speed: f64) -> Result<Vec<u8>> {
            let mut calls = self.0.lock().unwrap();
            *calls += 1;
            if *calls == 1 {
                Err(Error::SpeechUnavailable("crash".to_owned()))
            } else {
                Ok(vec![1])
            }
        }
        fn warm_up(&self, _speed: f64) {}
    }
    let engine = Arc::new(Failing(Mutex::new(0)));
    let preparer = SpeechPreparer::new(engine.clone());
    let series = Uuid::now_v7();
    preparer.prepare(series, readings(&["un stylo."]));
    wait_idle(&preparer);

    assert_eq!(vec![1], preparer.read(series, &word("un stylo.")).unwrap());
    assert_eq!(2, *engine.0.lock().unwrap());
}

#[test]
fn a_new_series_takes_the_place_of_the_others_in_the_queue() {
    let engine = GatedEngine::closed();
    let preparer = SpeechPreparer::new(engine.clone());
    let left = Uuid::now_v7();
    preparer.prepare(left, readings(SERIES));
    engine.wait_asked(&SERIES[..1]);

    // The child left that series without abandoning it, and starts another one.
    let next = Uuid::now_v7();
    preparer.prepare(next, readings(&["un hibou.", "Le hibou chante la nuit."]));
    engine.open_gate();
    wait_idle(&preparer);

    assert_eq!(
        vec!["un stylo.", "un hibou.", "Le hibou chante la nuit."],
        engine.asked()
    );
}
