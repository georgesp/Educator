//! Loads and validates the word bank of the spelling skills (SUB001402, SUB001403, SUB026706):
//! `content/spelling/sounds.json` (the sounds and their competing graphemes),
//! `content/spelling/homophones.json` (common homophones: they need their sentence) and
//! `content/words/*.json` (the words). A file with an invalid word is reported and the word skipped:
//! the tests fail on it, the child never meets a broken word.

use crate::{ContentProblem, Loaded};
use educator_core::spelling::{clean, nfc, Article, Grapheme, Sound, Word, PLURAL, SILENT};
use educator_core::Grade;
use serde::Deserialize;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

/// Folder of the words, in the content folder.
pub const WORDS_FOLDER: &str = "words";

/// File of the sounds, in the content folder.
pub const SOUNDS_FILE: &str = "spelling/sounds.json";

/// File of the common homophones, in the content folder.
pub const HOMOPHONES_FILE: &str = "spelling/homophones.json";

/// Category of the nouns: read and shown with their article.
pub const NOUN: &str = "nom";

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
    article: Option<String>,
    pronunciation: Option<String>,
    source: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct HomophonesFile {
    #[serde(rename = "$schema")]
    _schema: Option<String>,
    groups: Vec<Vec<String>>,
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

    let homophones = load_homophones(content_directory, &mut problems);

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

    problems.extend(check_homophones(&words, &homophones));

    Loaded {
        value: WordBank::new(sounds, words),
        problems,
    }
}

/// The words of the homophone groups, lower case (an absent file: no group).
fn load_homophones(
    content_directory: &Path,
    problems: &mut Vec<ContentProblem>,
) -> HashSet<String> {
    let path = content_directory.join(HOMOPHONES_FILE);
    if !path.is_file() {
        return HashSet::new();
    }

    match read::<HomophonesFile>(&path) {
        Ok(file) => {
            let mut words = HashSet::new();
            for group in file.groups {
                if group.len() < 2 || group.iter().any(|w| w.trim().is_empty() || nfc(w) != *w) {
                    problems.push(ContentProblem::new(
                        HOMOPHONES_FILE,
                        format!("group {group:?}: at least two words, not empty, NFC."),
                    ));
                    continue;
                }
                words.extend(group.into_iter().map(|w| w.to_lowercase()));
            }
            words
        }
        Err(e) => {
            problems.push(ContentProblem::new(HOMOPHONES_FILE, e));
            HashSet::new()
        }
    }
}

/// A homophone needs its sentence (« prêt » and « près » sound the same): a word of a homophone
/// group, or two words of the bank read the same way.
fn check_homophones(words: &[Word], homophones: &HashSet<String>) -> Vec<ContentProblem> {
    let said = |w: &Word| clean(w.pronunciation.as_deref().unwrap_or(&w.text)).to_lowercase();
    let mut spoken: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for word in words {
        *spoken.entry(said(word)).or_default() += 1;
    }

    words
        .iter()
        .filter(|w| w.example.is_none())
        .filter(|w| homophones.contains(&w.text.to_lowercase()) || spoken[&said(w)] > 1)
        .map(|w| {
            ContentProblem::new(
                WORDS_FOLDER,
                format!(
                    "'{}': a homophone needs its example sentence (the word alone sounds like another).",
                    w.text
                ),
            )
        })
        .collect()
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

    let article = match (file.article.as_deref(), file.category == NOUN) {
        (Some(article), true) => match article.parse::<Article>() {
            Ok(article) => Some(article),
            Err(()) => {
                errors.push(format!(
                    "article '{article}' is unknown (allowed: un, une; never le nor l')."
                ));
                None
            }
        },
        (None, true) => {
            errors.push("a noun needs its article (un or une).".to_owned());
            None
        }
        (Some(_), false) => {
            errors.push("only a noun has an article.".to_owned());
            None
        }
        (None, false) => None,
    };
    if let Some(pronunciation) = &file.pronunciation {
        if pronunciation.trim().is_empty()
            || pronunciation != pronunciation.trim()
            || pronunciation.contains("  ")
            || pronunciation.contains(['\t', '\n', '\r'])
        {
            errors.push(
                "pronunciation must not be empty nor start, end or repeat spaces.".to_owned(),
            );
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
        article,
        pronunciation: file.pronunciation,
        source: file.source,
    })
}
