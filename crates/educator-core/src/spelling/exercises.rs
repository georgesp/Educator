//! The spelling exercises, drawn by the common loop of the families (`exercises::family`): a word of
//! the skill, never twice in a series.

use super::{clean, Grapheme, SkillWord, Sound, SpokenWord, WordSet};
use crate::exercises::{Answer, Draw, Exercise, Input, Position};
use crate::random::SeededRng;
use crate::skills::{AnswerMode, ExerciseFormat, Skill};
use rand::seq::{IndexedRandom, SliceRandom};
use rand::Rng;

/// Proposals of a pick: the right one and two wrong ones (« bato / bateau / batau »).
pub const CHOICES: usize = 3;

/// Room left in the typed word beyond its length (letters too many are part of the mistakes).
const TYPED_MARGIN: usize = 6;

/// Draws one spelling exercise of `skill` for `format`.
pub fn draw(
    skill: &Skill,
    format: ExerciseFormat,
    answer_mode: AnswerMode,
    _position: Position,
    random: &mut SeededRng,
) -> Option<Draw> {
    let words = skill.params.words.as_ref()?;
    let entry = words.words.choose(random)?;
    let exercise = match format {
        ExerciseFormat::Dictation => dictation(words, entry, random),
        ExerciseFormat::MissingLetters => missing_letters(words, entry, answer_mode, random),
        ExerciseFormat::ChooseSpelling => choose_spelling(words, entry, random),
        _ => None,
    }?;
    Some(Draw {
        difficulty: entry.word.frequency,
        exercise,
    })
}

fn spoken_word(entry: &SkillWord, blank: Option<(String, String)>) -> SpokenWord {
    SpokenWord::of(&entry.word, blank)
}

/// A worked grapheme of the word, at random (`None` for a skill of whole words).
fn worked_grapheme<'a>(entry: &'a SkillWord, random: &mut SeededRng) -> Option<&'a Grapheme> {
    entry
        .worked
        .choose(random)
        .and_then(|&i| entry.word.graphemes.get(i))
}

/// The rule recalled after a mistake: « Dans « bateau », le son o s'écrit « eau ». ».
fn explanation(words: &WordSet, entry: &SkillWord, grapheme: Option<&Grapheme>) -> String {
    let word = &entry.word.text;
    let grapheme = grapheme.or_else(|| {
        entry
            .worked
            .first()
            .and_then(|&i| entry.word.graphemes.get(i))
    });
    let Some(grapheme) = grapheme else {
        return format!("On écrit « {word} ».");
    };

    match words.sound(&grapheme.sound) {
        Some(sound) if sound.id == super::SILENT => format!(
            "On écrit « {word} » : la lettre « {} » ne s'entend pas, mais elle s'écrit.",
            grapheme.text
        ),
        Some(sound) if sound.id == super::PLURAL => format!(
            "On écrit « {word} » : au pluriel, on ajoute « {} ».",
            grapheme.text
        ),
        Some(sound) => format!(
            "On écrit « {word} » : ici, {} s'écrit « {} ».",
            sound.spoken, grapheme.text
        ),
        None => format!("On écrit « {word} »."),
    }
}

fn base_exercise(
    format: ExerciseFormat,
    entry: &SkillWord,
    question: &str,
    input: Input,
    expected: String,
) -> Exercise {
    Exercise {
        index: 0,
        format,
        question: Some(question.to_owned()),
        statement: Vec::new(),
        picture: None,
        input,
        expected: Answer::Spelling(expected),
        choices: None,
        spoken: entry.word.text.clone(),
        explanation: String::new(),
        help: None,
        key: entry.word.text.clone(),
        calculation: None,
        word: None,
    }
}

/// SUB001406: the word is read, the child writes it.
fn dictation(words: &WordSet, entry: &SkillWord, random: &mut SeededRng) -> Option<Exercise> {
    let grapheme = worked_grapheme(entry, random);
    let mut exercise = base_exercise(
        ExerciseFormat::Dictation,
        entry,
        "Écoute le mot et écris-le.",
        Input::Spelling {
            max_length: entry.word.text.chars().count() + TYPED_MARGIN,
        },
        entry.word.text.clone(),
    );
    exercise.explanation = explanation(words, entry, grapheme);
    exercise.word = Some(spoken_word(entry, None));
    Some(exercise)
}

/// Graphemes competing with `grapheme`: the other graphemes of its sound, then its confusions.
fn competitors<'a>(sound: &'a Sound, grapheme: &str) -> Vec<&'a str> {
    let mut found: Vec<&str> = Vec::new();
    for g in sound.graphemes.iter().chain(&sound.confusions) {
        if g != grapheme && !found.contains(&g.as_str()) {
            found.push(g);
        }
    }

    found
}

