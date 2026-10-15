#![cfg(not(any(target_os = "ios", target_os = "android")))]

use educator_content::speech::{
    build_input_line, locate, to_length_scale, PiperLocatorOptions, PiperPlatform,
    PiperSpeechSynthesizer, SpeechCache, IDLE_TIMEOUT,
};
use educator_core::clock::ManualClock;
use educator_core::speech::{SpeechEngineStatus, SpeechSynthesizer, UnavailableSpeechSynthesizer};
use educator_core::{Error, Result};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

fn create_layout(root: &Path) -> PathBuf {
    std::fs::create_dir_all(root.join("macos").join("espeak-ng-data")).unwrap();
    std::fs::write(root.join("macos").join("piper"), "").unwrap();
    std::fs::create_dir_all(root.join("voices")).unwrap();
    std::fs::write(root.join("voices").join("fr_FR-siwis-medium.onnx"), "").unwrap();
    std::fs::write(
        root.join("voices").join("fr_FR-siwis-medium.onnx.json"),
        "{}",
    )
    .unwrap();
    root.to_path_buf()
}

fn options(explicit_root: Option<PathBuf>, base: &Path) -> PiperLocatorOptions {
    PiperLocatorOptions {
        explicit_root,
        voice_name: "fr_FR-siwis-medium".to_owned(),
        packaged_directories: Vec::new(),
        base_directory: base.to_path_buf(),
        platform: PiperPlatform::MACOS,
    }
}

#[test]
fn locate_complete_layout_returns_the_installation() {
    let temp = tempfile::tempdir().unwrap();
    let root = create_layout(&temp.path().join("piper"));

    let installation = locate(&options(Some(root.clone()), temp.path())).unwrap();

    assert_eq!(
        root.join("macos").join("piper"),
        installation.executable_path
    );
    assert_eq!(
        root.join("voices").join("fr_FR-siwis-medium.onnx"),
        installation.model_path
    );
    assert_eq!(
        PathBuf::from(format!("{}.json", installation.model_path.display())),
        installation.model_config_path
    );
    assert_eq!("fr_FR-siwis-medium", installation.voice_name);
}

#[test]
fn locate_without_explicit_root_finds_vendor_piper_above_the_application() {
    let temp = tempfile::tempdir().unwrap();
    create_layout(&temp.path().join("vendor").join("piper"));
    let app = temp
        .path()
        .join("web")
        .join("src-tauri")
        .join("target")
        .join("debug");
    std::fs::create_dir_all(&app).unwrap();

    assert!(locate(&options(None, &app)).is_ok());
}

#[test]
fn locate_packaged_application_prefers_its_piper_folder() {
    let temp = tempfile::tempdir().unwrap();
    let resources = temp.path().join("app").join("resources");
    let packaged = create_layout(&resources.join("piper"));
    create_layout(&temp.path().join("vendor").join("piper"));
    let mut options = options(None, &temp.path().join("app"));
    options.packaged_directories = vec![resources];

    let installation = locate(&options).unwrap();

    assert!(installation.executable_path.starts_with(packaged));
}

#[test]
fn locate_nothing_installed_explains_how_to_fetch_piper() {
    let temp = tempfile::tempdir().unwrap();
    let empty = temp.path().join("empty");
    std::fs::create_dir_all(&empty).unwrap();

    assert!(locate(&options(None, &empty))
        .unwrap_err()
        .contains("fetch-piper"));
}

#[test]
fn locate_missing_voice_reports_the_voice() {
    let temp = tempfile::tempdir().unwrap();
    let root = create_layout(&temp.path().join("piper"));
    let mut options = options(Some(root), temp.path());
    options.voice_name = "fr_FR-tom-medium".to_owned();

    assert!(locate(&options).unwrap_err().contains("fr_FR-tom-medium"));
}

#[test]
fn locate_missing_executable_reports_it() {
    let temp = tempfile::tempdir().unwrap();
    let root = create_layout(&temp.path().join("piper"));
    std::fs::remove_file(root.join("macos").join("piper")).unwrap();

    assert!(locate(&options(Some(root), temp.path()))
        .unwrap_err()
        .contains("executable"));
}

