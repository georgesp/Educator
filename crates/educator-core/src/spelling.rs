//! Spelling (lot 2): the words of the bank (SUB001402), the checking of a written word letter by
//! letter (SUB001405) and the three exercises: dictation (SUB001406), missing letters (SUB001407)
//! and « Choisis la bonne écriture » (SUB001408).
//!
//! Rules of the checking: text compared after Unicode composition (NFC, French letters), spaces at
//! both ends ignored and inner spaces collapsed; **no tolerance on accents** (« éte » ≠ « été »);
//! capital letters tolerated (« Paris » = « paris »), a rule each skill may change later; the
//! ligature « œ » is a letter of its own (« soeur » ≠ « sœur », decision of 2026-10-14: the on-screen
//! keyboard has it).

mod exercises;
mod nfc;

pub use exercises::{draw, wrong_spellings, CHOICES};
pub use nfc::nfc;

use crate::Grade;
use std::sync::Arc;

/// A letter of a corrected word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LetterKind {
    /// The right letter at the right place.
    Right,
    /// A letter instead of another one (`given` written, `expected` wanted).
    Wrong,
    /// A letter forgotten (`expected`).
    Missing,
    /// A letter too many (`given`).
    Extra,
}

/// One position of the correction: what the child wrote and what was expected there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LetterMark {
    pub kind: LetterKind,
    pub given: Option<char>,
    pub expected: Option<char>,
}

/// The child's word aligned on the expected word, letter by letter (alignment of the Levenshtein
/// distance): the front highlights each letter.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpellingCorrection {
    pub correct: bool,
    pub letters: Vec<LetterMark>,
}

/// Whether two letters are the same for the checking: same letter, capital letters tolerated,
/// accents never.
fn same_letter(a: char, b: char) -> bool {
    a == b || a.to_lowercase().eq(b.to_lowercase())
}

/// Text as compared: NFC, trimmed, inner spaces collapsed to one, typographic apostrophes as `'`.
pub fn clean(text: &str) -> String {
    nfc(text)
        .replace(['\u{2019}', '\u{02BC}'], "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Whether `given` is the right spelling of `expected` (no tolerance on accents nor on « œ »,
/// capitals tolerated).
pub fn is_right_spelling(expected: &str, given: &str) -> bool {
    let (expected, given) = (clean(expected), clean(given));
    expected.chars().count() == given.chars().count()
        && expected
            .chars()
            .zip(given.chars())
            .all(|(e, g)| same_letter(e, g))
}

/// Compares the child's word with the expected one, letter by letter.
pub fn correct(expected: &str, given: &str) -> SpellingCorrection {
    let expected: Vec<char> = clean(expected).chars().collect();
    let given: Vec<char> = clean(given).chars().collect();
    let (n, m) = (expected.len(), given.len());

    // distance[i][j]: edits between the first i expected letters and the first j given ones.
    let mut distance = vec![vec![0usize; m + 1]; n + 1];
    for (i, row) in distance.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in distance[0].iter_mut().enumerate() {
        *cell = j;
    }
    for i in 1..=n {
        for j in 1..=m {
            let substitution = usize::from(!same_letter(expected[i - 1], given[j - 1]));
            distance[i][j] = (distance[i - 1][j - 1] + substitution)
                .min(distance[i - 1][j] + 1)
                .min(distance[i][j - 1] + 1);
        }
    }

    // Back from the end; on a tie, a pairing (right or wrong letter) before a missing or extra one.
    let mut letters = Vec::with_capacity(n.max(m));
    let (mut i, mut j) = (n, m);
    while i > 0 || j > 0 {
        if i > 0 && j > 0 {
            let same = same_letter(expected[i - 1], given[j - 1]);
            if distance[i][j] == distance[i - 1][j - 1] + usize::from(!same) {
                letters.push(LetterMark {
                    kind: if same {
                        LetterKind::Right
                    } else {
                        LetterKind::Wrong
                    },
                    given: Some(given[j - 1]),
                    expected: Some(expected[i - 1]),
                });
                i -= 1;
                j -= 1;
                continue;
            }
        }

        if i > 0 && distance[i][j] == distance[i - 1][j] + 1 {
            letters.push(LetterMark {
                kind: LetterKind::Missing,
                given: None,
                expected: Some(expected[i - 1]),
            });
            i -= 1;
        } else {
            letters.push(LetterMark {
                kind: LetterKind::Extra,
                given: Some(given[j - 1]),
                expected: None,
            });
            j -= 1;
        }
    }

    letters.reverse();
    SpellingCorrection {
        correct: letters.iter().all(|l| l.kind == LetterKind::Right),
        letters,
    }
}

/// A sound and the graphemes that write it (`content/spelling/sounds.json`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sound {
    /// `o`, `an`, `muet`, `pluriel`…
    pub id: String,
    /// Written label: « [o] ».
    pub label: String,
    /// Read label: « le son o ».
    pub spoken: String,
    /// Competing graphemes of the sound: `o`, `au`, `eau`.
    pub graphemes: Vec<String>,
    /// Graphemes often mistaken for them, proposed when the sound has fewer than 3 graphemes.
    pub confusions: Vec<String>,
}

impl Sound {
    /// A silent letter or the mark of the plural: leaving the letter out is a classic mistake.
    pub fn may_be_left_out(&self) -> bool {
        self.id == SILENT || self.id == PLURAL
    }
}

pub const SILENT: &str = "muet";
pub const PLURAL: &str = "pluriel";

/// A grapheme worked in a word: its text, its sound, where it starts (in letters, from 0).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grapheme {
    pub text: String,
    pub sound: String,
    pub position: usize,
}