/// SUB001407: the word with a worked grapheme hidden; the child types it or picks it among 3
/// competing graphemes of the same sound.
fn missing_letters(
    words: &WordSet,
    entry: &SkillWord,
    answer_mode: AnswerMode,
    random: &mut SeededRng,
) -> Option<Exercise> {
    let grapheme = worked_grapheme(entry, random)?;
    let (before, after) = entry.word.around(grapheme);
    let mut exercise = base_exercise(
        ExerciseFormat::MissingLetters,
        entry,
        "Écris les lettres qui manquent.",
        Input::Spelling {
            max_length: grapheme.text.chars().count() + TYPED_MARGIN,
        },
        grapheme.text.clone(),
    );
    if answer_mode == AnswerMode::Choice {
        let sound = words.sound(&grapheme.sound)?;
        // The other graphemes of the sound first (shuffled), then nothing at all for a silent letter
        // or a plural mark (« des chat_ » : s, x or nothing), then the confusions. A competitor that
        // also makes a real word (« ver_ » → « vers ») is left out: both would be right.
        let mut own: Vec<&str> = sound
            .graphemes
            .iter()
            .map(String::as_str)
            .filter(|g| *g != grapheme.text)
            .collect();
        own.shuffle(random);
        let nothing: &[&str] = if sound.may_be_left_out() { &[""] } else { &[] };
        let others: Vec<&str> = own
            .into_iter()
            .chain(nothing.iter().copied())
            .chain(sound.confusions.iter().map(String::as_str))
            .filter(|c| *c != grapheme.text && !words.is_bank_word(&format!("{before}{c}{after}")))
            .collect();
        let mut picked: Vec<&str> = others.into_iter().take(CHOICES - 1).collect();
        if picked.len() < CHOICES - 1 {
            return None;
        }

        picked.push(&grapheme.text);
        picked.shuffle(random);
        exercise.question = Some("Choisis les lettres qui manquent.".to_owned());
        exercise.input = Input::Pick;
        exercise.choices = Some(
            picked
                .into_iter()
                .map(|c| Answer::Spelling(c.to_owned()))
                .collect(),
        );
    }

    exercise.explanation = explanation(words, entry, Some(grapheme));
    exercise.word = Some(spoken_word(entry, Some((before, after))));
    Some(exercise)
}

/// SUB001408: the word is read; three spellings, only one right.
fn choose_spelling(words: &WordSet, entry: &SkillWord, random: &mut SeededRng) -> Option<Exercise> {
    let wrong = wrong_spellings(words, entry, CHOICES - 1, random);
    if wrong.len() < CHOICES - 1 {
        return None;
    }

    let mut exercise = base_exercise(
        ExerciseFormat::ChooseSpelling,
        entry,
        "Écoute le mot et choisis la bonne écriture.",
        Input::Pick,
        entry.word.text.clone(),
    );
    let mut choices: Vec<Answer> = wrong.into_iter().map(Answer::Spelling).collect();
    choices.push(Answer::Spelling(entry.word.text.clone()));
    choices.shuffle(random);
    exercise.choices = Some(choices);
    exercise.explanation = explanation(words, entry, None);
    exercise.word = Some(spoken_word(entry, None));
    Some(exercise)
}

/// Up to `count` wrong spellings of a word: a grapheme replaced by a competing grapheme of the same
/// sound (the worked ones first), a silent letter or a plural mark left out; then a letter doubled
/// or undoubled. Never the word itself, never a real word of the bank, never twice the same.
pub fn wrong_spellings(
    words: &WordSet,
    entry: &SkillWord,
    count: usize,
    random: &mut impl Rng,
) -> Vec<String> {
    let word = &entry.word;
    let replaced = |g: &Grapheme, by: &str| {
        let (before, after) = word.around(g);
        format!("{before}{by}{after}")
    };
    let swaps = |g: &Grapheme| -> Vec<String> {
        let Some(sound) = words.sound(&g.sound) else {
            return Vec::new();
        };
        let mut found: Vec<String> = competitors(sound, &g.text)
            .into_iter()
            .map(|c| replaced(g, c))
            .collect();
        if sound.may_be_left_out() {
            found.push(replaced(g, ""));
        }
        found
    };

    let mut worked: Vec<String> = Vec::new();
    for &i in &entry.worked {
        if let Some(g) = word.graphemes.get(i) {
            worked.extend(swaps(g));
        }
    }
    worked.shuffle(random);

    let mut others: Vec<String> = word
        .graphemes
        .iter()
        .enumerate()
        .filter(|(i, _)| !entry.worked.contains(i))
        .flat_map(|(_, g)| swaps(g))
        .collect();
    others.shuffle(random);

    let mut anywhere = swaps_anywhere(words, &word.text);
    anywhere.shuffle(random);

    let mut letters = letter_mistakes(&word.text);
    letters.shuffle(random);

    let mut found: Vec<String> = Vec::with_capacity(count);
    for candidate in worked
        .into_iter()
        .chain(others)
        .chain(anywhere)
        .chain(letters)
    {
        if found.len() == count {
            break;
        }

        let candidate = clean(&candidate);
        let fine = !candidate.is_empty()
            && !super::is_right_spelling(&word.text, &candidate)
            && !words.is_bank_word(&candidate)
            && !found
                .iter()
                .any(|f| super::is_right_spelling(f, &candidate));
        if fine {
            found.push(candidate);
        }
    }

    found
}

