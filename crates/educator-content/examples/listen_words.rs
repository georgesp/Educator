//! Listening check of the words read by the voice (SUB026706, ADR 0004): writes, in a temporary
//! folder (never embedded in the application), the WAV of a sample of words or of the words of a
//! skill, each alone (« un stylo. ») then in its sentence, with the list to listen to. The Piper voice
//! is found as by the application (`EDUCATOR_PIPER_DIR`, `EDUCATOR_PIPER_VOICE`, or `vendor/piper`
//! filled by `scripts/fetch-piper`).
//!
//! Piper draws at random at each reading: a failed reading can be done again with `--again stylo`,
//! which replaces it in the application's voice cache (`--cache`, given by the scripts).
//!
//! `scripts/listen-words.sh` / `scripts/listen-words.ps1`, or
//! `cargo run -p educator-content --example listen_words --release -- [options]`:
//!
//! - `--sample 50`: 50 words drawn at random (the default);
//! - `--skill ce1-son-o-au-eau`: the words of a skill;
//! - `--again stylo` (repeatable): reads these words again, replacing their cached reading;
//! - `--slow`: also writes each word read very slowly (« Plus lentement », SUB026702);
//! - `--cache <folder>`: the application's voice cache (`<data folder>/speech-cache`);
//! - `--out <folder>`: where to write the files (a new temporary folder by default).

use educator_content::speech::{locate, PiperLocatorOptions, PiperSpeechSynthesizer, SpeechCache};
use educator_content::EducatorContent;
use educator_core::skills::SkillCatalog;
use educator_core::speech::{Reading, SpeechSynthesizer};
use educator_core::spelling::{SpokenWord, Word};
use rand::seq::IndexedRandom;
use rand::SeedableRng;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

struct Options {
    sample: usize,
    skill: Option<String>,
    again: Vec<String>,
    slow: bool,
    cache: Option<PathBuf>,
    out: Option<PathBuf>,
}

fn options() -> Options {
    let mut options = Options {
        sample: 50,
        skill: None,
        again: Vec::new(),
        slow: false,
        cache: None,
        out: None,
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = || {
            args.next().unwrap_or_else(|| {
                eprintln!("✗ {arg} attend une valeur.");
                std::process::exit(2);
            })
        };
        match arg.as_str() {
            "--sample" => {
                options.sample = value().parse().unwrap_or_else(|_| {
                    eprintln!("✗ --sample attend un nombre.");
                    std::process::exit(2);
                })
            }
            "--skill" => options.skill = Some(value()),
            "--again" => options.again.push(value()),
            "--slow" => options.slow = true,
            "--cache" => options.cache = Some(PathBuf::from(value())),
            "--out" => options.out = Some(PathBuf::from(value())),
            other => {
                eprintln!("✗ Option inconnue : {other} (voir l'en-tête de listen_words.rs).");
                std::process::exit(2);
            }
        }
    }
    options
}

