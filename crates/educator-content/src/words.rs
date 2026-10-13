//! Loads and validates the word bank of the spelling skills (SUB001402, SUB001403):
//! `content/spelling/sounds.json` (the sounds and their competing graphemes) and
//! `content/words/*.json` (the words). A file with an invalid word is reported and the word skipped:
//! the tests fail on it, the child never meets a broken word.

use crate::{ContentProblem, Loaded};
use educator_core::spelling::{clean, nfc, Grapheme, Sound, Word, PLURAL, SILENT};
use educator_core::Grade;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

/// Folder of the words, in the content folder.
pub const WORDS_FOLDER: &str = "words";

/// File of the sounds, in the content folder.
pub const SOUNDS_FILE: &str = "spelling/sounds.json";

/// Folder of the pre-generated audio of the words (SUB001404), in the content folder.
pub const WORD_AUDIO_FOLDER: &str = "audio/words";

/// Speaking rate of a word to write (dictation): slower than the sentences (SUB001404).
pub const WORD_AUDIO_SPEED: f64 = 0.75;

/// Speaking rate of the example sentence of a word.
pub const EXAMPLE_AUDIO_SPEED: f64 = 0.9;

/// File listing the audio generated and what each was generated from (voice, rate, text).
pub const WORD_AUDIO_MANIFEST: &str = "manifest.json";

pub const CATEGORIES: &[&str] = &[
    "nom",
    "verbe",
    "adjectif",
    "adverbe",
    "mot-outil",
    "groupe-nominal",
];

/// The sounds and the words read from the content.
#[derive(Debug, Clone, Default)]
pub struct WordBank {
    pub sounds: Arc<Vec<Sound>>,
    pub words: Vec<Word>,
    /// Every word, lower case, sorted.
    pub sorted: Arc<Vec<String>>,
}

impl WordBank {
    pub fn new(sounds: Vec<Sound>, words: Vec<Word>) -> Self {
        let mut sorted: Vec<String> = words.iter().map(|w| w.text.to_lowercase()).collect();
        sorted.sort();
        Self {
            sounds: Arc::new(sounds),
            words,
            sorted: Arc::new(sorted),
        }
    }

    pub fn find(&self, text: &str) -> Option<&Word> {
        let text = clean(text).to_lowercase();
        self.words.iter().find(|w| w.text.to_lowercase() == text)
    }
}

/// Name of the pre-generated audio of a word (`bateau-1a2b3c4d.wav`), or of its example sentence
/// (`bateau-1a2b3c4d.example.wav`): ASCII letters of the word, then a hash of the exact text, so
/// that « ou » and « où » get two files.
pub fn word_audio_file(word: &str, example: bool) -> String {
    let word = nfc(word);
    let slug: String = word
        .chars()
        .map(|c| match c {
            'à' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'î' | 'ï' => 'i',
            'ô' | 'ö' => 'o',
            'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            'ÿ' => 'y',
            c if c.is_ascii_alphanumeric() => c.to_ascii_lowercase(),
            _ => '-',
        })
        .collect::<String>()
        .split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    let hash = Sha256::digest(word.as_bytes());
    let short: String = hash[..4].iter().map(|b| format!("{b:02x}")).collect();
    let suffix = if example { ".example.wav" } else { ".wav" };
    format!("{slug}-{short}{suffix}")
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SoundsFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    sounds: Vec<SoundFile>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SoundFile {
    id: String,
    label: String,
    spoken: String,
    graphemes: Vec<String>,
    #[serde(default)]
    confusions: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct WordsFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    words: Vec<WordFile>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct WordFile {
    word: String,
    grades: Vec<String>,
    sounds: Vec<String>,
    graphemes: Vec<GraphemeFile>,
    frequency: u8,
    category: String,
    example: Option<String>,
    /// File name of the pre-generated audio; [`word_audio_file`] when absent.
    #[serde(rename = "audio")]
    _audio: Option<String>,
    source: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GraphemeFile {
    text: String,
    sound: String,
    position: usize,
}

/// Reads the sounds and every `*.json` of the words folder (in file name order).
pub fn load_words(content_directory: &Path) -> Loaded<WordBank> {
    let mut problems = Vec::new();
    let sounds_path = content_directory.join(SOUNDS_FILE);
    let words_directory = content_directory.join(WORDS_FOLDER);
    if !sounds_path.is_file() && !words_directory.is_dir() {
        // No spelling content at all (tests of other folders).
        return Loaded {
            value: WordBank::default(),
            problems,
        };
    }

    let sounds = match read::<SoundsFile>(&sounds_path) {
        Ok(file) => parse_sounds(file, &mut problems),
        Err(e) => {
            problems.push(ContentProblem::new(SOUNDS_FILE, e));
            Vec::new()
        }
    };

    let mut files: Vec<_> = std::fs::read_dir(&words_directory)
        .map(|entries| {
            entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|e| e == "json"))
                .collect()
        })
        .unwrap_or_default();
    files.sort();
    if files.is_empty() {
        problems.push(ContentProblem::new(WORDS_FOLDER, "No word file."));
    }

    let mut words: Vec<Word> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for path in files {
        let name = format!(
            "{WORDS_FOLDER}/{}",
            path.file_name().unwrap_or_default().to_string_lossy()
        );
        let file = match read::<WordsFile>(&path) {
            Ok(file) => file,
            Err(e) => {
                problems.push(ContentProblem::new(name, e));
                continue;
            }
        };

        for entry in file.words {
            let text = entry.word.clone();
            match parse_word(entry, &sounds) {
                Ok(word) => {
                    if seen.insert(word.text.to_lowercase()) {
                        words.push(word);
                    } else {
                        problems.push(ContentProblem::new(
                            name.clone(),
                            format!("word '{text}' is already in the bank."),
                        ));
                    }
                }
                Err(errors) => problems.extend(
                    errors
                        .into_iter()
                        .map(|e| ContentProblem::new(name.clone(), format!("'{text}': {e}"))),
                ),
            }
        }
    }

    Loaded {
        value: WordBank::new(sounds, words),
        problems,
    }
}

fn read<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    let json = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&json).map_err(|e| format!("Invalid JSON: {e}"))
}