#[test]
fn build_input_line_is_ascii_json_keeping_accents() {
    let line = build_input_line("L'élève dit \"ça\"\nfin");

    assert!(line.is_ascii(), "{line}");
    let json: serde_json::Value = serde_json::from_str(&line).unwrap();
    assert_eq!("L'élève dit \"ça\" fin", json["text"]);
}

#[test]
fn to_length_scale_inverts_and_clamps_the_speed() {
    for (speed, expected) in [
        (1.0, 1.0),
        (1.25, 0.8),
        (0.5, 2.0),
        (10.0, 0.67),
        (0.1, 2.0),
        (f64::NAN, 1.0),
    ] {
        assert_eq!(expected, to_length_scale(speed), "{speed}");
    }
}

#[test]
fn unavailable_synthesizer_fails_with_the_problem() {
    let synthesizer = UnavailableSpeechSynthesizer::new("Piper not found");

    assert!(!synthesizer.status().is_available);
    assert_eq!(
        Err(Error::SpeechUnavailable("Piper not found".to_owned())),
        synthesizer.synthesize("Bonjour", 1.0)
    );
}

#[derive(Default)]
struct CountingEngine(AtomicUsize);

impl SpeechSynthesizer for CountingEngine {
    fn status(&self) -> SpeechEngineStatus {
        SpeechEngineStatus::available("test-voice")
    }

    fn synthesize(&self, _text: &str, _speed: f64) -> Result<Vec<u8>> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(vec![1, 2, 3])
    }

    fn warm_up(&self, _speed: f64) {}
}

#[test]
fn same_sentence_is_synthesized_once_per_voice_and_speed() {
    let temp = tempfile::tempdir().unwrap();
    let engine = Arc::new(CountingEngine::default());
    let cache = SpeechCache::new(engine.clone(), temp.path().join("cache"));

    let first = cache.synthesize("sept plus cinq", 0.9).unwrap();
    let second = cache.synthesize("sept plus cinq", 0.9).unwrap();
    cache.synthesize("sept plus cinq", 1.0).unwrap();

    assert_eq!(first, second);
    assert_eq!(2, engine.0.load(Ordering::SeqCst));
}

/// An engine answering `size` bytes for every text.
struct SizedEngine(usize);

impl SpeechSynthesizer for SizedEngine {
    fn status(&self) -> SpeechEngineStatus {
        SpeechEngineStatus::available("test-voice")
    }

    fn synthesize(&self, _text: &str, _speed: f64) -> Result<Vec<u8>> {
        Ok(vec![7; self.0])
    }

    fn warm_up(&self, _speed: f64) {}
}

#[test]
fn the_cache_never_grows_above_its_cap() {
    let temp = tempfile::tempdir().unwrap();
    let clock = Arc::new(ManualClock::new(1_800_000_000_000));
    let cache = SpeechCache::with_limit(
        Arc::new(SizedEngine(1000)),
        temp.path().join("cache"),
        10_000,
        clock.clone(),
    );

    for i in 0..50 {
        clock.advance(1000);
        cache.synthesize(&format!("mot {i}"), 0.85).unwrap();
        assert!(cache.size() <= 10_000, "{} bytes after {i}", cache.size());
    }
    assert!(cache.size() >= 8_000);
}

#[test]
fn a_file_read_again_stays_longer_than_a_file_never_read() {
    let temp = tempfile::tempdir().unwrap();
    let clock = Arc::new(ManualClock::new(1_800_000_000_000));
    let engine = Arc::new(CountingEngine::default());
    let cache =
        SpeechCache::with_limit(engine.clone(), temp.path().join("cache"), 10, clock.clone());

    // 3 bytes each: three files fit, a fourth goes above the cap.
    for text in ["un", "deux", "trois"] {
        clock.advance(1000);
        cache.synthesize(text, 1.0).unwrap();
    }
    clock.advance(1000);
    cache.synthesize("un", 1.0).unwrap(); // read again: now the most recent
    clock.advance(1000);
    cache.synthesize("quatre", 1.0).unwrap();

    assert_eq!(4, engine.0.load(Ordering::SeqCst));
    assert!(cache.contains("un", 1.0), "« un » was read again");
    assert!(
        !cache.contains("deux", 1.0),
        "« deux » was never read again"
    );
    assert!(cache.contains("quatre", 1.0));
}