/// The article a noun is read with (« un stylo. », SUB026706): « un » or « une », never « le » nor
/// « l' », which would merge with a vowel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Article {
    Un,
    Une,
}

impl Article {
    pub fn as_str(self) -> &'static str {
        match self {
            Article::Un => "un",
            Article::Une => "une",
        }
    }
}

impl std::str::FromStr for Article {
    type Err = ();

    fn from_str(text: &str) -> std::result::Result<Self, ()> {
        match text {
            "un" => Ok(Article::Un),
            "une" => Ok(Article::Une),
            _ => Err(()),
        }
    }
}

/// The text read for a word alone (decision of 2026-10-14, SUB026706): a noun with its article
/// (« un stylo. » — alone at the start of a reading, a noun often loses its first consonant), any
/// other word followed by a comma (« beau, »: decision of 2026-10-15 — followed by a period, a short
/// word is often read wrong, « beau. » sounds « beu »). `pronunciation` replaces the word when the voice
/// reads it wrong (« fils » → « fisse »).
pub fn word_reading(text: &str, article: Option<Article>, pronunciation: Option<&str>) -> String {
    let said = pronunciation.unwrap_or(text);
    match article {
        Some(article) => format!("{} {said}.", article.as_str()),
        None => format!("{said},"),
    }
}

/// A word of the bank.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Word {
    pub text: String,
    pub grades: Vec<Grade>,
    pub sounds: Vec<String>,
    pub graphemes: Vec<Grapheme>,
    /// 1 = very frequent, 3 = less frequent.
    pub frequency: u8,
    pub category: String,
    /// A short sentence with the word, read after it (homophones: « vert », « verre », « ver »).
    pub example: Option<String>,
    /// Nouns: the article they are read and shown with (« un [______] »).
    pub article: Option<Article>,
    /// What the voice reads instead of the word, when it reads it wrong.
    pub pronunciation: Option<String>,
    pub source: String,
}

impl Word {
    /// The text read for the word alone: [`word_reading`].
    pub fn reading(&self) -> String {
        word_reading(&self.text, self.article, self.pronunciation.as_deref())
    }

    /// The letters before and after a grapheme of the word.
    pub fn around(&self, grapheme: &Grapheme) -> (String, String) {
        let letters: Vec<char> = self.text.chars().collect();
        let start = grapheme.position.min(letters.len());
        let end = (start + grapheme.text.chars().count()).min(letters.len());
        (
            letters[..start].iter().collect(),
            letters[end..].iter().collect(),
        )
    }
}