/// Any grapheme of any sound found in the word, replaced by a competitor (for the words whose
/// graphemes are not listed, like some « mots outils »).
fn swaps_anywhere(words: &WordSet, text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for sound in words.sounds.iter().filter(|s| !s.may_be_left_out()) {
        for grapheme in sound.graphemes.iter().filter(|g| !g.is_empty()) {
            for (start, _) in text.match_indices(grapheme.as_str()) {
                let (before, after) = (&text[..start], &text[start + grapheme.len()..]);
                for by in competitors(sound, grapheme) {
                    found.push(format!("{before}{by}{after}"));
                }
            }
        }
    }

    found
}

/// A consonant doubled or a double consonant made single (« ami » → « ammi », « pomme » → « pome »).
fn letter_mistakes(word: &str) -> Vec<String> {
    let letters: Vec<char> = word.chars().collect();
    let consonant = |c: char| c.is_alphabetic() && !"aeiouyàâéèêëîïôöùûüœ".contains(c);
    let mut found = Vec::new();
    for i in 1..letters.len() {
        let c = letters[i];
        if !consonant(c) {
            continue;
        }

        if letters[i - 1] == c {
            let mut single = letters.clone();
            single.remove(i);
            found.push(single.into_iter().collect());
        } else if i + 1 < letters.len() && letters[i + 1] != c {
            let mut double = letters.clone();
            double.insert(i, c);
            found.push(double.into_iter().collect());
        }
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spelling::{Grapheme, Word};
    use crate::Grade;
    use rand::SeedableRng;
    use std::sync::Arc;

    fn sound(id: &str, graphemes: &[&str], confusions: &[&str]) -> Sound {
        Sound {
            id: id.to_owned(),
            label: format!("[{id}]"),
            spoken: format!("le son {id}"),
            graphemes: graphemes.iter().map(|g| (*g).to_owned()).collect(),
            confusions: confusions.iter().map(|g| (*g).to_owned()).collect(),
        }
    }

    fn entry(text: &str, graphemes: &[(&str, &str, usize)]) -> SkillWord {
        SkillWord {
            word: Word {
                text: text.to_owned(),
                grades: vec![Grade::CE1],
                sounds: Vec::new(),
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
                article: None,
                pronunciation: None,
                source: "Tests".to_owned(),
            },
            worked: (0..graphemes.len()).collect(),
        }
    }

    fn set(words: Vec<SkillWord>, bank: &[&str]) -> WordSet {
        let mut bank: Vec<String> = bank.iter().map(|w| (*w).to_owned()).collect();
        bank.extend(words.iter().map(|w| w.word.text.clone()));
        bank.sort();
        WordSet {
            words,
            sounds: Arc::new(vec![
                sound("o", &["o", "au", "eau"], &[]),
                sound("ou", &["ou"], &["o", "u"]),
                sound("muet", &["t", "d", "s", "x", "p"], &[]),
                sound("pluriel", &["s", "x"], &[]),
            ]),
            bank: Arc::new(bank),
        }
    }

    #[test]
    fn wrong_spellings_replace_a_grapheme_by_a_competitor() {
        let bateau = entry("bateau", &[("eau", "o", 3)]);
        let words = set(vec![bateau.clone()], &[]);
        let mut random = SeededRng::seed_from_u64(1);
        let mut wrong = wrong_spellings(&words, &bateau, 2, &mut random);
        wrong.sort();
        assert_eq!(vec!["batau", "bato"], wrong);
    }

    #[test]
    fn wrong_spellings_are_never_real_words() {
        // « pot » with its silent t: « po » and « pos » are fine, « peau » is a real word.
        let pot = entry("pot", &[("o", "o", 1), ("t", "muet", 2)]);
        let words = set(vec![pot.clone()], &["peau", "pod"]);
        for seed in 0..50 {
            let mut random = SeededRng::seed_from_u64(seed);
            for w in wrong_spellings(&words, &pot, 2, &mut random) {
                assert!(!["pot", "peau", "pod"].contains(&w.as_str()), "{w}");
            }
        }
    }

    #[test]
    fn a_word_without_graphemes_gets_letter_mistakes() {
        let alors = SkillWord {
            worked: Vec::new(),
            ..entry("alors", &[])
        };
        let words = set(vec![alors.clone()], &[]);
        let mut random = SeededRng::seed_from_u64(3);
        let wrong = wrong_spellings(&words, &alors, 2, &mut random);
        assert_eq!(2, wrong.len());
        assert!(wrong.iter().all(|w| w != "alors"));
    }

    #[test]
    fn missing_letters_hide_a_worked_grapheme_and_propose_competitors() {
        let bateau = entry("bateau", &[("eau", "o", 3)]);
        let words = set(vec![bateau.clone()], &[]);
        let mut random = SeededRng::seed_from_u64(9);
        let exercise = missing_letters(&words, &bateau, AnswerMode::Choice, &mut random).unwrap();
        let spoken = exercise.word.clone().unwrap();
        assert_eq!(Some(("bat".to_owned(), String::new())), spoken.blank);
        let mut choices: Vec<String> = exercise
            .choices
            .as_ref()
            .unwrap()
            .iter()
            .map(Answer::label)
            .collect();
        choices.sort();
        assert_eq!(vec!["au", "eau", "o"], choices);
        assert_eq!(Answer::Spelling("eau".to_owned()), exercise.expected);
        assert_eq!("bat_", exercise.prompt());
    }

    #[test]
    fn missing_letters_of_a_sound_with_one_grapheme_use_its_confusions() {
        let loup = entry("loup", &[("ou", "ou", 1)]);
        let words = set(vec![loup.clone()], &[]);
        let mut random = SeededRng::seed_from_u64(2);
        let exercise = missing_letters(&words, &loup, AnswerMode::Choice, &mut random).unwrap();
        let mut choices: Vec<String> = exercise
            .choices
            .unwrap()
            .iter()
            .map(Answer::label)
            .collect();
        choices.sort();
        assert_eq!(vec!["o", "ou", "u"], choices);
    }

    #[test]
    fn a_plural_mark_may_be_left_out() {
        let chats = entry("des chats", &[("s", "pluriel", 8)]);
        let words = set(vec![chats.clone()], &[]);
        let mut random = SeededRng::seed_from_u64(5);
        let exercise = missing_letters(&words, &chats, AnswerMode::Choice, &mut random).unwrap();
        let mut choices: Vec<String> = exercise
            .choices
            .unwrap()
            .iter()
            .map(Answer::label)
            .collect();
        choices.sort();
        assert_eq!(vec!["", "s", "x"], choices);
    }

    #[test]
    fn a_competitor_making_a_real_word_is_not_proposed() {
        // « ver_ » : « vers » is a real word, only « verd » and « verx » are proposed besides « vert ».
        let vert = entry("vert", &[("t", "muet", 3)]);
        let words = set(vec![vert.clone()], &["vers"]);
        for seed in 0..20 {
            let mut random = SeededRng::seed_from_u64(seed);
            let exercise = missing_letters(&words, &vert, AnswerMode::Choice, &mut random).unwrap();
            assert!(exercise.choices.unwrap().iter().all(|c| c.label() != "s"));
        }
    }

    #[test]
    fn dictation_reads_the_word_and_expects_it_written() {
        let bateau = entry("bateau", &[("eau", "o", 3)]);
        let words = set(vec![bateau.clone()], &[]);
        let mut random = SeededRng::seed_from_u64(4);
        let exercise = dictation(&words, &bateau, &mut random).unwrap();
        assert_eq!("bateau", exercise.word.as_ref().unwrap().word);
        assert!(exercise.is_correct(&Answer::Text("Bateau".to_owned())));
        assert!(!exercise.is_correct(&Answer::Text("bato".to_owned())));
        assert_eq!(
            "On écrit « bateau » : ici, le son o s'écrit « eau ».",
            exercise.explanation
        );
        // The written prompt never shows the word to find.
        assert!(!exercise.prompt().contains("bateau"));
    }
}