fn main() {
    let options = options();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let content = EducatorContent::load(&root.join("content"));
    if !content.problems.is_empty() {
        for problem in &content.problems {
            eprintln!("✗ {problem}");
        }
        std::process::exit(1);
    }

    let installation = match locate(&PiperLocatorOptions::from_environment(
        Vec::new(),
        root.clone(),
    )) {
        Ok(installation) => installation,
        Err(problem) => {
            eprintln!("✗ {problem}");
            std::process::exit(1);
        }
    };
    let voice = installation.voice_name.clone();
    let piper: Arc<dyn SpeechSynthesizer> = Arc::new(PiperSpeechSynthesizer::new(installation));
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let cache = SpeechCache::new(
        Arc::clone(&piper),
        options
            .cache
            .clone()
            .unwrap_or_else(|| std::env::temp_dir().join(format!("educator-cache-{stamp}"))),
    );

    let words: Vec<&Word> = if !options.again.is_empty() {
        options
            .again
            .iter()
            .map(|text| {
                content.words.find(text).unwrap_or_else(|| {
                    eprintln!("✗ « {text} » n'est pas dans la banque de mots.");
                    std::process::exit(1);
                })
            })
            .collect()
    } else if let Some(id) = &options.skill {
        let skill = content.skills.find(id).unwrap_or_else(|| {
            eprintln!("✗ Compétence inconnue : {id}.");
            std::process::exit(1);
        });
        let Some(set) = &skill.params.words else {
            eprintln!("✗ {id} n'est pas une compétence d'orthographe.");
            std::process::exit(1);
        };
        set.words.iter().map(|w| &w.word).collect()
    } else {
        let mut random = rand::rngs::StdRng::seed_from_u64(stamp);
        let mut drawn: Vec<&Word> = content
            .words
            .words
            .choose_multiple(&mut random, options.sample)
            .collect();
        drawn.sort_by(|a, b| a.text.cmp(&b.text));
        drawn
    };

    let out = options
        .out
        .clone()
        .unwrap_or_else(|| std::env::temp_dir().join(format!("educator-ecoute-{stamp}")));
    std::fs::create_dir_all(&out).expect("listening folder");
    let mut list = vec![format!(
        "Écoute de contrôle — voix {voice}, {} mot(s). Chaque mot seul, puis dans sa phrase.\n",
        words.len()
    )];
    let started = Instant::now();
    let mut failed = 0usize;
    for (n, word) in words.iter().enumerate() {
        let spoken = SpokenWord::of(word, None);
        let mut readings: Vec<(&str, Reading)> =
            vec![("", spoken.speech(false, false).expect("word"))];
        if let Some(sentence) = spoken.speech(true, false) {
            readings.push(("phrase", sentence));
        }
        if options.slow {
            readings.push(("lent", spoken.speech(false, true).expect("word")));
        }

        let mut files = Vec::new();
        for (suffix, reading) in readings {
            if !options.again.is_empty() {
                cache.forget(&reading.text, reading.speed);
            }
            let name = file_name(n + 1, &word.text, suffix);
            match cache.synthesize(&reading.text, reading.speed) {
                Ok(wav) => {
                    std::fs::write(out.join(&name), wav).expect("write the audio");
                    files.push(name);
                }
                Err(e) => {
                    eprintln!("✗ « {} » : {e}", reading.text);
                    failed += 1;
                }
            }
        }
        list.push(format!(
            "{:>3}. {:<20} lu « {} »  →  {}",
            n + 1,
            word.text,
            spoken.reading,
            files.join(", ")
        ));
    }

    let list_path = out.join("ecoute.txt");
    std::fs::write(&list_path, list.join("\n") + "\n").expect("listening list");
    println!(
        "✓ {} mot(s) en {:.1} s, {failed} lecture(s) en échec. Voix {voice}.",
        words.len(),
        started.elapsed().as_secs_f64()
    );
    println!("  Fichiers à écouter : {}", out.display());
    println!("  Liste : {}", list_path.display());
    if options.cache.is_some() && !options.again.is_empty() {
        println!("  Lecture(s) remplacée(s) dans le cache de la voix de l'application.");
    }
    if failed > 0 {
        std::process::exit(1);
    }
}

/// `001-stylo.wav`, `001-stylo-phrase.wav`: the number keeps the order of the list, ASCII letters
/// keep the name readable everywhere.
fn file_name(number: usize, word: &str, suffix: &str) -> String {
    let slug: String = word
        .chars()
        .map(|c| match c {
            'à' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'î' | 'ï' => 'i',
            'ô' | 'ö' => 'o',
            'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            'œ' => 'o',
            c if c.is_ascii_alphanumeric() => c.to_ascii_lowercase(),
            _ => '-',
        })
        .collect();
    if suffix.is_empty() {
        format!("{number:03}-{slug}.wav")
    } else {
        format!("{number:03}-{slug}-{suffix}.wav")
    }
}