/// A word of a skill and the graphemes the skill works on in it (indexes in [`Word::graphemes`];
/// empty for the skills of whole words, like the « mots outils »).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SkillWord {
    pub word: Word,
    pub worked: Vec<usize>,
}

/// The words of a spelling skill, resolved when the content is loaded.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct WordSet {
    pub words: Vec<SkillWord>,
    /// Every sound of the content.
    pub sounds: Arc<Vec<Sound>>,
    /// Every word of the bank, sorted: a wrong spelling proposed is never a real word.
    pub bank: Arc<Vec<String>>,
}

impl WordSet {
    pub fn sound(&self, id: &str) -> Option<&Sound> {
        self.sounds.iter().find(|s| s.id == id)
    }

    pub fn is_bank_word(&self, text: &str) -> bool {
        let text = clean(text).to_lowercase();
        self.bank.binary_search(&text).is_ok()
    }
}

/// What the skill takes from the bank: words of these grades, with a grapheme of these sounds and
/// texts, of these categories (each criterion ignored when empty).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct WordFilter {
    pub grades: Vec<Grade>,
    pub sounds: Vec<String>,
    pub graphemes: Vec<String>,
    pub categories: Vec<String>,
}

impl WordFilter {
    /// The word as worked by the skill, or `None` when it does not belong to it.
    pub fn select(&self, word: &Word) -> Option<SkillWord> {
        if !self.grades.is_empty() && !word.grades.iter().any(|g| self.grades.contains(g)) {
            return None;
        }

        if !self.categories.is_empty() && !self.categories.contains(&word.category) {
            return None;
        }

        if self.sounds.is_empty() && self.graphemes.is_empty() {
            return Some(SkillWord {
                word: word.clone(),
                worked: Vec::new(),
            });
        }

        let worked: Vec<usize> = word
            .graphemes
            .iter()
            .enumerate()
            .filter(|(_, g)| {
                (self.sounds.is_empty() || self.sounds.contains(&g.sound))
                    && (self.graphemes.is_empty() || self.graphemes.contains(&g.text))
            })
            .map(|(i, _)| i)
            .collect();
        (!worked.is_empty()).then(|| SkillWord {
            word: word.clone(),
            worked,
        })
    }
}

/// The word read in an exercise (voice, ADR 0004) and, for the missing letters, the letters around
/// the blank.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpokenWord {
    pub word: String,
    pub example: Option<String>,
    /// Missing letters: the letters before and after the blank.
    pub blank: Option<(String, String)>,
    /// Nouns: the article shown before the answer (the child writes the noun only).
    pub article: Option<Article>,
    /// The text read for the word alone (« un stylo. »).
    pub reading: String,
}

impl SpokenWord {
    /// A word of the bank, read with its article or its pronunciation.
    pub fn of(word: &Word, blank: Option<(String, String)>) -> Self {
        Self {
            word: word.text.clone(),
            example: word.example.clone(),
            blank,
            article: word.article,
            reading: word.reading(),
        }
    }

    /// A word that is not in the bank (a lesson quiz): read followed by a comma.
    pub fn plain(word: impl Into<String>) -> Self {
        let word = word.into();
        Self {
            reading: word_reading(&word, None, None),
            word,
            example: None,
            blank: None,
            article: None,
        }
    }

    /// What the voice reads: the word alone or its sentence (`example`), at the speed of the words
    /// or slowly (SUB026702). `None` when there is no sentence.
    pub fn speech(&self, example: bool, slow: bool) -> Option<crate::speech::Reading> {
        let text = if example {
            self.example.as_deref()?
        } else {
            self.reading.as_str()
        };
        let speed = if slow {
            crate::speech::speed::SLOW
        } else {
            crate::speech::speed::WORDS
        };
        Some(crate::speech::Reading::new(text, speed))
    }

    /// Missing letters: how many letters the blank hides; the child sees as many dashes
    /// (« bat___ », SUB001407).
    pub fn missing_count(&self) -> Option<usize> {
        self.blank.as_ref().map(|(before, after)| {
            self.word
                .chars()
                .count()
                .saturating_sub(before.chars().count() + after.chars().count())
        })
    }

