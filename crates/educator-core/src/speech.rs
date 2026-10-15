//! Speech: the text read by the voice (SUB000111), the speech engine trait (Piper, SUB006006,
//! implemented in `educator-content` on the desktop) and the preparation of the words of a series
//! ahead of the child (SUB026705).

mod preparation;
mod wav;

pub use preparation::{readings_ahead, Reading, SpeechPreparer, MAX_PREPARED_SERIES, WORDS_AHEAD};
pub use wav::keep_after_first_pause;

use crate::Result;

/// Speaking rates (1 = normal).
pub mod speed {
    pub const MIN: f64 = 0.5;
    pub const NORMAL: f64 = 1.0;
    pub const MAX: f64 = 1.5;

    /// Instructions and feedback: slightly slow for children who are learning to read.
    pub const INSTRUCTIONS: f64 = 0.9;

    /// A word to write and its example sentence, at the same speed so that the word sounds the
    /// same alone and in the sentence (decision of 2026-10-14, SUB026708).
    pub const WORDS: f64 = 0.85;

    /// « Plus lentement »: the word read very slowly (SUB026702).
    pub const SLOW: f64 = 0.5;
}

/// State of the speech engine, shown in the status bar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeechEngineStatus {
    /// Engine and voice found.
    pub is_available: bool,
    /// Voice name (e.g. `fr_FR-siwis-medium`), when found.
    pub voice: Option<String>,
    /// Why the engine is unavailable, in English (logs and diagnostics).
    pub problem: Option<String>,
    /// The engine works but no French voice is installed: the parent can install one (system voice
    /// of the iPad, ADR 0005).
    pub voice_to_install: bool,
}

impl SpeechEngineStatus {
    pub fn available(voice: impl Into<String>) -> Self {
        Self {
            is_available: true,
            voice: Some(voice.into()),
            problem: None,
            voice_to_install: false,
        }
    }

    pub fn unavailable(problem: impl Into<String>) -> Self {
        Self {
            is_available: false,
            voice: None,
            problem: Some(problem.into()),
            voice_to_install: false,
        }
    }

    /// The engine works but has no French voice to read with.
    pub fn voice_missing(problem: impl Into<String>) -> Self {
        Self {
            voice_to_install: true,
            ..Self::unavailable(problem)
        }
    }
}

/// Local, offline text-to-speech.
pub trait SpeechSynthesizer: Send + Sync {
    /// Whether the engine and the voice were found, without starting the engine.
    fn status(&self) -> SpeechEngineStatus;

    /// Synthesizes `text` (French) to a WAV file (16-bit PCM mono) returned as bytes. `speed`: 1 =
    /// normal, from [`speed::MIN`] to [`speed::MAX`]. Fails with
    /// [`crate::Error::SpeechUnavailable`] when the engine or the voice is missing, or the engine failed.
    fn synthesize(&self, text: &str, speed: f64) -> Result<Vec<u8>>;

    /// Starts the engine ahead of the first request (loads the voice model) at a speed the
    /// application uses all the time, and keeps it ready (Piper: one process per speed). Never fails.
    fn warm_up(&self, speed: f64);
}

/// Used when the engine is missing (no Piper, or a platform without it): the application works,
/// without sound.
#[derive(Debug, Clone)]
pub struct UnavailableSpeechSynthesizer {
    problem: String,
}

impl UnavailableSpeechSynthesizer {
    pub fn new(problem: impl Into<String>) -> Self {
        Self {
            problem: problem.into(),
        }
    }
}

impl SpeechSynthesizer for UnavailableSpeechSynthesizer {
    fn status(&self) -> SpeechEngineStatus {
        SpeechEngineStatus::unavailable(self.problem.clone())
    }

    fn synthesize(&self, _text: &str, _speed: f64) -> Result<Vec<u8>> {
        Err(crate::Error::SpeechUnavailable(self.problem.clone()))
    }

    fn warm_up(&self, _speed: f64) {}
}

const UNITS: [&str; 20] = [
    "zéro", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf", "dix", "onze",
    "douze", "treize", "quatorze", "quinze", "seize", "dix-sept", "dix-huit", "dix-neuf",
];

const TENS: [&str; 7] = [
    "",
    "",
    "vingt",
    "trente",
    "quarante",
    "cinquante",
    "soixante",
];

const MAX_NUMBER: u32 = 999_999_999;

fn is_sign(c: char) -> bool {
    matches!(c, '+' | '-' | '−' | '–' | '=' | '×' | '<' | '>')
}

