#![cfg(not(any(target_os = "ios", target_os = "android")))]

use educator_content::speech::{
    build_input_line, locate, to_length_scale, PiperLocatorOptions, PiperPlatform,
    PiperSpeechSynthesizer, SpeechCache, MAX_FILES,
};
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

#[test]
fn prune_keeps_the_cache_bounded() {
    let temp = tempfile::tempdir().unwrap();
    let cache = SpeechCache::new(
        Arc::new(CountingEngine::default()),
        temp.path().to_path_buf(),
    );
    for i in 0..MAX_FILES + 10 {
        std::fs::write(temp.path().join(format!("{i:05}.wav")), [1]).unwrap();
    }

    cache.prune();

    let count = std::fs::read_dir(temp.path()).unwrap().count();
    assert_eq!(MAX_FILES * 9 / 10, count);
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

/// The process protocol with a fake engine (a shell script answering like `piper --json-input`):
/// one JSON line in, one WAV file written in the output folder, its path printed.
#[cfg(unix)]
#[test]
fn piper_protocol_with_a_fake_engine() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempfile::tempdir().unwrap();
    let root = create_layout(&temp.path().join("piper"));
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
  n=$((n+1))
  printf 'RIFF....WAVE %s %s' "$SCALE" "$line" > "$OUT/$n.wav"
  echo "$OUT/$n.wav"
done
"#,
    )
    .unwrap();
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
    let mut options = options(Some(root), temp.path());
    options.platform = platform;
    let synthesizer = PiperSpeechSynthesizer::new(locate(&options).unwrap());

    let first = String::from_utf8(synthesizer.synthesize("sept plus cinq", 1.0).unwrap()).unwrap();
    let second = String::from_utf8(synthesizer.synthesize("élève", 1.0).unwrap()).unwrap();
    let slower = String::from_utf8(synthesizer.synthesize("lent", 0.5).unwrap()).unwrap();

    assert_eq!(r#"RIFF....WAVE 1 {"text":"sept plus cinq"}"#, first);
    assert!(
        second.starts_with("RIFF....WAVE 1 {") && second.ends_with("ve\"}"),
        "{second}"
    );
    assert_eq!(r#"RIFF....WAVE 2 {"text":"lent"}"#, slower);
}