fn parse_sounds(file: SoundsFile, problems: &mut Vec<ContentProblem>) -> Vec<Sound> {
    let mut sounds: Vec<Sound> = Vec::new();
    for sound in file.sounds {
        let mut errors = Vec::new();
        if sound.id.trim().is_empty() || sounds.iter().any(|s| s.id == sound.id) {
            errors.push("id must be unique and not empty.".to_owned());
        }
        if sound.label.trim().is_empty() || sound.spoken.trim().is_empty() {
            errors.push("label and spoken are required.".to_owned());
        }
        let unique: HashSet<&String> = sound.graphemes.iter().chain(&sound.confusions).collect();
        if sound.graphemes.is_empty()
            || unique.len() != sound.graphemes.len() + sound.confusions.len()
            || unique.iter().any(|g| g.is_empty() || nfc(g) != **g)
        {
            errors.push(
                "graphemes and confusions: at least one grapheme, no duplicate, no empty text, NFC."
                    .to_owned(),
            );
        }
        let left_out = sound.id == SILENT || sound.id == PLURAL;
        if !left_out && sound.graphemes.len() + sound.confusions.len() < 3 {
            errors.push(
                "graphemes and confusions: at least 3 in all (proposals of the missing letters)."
                    .to_owned(),
            );
        }

        if errors.is_empty() {
            sounds.push(Sound {
                id: sound.id,
                label: sound.label,
                spoken: sound.spoken,
                graphemes: sound.graphemes,
                confusions: sound.confusions,
            });
        } else {
            problems.extend(
                errors.into_iter().map(|e| {
                    ContentProblem::new(SOUNDS_FILE, format!("sound '{}': {e}", sound.id))
                }),
            );
        }
    }

    sounds
}

fn parse_word(file: WordFile, sounds: &[Sound]) -> Result<Word, Vec<String>> {
    let mut errors = Vec::new();
    let text = file.word;
    if text.trim().is_empty() || text != text.trim() || text.contains("  ") {
        errors.push("the word must not be empty nor start, end or repeat spaces.".to_owned());
    }
    if nfc(&text) != text {
        errors.push("the word must be in NFC.".to_owned());
    }
    if text != text.to_lowercase() {
        errors.push("the word must be in lower case.".to_owned());
    }

    let mut grades = Vec::new();
    for grade in &file.grades {
        match grade.parse::<Grade>() {
            Ok(g) if !grades.contains(&g) => grades.push(g),
            _ => errors.push(format!("grade '{grade}' is unknown or repeated.")),
        }
    }
    if grades.is_empty() {
        errors.push("grades must not be empty.".to_owned());
    }

    if !(1..=3).contains(&file.frequency) {
        errors.push("frequency must be 1, 2 or 3.".to_owned());
    }
    if !CATEGORIES.contains(&file.category.as_str()) {
        errors.push(format!(
            "category '{}' is unknown (allowed: {}).",
            file.category,
            CATEGORIES.join(", ")
        ));
    }
    if file.source.trim().is_empty() {
        errors.push("source is required.".to_owned());
    }
    if let Some(example) = &file.example {
        if !example.to_lowercase().contains(&text) {
            errors.push("the example must contain the word.".to_owned());
        }
    }

    let letters: Vec<char> = text.chars().collect();
    let mut graphemes = Vec::new();
    for g in file.graphemes {
        let end = g.position + g.text.chars().count();
        let found: String = letters
            .get(g.position..end.min(letters.len()))
            .map(|s| s.iter().collect())
            .unwrap_or_default();
        if g.text.is_empty() || found != g.text {
            errors.push(format!(
                "grapheme '{}' is not at position {}.",
                g.text, g.position
            ));
        }
        match sounds.iter().find(|s| s.id == g.sound) {
            None => errors.push(format!("sound '{}' is unknown.", g.sound)),
            Some(sound) if !sound.graphemes.contains(&g.text) => errors.push(format!(
                "'{}' is not a grapheme of the sound '{}'.",
                g.text, g.sound
            )),
            Some(_) => {}
        }
        if !file.sounds.contains(&g.sound) {
            errors.push(format!("sounds must list '{}'.", g.sound));
        }
        graphemes.push(Grapheme {
            text: g.text,
            sound: g.sound,
            position: g.position,
        });
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(Word {
        text,
        grades,
        sounds: file.sounds,
        graphemes,
        frequency: file.frequency,
        category: file.category,
        example: file.example,
        source: file.source,
    })
}

#[cfg(test)]
mod tests {
    use super::word_audio_file;

    #[test]
    fn audio_files_are_ascii_and_distinct_for_accents() {
        let a = word_audio_file("a", false);
        let à = word_audio_file("à", false);
        assert!(a.starts_with("a-") && a.ends_with(".wav"));
        assert!(à.starts_with("a-"));
        assert_ne!(a, à);
        assert!(word_audio_file("des chats", false).starts_with("des-chats-"));
        assert!(word_audio_file("garçon", true).ends_with(".example.wav"));
        assert!(word_audio_file("été", false).is_ascii());
    }
}