/// Prepares a text for the speech engine: numbers become French words and the calculation symbols
/// become words (`+` « plus », `−` « moins », `=` « égale », `×` « fois », `<` « est plus petit
/// que », `>` « est plus grand que », a `?` standing for a number « combien »), so that the voice
/// reads « sept plus cinq égale douze ».
pub fn normalize(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();

    // A `?` next to a calculation sign stands for a number: `7 + ? = 12`.
    let mut step = String::with_capacity(text.len() + 16);
    for (i, &c) in chars.iter().enumerate() {
        if c == '?'
            && (neighbour(&chars, i, Direction::Before).is_some_and(is_sign)
                || neighbour(&chars, i, Direction::After).is_some_and(is_sign))
        {
            step.push_str(" combien ");
        } else {
            step.push(c);
        }
    }

    let step = step
        .replace('+', " plus ")
        .replace(['−', '–'], " moins ")
        .replace('=', " égale ")
        .replace('×', " fois ")
        .replace('<', " est plus petit que ")
        .replace('>', " est plus grand que ")
        .replace('→', " : ");

    // A hyphen between two numbers is a minus sign; hyphens inside words are kept.
    let chars: Vec<char> = step.chars().collect();
    let mut step = String::with_capacity(step.len() + 16);
    for (i, &c) in chars.iter().enumerate() {
        let digit = |d: Option<char>| d.is_some_and(|d| d.is_ascii_digit());
        if c == '-'
            && digit(neighbour(&chars, i, Direction::Before))
            && digit(neighbour(&chars, i, Direction::After))
        {
            step.push_str(" moins ");
        } else {
            step.push(c);
        }
    }

    // Numbers (runs of at most 9 digits) in words.
    let mut words = String::with_capacity(step.len() * 2);
    let mut digits = String::new();
    let flush = |digits: &mut String, words: &mut String| {
        if !digits.is_empty() {
            let number: u32 = digits.parse().unwrap_or(0);
            words.push_str(&number_to_words(number));
            digits.clear();
        }
    };
    // A space between groups of three digits belongs to the number: « 1 000 », « 12 500 ».
    let all: Vec<char> = step.chars().collect();
    let thousands_space = |i: usize, digits: &str| {
        matches!(all[i], ' ' | '\u{a0}' | '\u{202f}')
            && !digits.is_empty()
            && digits.len() <= 6
            && all
                .get(i + 1..i + 4)
                .is_some_and(|g| g.iter().all(char::is_ascii_digit))
            && !all.get(i + 4).is_some_and(char::is_ascii_digit)
    };
    for (i, &c) in all.iter().enumerate() {
        if thousands_space(i, &digits) {
            continue;
        }
        if c.is_ascii_digit() {
            if digits.len() == 9 {
                flush(&mut digits, &mut words);
            }

            digits.push(c);
        } else {
            flush(&mut digits, &mut words);
            words.push(c);
        }
    }

    flush(&mut digits, &mut words);

    // Collapse the spaces, then no space before a comma or a period.
    let collapsed = words.split_whitespace().collect::<Vec<_>>().join(" ");
    let chars: Vec<char> = collapsed.chars().collect();
    chars
        .iter()
        .enumerate()
        .filter(|(i, c)| !(**c == ' ' && matches!(chars.get(i + 1), Some(',' | '.'))))
        .map(|(_, c)| *c)
        .collect()
}

#[derive(Clone, Copy)]
enum Direction {
    Before,
    After,
}

/// The first non-whitespace character before or after position `i`.
fn neighbour(chars: &[char], i: usize, direction: Direction) -> Option<char> {
    match direction {
        Direction::Before => chars[..i]
            .iter()
            .rev()
            .find(|c| !c.is_whitespace())
            .copied(),
        Direction::After => chars[i + 1..].iter().find(|c| !c.is_whitespace()).copied(),
    }
}

/// French words of a number from 0 to 999 999 999 (« soixante et onze », « quatre-vingts »).
pub fn number_to_words(number: u32) -> String {
    assert!(number <= MAX_NUMBER, "number above 999 999 999");
    if number == 0 {
        return UNITS[0].to_owned();
    }

    let millions = number / 1_000_000;
    let thousands = number / 1000 % 1000;
    let rest = number % 1000;
    let mut parts = Vec::new();
    if millions > 0 {
        parts.push(
            below_thousand(millions, true)
                + if millions > 1 {
                    " millions"
                } else {
                    " million"
                },
        );
    }

    if thousands > 0 {
        // « mille » never takes an s, and « cent » / « vingt » stay singular before it.
        parts.push(if thousands == 1 {
            "mille".to_owned()
        } else {
            below_thousand(thousands, false) + " mille"
        });
    }

    if rest > 0 {
        parts.push(below_thousand(rest, true));
    }

    parts.join(" ")
}

fn below_thousand(number: u32, is_final: bool) -> String {
    let hundreds = number / 100;
    let rest = number % 100;
    let mut words = String::new();
    if hundreds > 0 {
        if hundreds == 1 {
            words.push_str("cent");
        } else {
            words.push_str(UNITS[hundreds as usize]);
            words.push_str(" cent");
            if rest == 0 && is_final {
                words.push('s');
            }
        }
    }

    if rest > 0 {
        if !words.is_empty() {
            words.push(' ');
        }

        words.push_str(&below_hundred(rest, is_final));
    }

    words
}

fn below_hundred(number: u32, is_final: bool) -> String {
    if number < 20 {
        return UNITS[number as usize].to_owned();
    }

    let ten = (number / 10) as usize;
    let unit = (number % 10) as usize;
    match ten {
        7 if unit == 1 => "soixante et onze".to_owned(),
        7 => format!("soixante-{}", UNITS[10 + unit]),
        8 if unit == 0 => if is_final {
            "quatre-vingts"
        } else {
            "quatre-vingt"
        }
        .to_owned(),
        8 => format!("quatre-vingt-{}", UNITS[unit]),
        9 => format!("quatre-vingt-{}", UNITS[10 + unit]),
        _ => match unit {
            0 => TENS[ten].to_owned(),
            1 => format!("{} et un", TENS[ten]),
            _ => format!("{}-{}", TENS[ten], UNITS[unit]),
        },
    }
}