#[test]
fn the_cache_counts_what_earlier_runs_left() {
    let temp = tempfile::tempdir().unwrap();
    let directory = temp.path().join("cache");
    std::fs::create_dir_all(&directory).unwrap();
    for i in 0..20 {
        std::fs::write(directory.join(format!("{i:05}.wav")), [1; 1000]).unwrap();
    }
    let cache = SpeechCache::with_limit(
        Arc::new(SizedEngine(1000)),
        directory,
        10_000,
        Arc::new(ManualClock::new(1_800_000_000_000)),
    );

    cache.synthesize("nouveau", 1.0).unwrap();

    assert!(cache.size() <= 10_000);
    assert!(cache.contains("nouveau", 1.0));
}

#[test]
fn a_forgotten_reading_is_synthesized_again() {
    let temp = tempfile::tempdir().unwrap();
    let engine = Arc::new(CountingEngine::default());
    let cache = SpeechCache::new(engine.clone(), temp.path().join("cache"));

    cache.synthesize("un stylo.", 0.85).unwrap();
    cache.forget("un stylo.", 0.85);
    cache.synthesize("un stylo.", 0.85).unwrap();

    assert_eq!(2, engine.0.load(Ordering::SeqCst));
}

/// Runs the real Piper engine when it is installed (scripts/fetch-piper). Without Piper the test
/// returns early and says so: CI machines do not download the engine.
fn installed_piper() -> Option<PiperSpeechSynthesizer> {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    match locate(&PiperLocatorOptions::from_environment(Vec::new(), base)) {
        Ok(installation) => Some(PiperSpeechSynthesizer::new(installation)),
        Err(problem) => {
            eprintln!("Piper is not installed: test skipped ({problem}).");
            None
        }
    }
}

#[test]
fn piper_addition_is_read_in_less_than_half_a_second_after_warm_up() {
    let Some(synthesizer) = installed_piper() else {
        return;
    };
    synthesizer.warm_up(0.9);

    let started = Instant::now();
    let wav = synthesizer
        .synthesize(
            &educator_core::speech::normalize("Combien font 7 plus 5 ?"),
            0.9,
        )
        .unwrap();
    let elapsed = started.elapsed();
    eprintln!(
        "Synthesis: {} ms, {} bytes.",
        elapsed.as_millis(),
        wav.len()
    );

    assert_eq!(b"RIFF", &wav[0..4]);
    assert_eq!(b"WAVE", &wav[8..12]);
    assert!(
        elapsed.as_millis() < 500,
        "took {} ms",
        started.elapsed().as_millis()
    );
}

#[test]
fn piper_changing_speed_still_works() {
    let Some(synthesizer) = installed_piper() else {
        return;
    };

    let normal = synthesizer
        .synthesize("Un, deux, trois, quatre, cinq.", 1.0)
        .unwrap();
    let slow = synthesizer
        .synthesize("Un, deux, trois, quatre, cinq.", 0.5)
        .unwrap();

    assert!(
        slow.len() > normal.len(),
        "slower speech must give a longer audio file"
    );
}

/// A fake engine (a shell script answering like `piper --json-input`): one JSON line in, one WAV
/// file written in the output folder, its path printed. A text containing « crash » makes it exit.
#[cfg(unix)]
fn fake_piper(temp: &Path, clock: Arc<ManualClock>) -> PiperSpeechSynthesizer {
    use std::os::unix::fs::PermissionsExt;

    let root = create_layout(&temp.join("piper"));
    let platform = PiperPlatform::current();
    let engine = root.join(platform.folder_name);
    std::fs::create_dir_all(engine.join("espeak-ng-data")).unwrap();
    let script = engine.join(platform.executable_name);
    std::fs::write(
        &script,
        r#"#!/bin/sh
while [ $# -gt 0 ]; do
  case "$1" in
    --output_dir) OUT="$2"; shift ;;
    --length_scale) SCALE="$2"; shift ;;
  esac
  shift
done
n=0
while IFS= read -r line; do
  case "$line" in *crash*) exit 3 ;; esac
  n=$((n+1))
  printf 'RIFF....WAVE %s %s' "$SCALE" "$line" > "$OUT/$n.wav"
  echo "$OUT/$n.wav"