    /// The whole word written by the child: the letters typed or picked put in the blank.
    pub fn completed(&self, given: &str) -> String {
        match &self.blank {
            Some((before, after)) => format!("{before}{}{after}", clean(given)),
            None => given.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(correction: &SpellingCorrection) -> String {
        correction
            .letters
            .iter()
            .map(|l| match l.kind {
                LetterKind::Right => '=',
                LetterKind::Wrong => 'x',
                LetterKind::Missing => '-',
                LetterKind::Extra => '+',
            })
            .collect()
    }

    #[test]
    fn a_right_word_is_right_whatever_the_capitals_and_spaces() {
        assert!(is_right_spelling("été", "été"));
        assert!(is_right_spelling("paris", "Paris"));
        assert!(is_right_spelling("des chats", "  des   chats "));
        assert!(correct("bateau", "BATEAU").correct);
    }

    #[test]
    fn accents_are_never_tolerated() {
        assert!(!is_right_spelling("été", "ete"));
        assert!(!is_right_spelling("été", "éte"));
        let c = correct("été", "éte");
        assert!(!c.correct);
        assert_eq!("==x", kinds(&c));
        assert_eq!(Some('e'), c.letters[2].given);
        assert_eq!(Some('é'), c.letters[2].expected);
    }

    #[test]
    fn a_wrong_accent_is_a_wrong_letter() {
        let c = correct("mère", "mére");
        assert_eq!("=x==", kinds(&c));
        assert_eq!(
            (Some('é'), Some('è')),
            (c.letters[1].given, c.letters[1].expected)
        );
    }

    #[test]
    fn the_blank_tells_how_many_letters_are_missing() {
        let blank = |before: &str, after: &str| SpokenWord {
            blank: Some((before.to_owned(), after.to_owned())),
            ..SpokenWord::plain("bateau")
        };
        assert_eq!(Some(3), blank("bat", "").missing_count());
        assert_eq!(Some(1), blank("", "ateau").missing_count());
        let whole = SpokenWord::plain("bateau");
        assert_eq!(None, whole.missing_count());
    }

    #[test]
    fn the_ligature_is_a_letter_of_its_own() {
        assert!(is_right_spelling("sœur", "sœur"));
        assert!(is_right_spelling("Œuf", "œuf"));
        assert!(!is_right_spelling("sœur", "soeur"));
        assert!(!is_right_spelling("poème", "pœme"));
        // « oe » for « œ »: a wrong letter, then one too many.
        let c = correct("sœur", "soeur");
        assert!(!c.correct);
        assert_eq!(c.letters.len(), 5);
        assert!(c
            .letters
            .iter()
            .any(|l| l.expected == Some('œ') && l.kind != LetterKind::Right));
    }

    #[test]
    fn decomposed_accents_are_composed_first() {
        // « e » followed by a combining acute accent.
        assert!(is_right_spelling("été", "e\u{301}te\u{301}"));
        assert!(is_right_spelling("garçon", "garc\u{327}on"));
    }

    #[test]
    fn a_doubled_letter_is_a_letter_too_many() {
        let c = correct("ami", "ammi");
        assert_eq!(
            1,
            c.letters
                .iter()
                .filter(|l| l.kind == LetterKind::Extra)
                .count()
        );
        assert_eq!(4, c.letters.len());
    }

    #[test]
    fn a_forgotten_letter_is_missing() {
        let c = correct("bateau", "batau");
        assert_eq!(
            1,
            c.letters
                .iter()
                .filter(|l| l.kind == LetterKind::Missing)
                .count()
        );
        assert_eq!(6, c.letters.len());
        let missing = c
            .letters
            .iter()
            .find(|l| l.kind == LetterKind::Missing)
            .unwrap();
        assert_eq!(Some('e'), missing.expected);
    }

    #[test]
    fn two_letters_swapped_are_two_wrong_letters() {
        let c = correct("chien", "chein");
        assert_eq!("==xx=", kinds(&c));
    }

    #[test]
    fn missing_letters_complete_the_word() {
        let spoken = SpokenWord {
            blank: Some(("bat".to_owned(), String::new())),
            ..SpokenWord::plain("bateau")
        };
        assert_eq!("bateau", spoken.completed(" eau "));
        assert_eq!("batau", spoken.completed("au"));
    }

    #[test]
    fn a_noun_is_read_with_its_article_any_other_word_with_a_comma() {
        let mut stylo = word("stylo", &[]);
        assert_eq!("un stylo.", stylo.reading());
        stylo.article = Some(Article::Une);
        stylo.text = "école".to_owned();
        assert_eq!("une école.", stylo.reading());
        let mut tres = word("très", &[]);
        tres.article = None;
        tres.category = "adverbe".to_owned();
        assert_eq!("très,", tres.reading());
        let mut fils = word("fils", &[]);
        fils.pronunciation = Some("fisse".to_owned());
        assert_eq!("un fisse.", fils.reading());
        assert_eq!("fils", SpokenWord::of(&fils, None).word);
    }

    /// SUB026708: the word and its sentence at the same speed (0.85), the slow listening at 0.5.
    #[test]
    fn the_word_and_its_sentence_are_read_at_the_same_speed() {
        let mut stylo = word("stylo", &[]);
        stylo.example = Some("J'écris avec un stylo.".to_owned());
        let spoken = SpokenWord::of(&stylo, None);

        let alone = spoken.speech(false, false).unwrap();
        let sentence = spoken.speech(true, false).unwrap();
        let slow = spoken.speech(false, true).unwrap();

        assert_eq!(("un stylo.", 0.85), (alone.text.as_str(), alone.speed));
        assert_eq!(
            ("J'écris avec un stylo.", 0.85),
            (sentence.text.as_str(), sentence.speed)
        );
        assert_eq!(("un stylo.", 0.5), (slow.text.as_str(), slow.speed));
        assert!(SpokenWord::plain("hibou").speech(true, false).is_none());
        assert_eq!("hibou,", SpokenWord::plain("hibou").reading);
    }

    fn word(text: &str, graphemes: &[(&str, &str, usize)]) -> Word {
        Word {
            text: text.to_owned(),
            grades: vec![Grade::CP],
            sounds: graphemes.iter().map(|g| g.1.to_owned()).collect(),
            graphemes: graphemes
                .iter()
                .map(|(t, s, p)| Grapheme {
                    text: (*t).to_owned(),
                    sound: (*s).to_owned(),
                    position: *p,
                })
                .collect(),
            frequency: 1,
            category: "nom".to_owned(),
            example: None,
            article: Some(Article::Un),
            pronunciation: None,
            source: "Tests".to_owned(),
        }
    }

    #[test]
    fn a_filter_keeps_the_words_and_graphemes_of_the_skill() {
        let filter = WordFilter {
            sounds: vec!["o".to_owned()],
            graphemes: vec!["eau".to_owned(), "au".to_owned()],
            ..Default::default()
        };
        let bateau = word("bateau", &[("eau", "o", 3)]);
        let vélo = word("vélo", &[("é", "é", 1), ("o", "o", 3)]);
        assert_eq!(vec![0], filter.select(&bateau).unwrap().worked);
        assert_eq!(None, filter.select(&vélo));

        let mut ce1 = WordFilter {
            grades: vec![Grade::CE1],
            ..Default::default()
        };
        assert_eq!(None, ce1.select(&bateau));
        ce1.grades.push(Grade::CP);
        assert!(ce1.select(&bateau).unwrap().worked.is_empty());
    }

    #[test]
    fn around_splits_the_word_at_a_grapheme() {
        let w = word("château", &[("ch", "ch", 0), ("eau", "o", 4)]);
        assert_eq!(
            ("".to_owned(), "âteau".to_owned()),
            w.around(&w.graphemes[0])
        );
        assert_eq!(
            ("chât".to_owned(), "".to_owned()),
            w.around(&w.graphemes[1])
        );
    }
}
