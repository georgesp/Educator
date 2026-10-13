//! Pre-generates the audio of the words of the spelling skills (SUB001404): one WAV per word, read
//! slowly, and one per example sentence, with the Piper voice found as by the application
//! (`EDUCATOR_PIPER_DIR`, `EDUCATOR_PIPER_VOICE`, or `vendor/piper` filled by `scripts/fetch-piper`).
//!
//! The files go to `content/audio/words/` (not committed, embedded with the content when the
//! application is built). Only the words added or changed are generated again: `manifest.json`
//! keeps, for each file, a hash of the voice, the rate and the text. Files no longer used are listed,
//! never deleted.
//!
//! `cargo run -p educator-content --example generate_word_audio --release` (from the repository root),
//! or `scripts/generate-word-audio.sh`. Options: `--sample 50` writes `content/audio/words/ecoute.txt`,
//! the list of 50 words drawn at random for the listening check.

use educator_content::speech::{locate, PiperLocatorOptions, PiperSpeechSynthesizer};
use educator_content::{
    load_words, word_audio_file, EXAMPLE_AUDIO_SPEED, WORD_AUDIO_FOLDER, WORD_AUDIO_MANIFEST,
    WORD_AUDIO_SPEED,
};
use educator_core::speech::{normalize, SpeechSynthesizer};
use rand::seq::IndexedRandom;
use rand::SeedableRng;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let content = root.join("content");
    let bank = load_words(&content);
    if !bank.problems.is_empty() {
        for problem in &bank.problems {
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
    let piper = PiperSpeechSynthesizer::new(installation);

    let folder = content.join(WORD_AUDIO_FOLDER);
    std::fs::create_dir_all(&folder).expect("audio folder");
    let manifest_path = folder.join(WORD_AUDIO_MANIFEST);
    let mut manifest: BTreeMap<String, String> = std::fs::read_to_string(&manifest_path)
        .ok()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default();

    // Every file wanted: (file name, text read, rate).
    let mut wanted: Vec<(String, String, f64)> = Vec::new();
    for word in &bank.value.words {
        wanted.push((
            word_audio_file(&word.text, false),
            word.text.clone(),
            WORD_AUDIO_SPEED,
        ));
        if let Some(example) = &word.example {
            wanted.push((
                word_audio_file(&word.text, true),
                example.clone(),
                EXAMPLE_AUDIO_SPEED,
            ));
        }
    }

    let (mut generated, mut kept, mut failed) = (0usize, 0usize, 0usize);
    for (index, (file, text, speed)) in wanted.iter().enumerate() {
        let hash = hash(&voice, *speed, text);
        let path = folder.join(file);
        if path.is_file() && manifest.get(file) == Some(&hash) {
            kept += 1;
            continue;
        }

        match piper.synthesize(&normalize(text), *speed) {
            Ok(wav) => {
                std::fs::write(&path, wav).expect("write the audio");
                manifest.insert(file.clone(), hash);
                generated += 1;
                if generated % 50 == 0 {
                    println!("… {}/{}", index + 1, wanted.len());
                }
            }
            Err(e) => {
                eprintln!("✗ « {text} » : {e}");
                failed += 1;
            }
        }
    }

    let wanted_names: Vec<&str> = wanted.iter().map(|(f, _, _)| f.as_str()).collect();
    manifest.retain(|file, _| wanted_names.contains(&file.as_str()));
    std::fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest).expect("manifest") + "\n",
    )
    .expect("write the manifest");

    let mut unused = Vec::new();
    let mut total_bytes = 0u64;
    for entry in std::fs::read_dir(&folder).expect("audio folder").flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !name.ends_with(".wav") {
            continue;
        }
        if wanted_names.contains(&name.as_str()) {
            total_bytes += entry.metadata().map(|m| m.len()).unwrap_or(0);
        } else {
            unused.push(name);
        }
    }

    println!(
        "✓ {} mots, {} fichiers : {generated} générés, {kept} déjà à jour, {failed} en échec. Voix {voice}.",
        bank.value.words.len(),
        wanted.len()
    );
    println!(
        "  Taille totale : {:.1} Mo ({})",
        total_bytes as f64 / 1_000_000.0,
        folder.display()
    );
    if !unused.is_empty() {
        println!(
            "  {} fichier(s) ne servent plus (à supprimer à la main) : {}",
            unused.len(),
            unused.join(", ")
        );
    }

    if let Some(count) = sample_option() {
        let mut random = rand::rngs::StdRng::seed_from_u64(2026);
        let mut words: Vec<&str> = bank
            .value
            .words
            .iter()
            .map(|w| w.text.as_str())
            .collect::<Vec<_>>()
            .choose_multiple(&mut random, count)
            .copied()
            .collect();
        words.sort_unstable();
        let lines: Vec<String> = words
            .iter()
            .map(|w| format!("{w}\t{}", word_audio_file(w, false)))
            .collect();
        let list = folder.join("ecoute.txt");
        std::fs::write(&list, lines.join("\n") + "\n").expect("listening list");
        println!("  Échantillon d'écoute ({count} mots) : {}", list.display());
    }

    if failed > 0 {
        std::process::exit(1);
    }
}

fn hash(voice: &str, speed: f64, text: &str) -> String {
    let digest = Sha256::digest(format!("{voice}|{speed:.2}|{text}").as_bytes());
    digest[..8].iter().map(|b| format!("{b:02x}")).collect()
}

fn sample_option() -> Option<usize> {
    let args: Vec<String> = std::env::args().collect();
    args.iter()
        .position(|a| a == "--sample")
        .and_then(|i| args.get(i + 1))
        .and_then(|n| n.parse().ok())
}