done
"#,
    )
    .unwrap();
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    let mut options = options(Some(root), temp);
    options.platform = platform;
    PiperSpeechSynthesizer::with_clock(locate(&options).unwrap(), clock)
}

#[cfg(unix)]
fn text_of(wav: Vec<u8>) -> String {
    String::from_utf8(wav).unwrap()
}

#[cfg(unix)]
#[test]
fn piper_protocol_with_a_fake_engine() {
    let temp = tempfile::tempdir().unwrap();
    let synthesizer = fake_piper(temp.path(), Arc::new(ManualClock::new(0)));

    let first = text_of(synthesizer.synthesize("sept plus cinq", 1.0).unwrap());
    let second = text_of(synthesizer.synthesize("élève", 1.0).unwrap());
    let slower = text_of(synthesizer.synthesize("lent", 0.5).unwrap());

    assert_eq!(r#"RIFF....WAVE 1 {"text":"sept plus cinq"}"#, first);
    assert!(
        second.starts_with("RIFF....WAVE 1 {") && second.ends_with("ve\"}"),
        "{second}"
    );
    assert_eq!(r#"RIFF....WAVE 2 {"text":"lent"}"#, slower);
}

/// SUB026708: one process per speed; going back and forth never restarts Piper.
#[cfg(unix)]
#[test]
fn several_speeds_are_served_without_restarting_piper() {
    let temp = tempfile::tempdir().unwrap();
    let synthesizer = fake_piper(temp.path(), Arc::new(ManualClock::new(0)));

    for _ in 0..3 {
        let instruction = text_of(synthesizer.synthesize("Écris le mot.", 0.9).unwrap());
        let word = text_of(synthesizer.synthesize("un stylo.", 0.85).unwrap());
        let sentence = text_of(
            synthesizer
                .synthesize("J'écris avec un stylo.", 0.85)
                .unwrap(),
        );
        let slow = text_of(synthesizer.synthesize("un stylo.", 0.5).unwrap());
        assert!(
            instruction.starts_with("RIFF....WAVE 1.11 "),
            "{instruction}"
        );
        // The word and its sentence at the same speed (0.85 → length scale 1.18).
        assert!(word.starts_with("RIFF....WAVE 1.18 "), "{word}");
        assert!(sentence.starts_with("RIFF....WAVE 1.18 "), "{sentence}");
        assert!(slow.starts_with("RIFF....WAVE 2 "), "{slow}");
    }

    assert_eq!(3, synthesizer.process_starts());
    assert_eq!(vec![1.11, 1.18, 2.0], synthesizer.running_length_scales());
}

/// SUB026708: a process not warmed up stops after 2 minutes without a request; the warmed up ones
/// stay.
#[cfg(unix)]
#[test]
fn an_idle_process_stops_unless_it_is_kept_warm() {
    let temp = tempfile::tempdir().unwrap();
    let clock = Arc::new(ManualClock::new(1_800_000_000_000));
    let synthesizer = fake_piper(temp.path(), clock.clone());
    synthesizer.warm_up(0.9);
    synthesizer.warm_up(0.85);
    synthesizer.synthesize("un stylo.", 0.5).unwrap();

    clock.advance(IDLE_TIMEOUT.as_millis() as i64 - 1000);
    synthesizer.stop_idle_processes();
    assert_eq!(vec![1.11, 1.18, 2.0], synthesizer.running_length_scales());

    clock.advance(1000);
    synthesizer.stop_idle_processes();
    assert_eq!(vec![1.11, 1.18], synthesizer.running_length_scales());

    // Asked again: started again.
    synthesizer.synthesize("un stylo.", 0.5).unwrap();
    assert_eq!(4, synthesizer.process_starts());
}

/// A crashed process is restarted at the next request.
#[cfg(unix)]
#[test]
fn a_crashed_process_is_restarted() {
    let temp = tempfile::tempdir().unwrap();
    let synthesizer = fake_piper(temp.path(), Arc::new(ManualClock::new(0)));
    synthesizer.synthesize("avant", 0.85).unwrap();

    let crash = synthesizer.synthesize("crash", 0.85);
    let after = text_of(synthesizer.synthesize("après", 0.85).unwrap());

    assert!(matches!(crash, Err(Error::SpeechUnavailable(_))));
    assert!(after.contains("apr"), "{after}");
    assert_eq!(2, synthesizer.process_starts());
}

/// SUB026708 with the real engine: the instruction, the words and the slow listening each keep
/// their process; going back and forth never restarts Piper.
#[test]
fn piper_serves_several_speeds_without_restarting() {
    let Some(synthesizer) = installed_piper() else {
        return;
    };
    synthesizer.warm_up(0.9);
    synthesizer.warm_up(0.85);

    for _ in 0..2 {
        synthesizer
            .synthesize("Écoute bien le mot, puis écris-le.", 0.9)
            .unwrap();
        synthesizer.synthesize("un stylo.", 0.85).unwrap();
        synthesizer
            .synthesize("J'écris avec un stylo.", 0.85)
            .unwrap();
        synthesizer.synthesize("un stylo.", 0.5).unwrap();
    }

    assert_eq!(3, synthesizer.process_starts());
}

/// SUB026705 with the real engine: a dictation of 10 words prepared while the instruction is read;
/// each word is then heard well under 200 ms after it is shown (mean and worst case printed).
#[test]
fn piper_prepared_dictation_is_heard_at_once() {
    use educator_core::speech::{Reading, SpeechPreparer};

    let Some(synthesizer) = installed_piper() else {
        return;
    };
    let synthesizer = Arc::new(synthesizer);
    synthesizer.warm_up(0.9);
    synthesizer.warm_up(0.85);
    let preparer = SpeechPreparer::new(synthesizer.clone());
    let words = [
        ("un stylo.", "J'écris avec un stylo."),
        ("un bateau.", "Le bateau flotte sur l'eau."),
        ("très,", "Il est très grand."),
        ("une école.", "Je vais à l'école à pied."),
        ("un gâteau.", "Maman prépare un gâteau."),
        ("une chaise.", "Je m'assois sur la chaise."),
        ("près.", "J'habite près de l'école."),
        ("un oiseau.", "L'oiseau chante dans l'arbre."),
        ("une fleur.", "La fleur sent bon."),
        ("toujours.", "Il fait toujours beau ici."),
    ];
    let series = educator_core::clock::uuid_v7(1_800_000_000_000);
    let readings = words
        .iter()
        .flat_map(|(w, s)| [Reading::new(*w, 0.85), Reading::new(*s, 0.85)])
        .collect();

    // The series starts: the preparation, while the instruction is synthesized then played (≈ 2 s).
    preparer.prepare(series, readings);
    synthesizer
        .synthesize("Écoute bien le mot, puis écris-le.", 0.9)
        .unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));

    let mut delays = Vec::new();
    for (word, sentence) in words {
        let shown = Instant::now();
        preparer.read(series, &Reading::new(word, 0.85)).unwrap();
        delays.push(shown.elapsed().as_millis());
        preparer
            .read(series, &Reading::new(sentence, 0.85))
            .unwrap();
        // The child listens and writes.
        std::thread::sleep(std::time::Duration::from_millis(800));
    }

    let mean = delays.iter().sum::<u128>() / delays.len() as u128;
    let worst = *delays.iter().max().unwrap();
    eprintln!("Dictation of 10 words: word heard after {mean} ms on average, {worst} ms at worst ({delays:?}).");
    assert!(worst < 200, "{delays:?}");
}

/// SUB026704: a Piper that cannot start (Rosetta missing, broken files) makes the voice
/// unavailable after its warm-up; the voice name stays known.
#[test]
fn a_piper_that_cannot_start_is_reported_unavailable() {
    let temp = tempfile::tempdir().unwrap();
    let root = create_layout(&temp.path().join("piper"));
    // An executable that cannot run (empty, not executable).
    let synthesizer =
        PiperSpeechSynthesizer::new(locate(&options(Some(root), temp.path())).unwrap());
    assert!(synthesizer.status().is_available, "nothing tried yet");

    synthesizer.warm_up(0.9);

    let status = synthesizer.status();
    assert!(!status.is_available);
    assert_eq!(Some("fr_FR-siwis-medium".to_owned()), status.voice);
    assert!(status.problem.is_some());
}
