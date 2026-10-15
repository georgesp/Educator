use super::{Answer, CalculationExercise, Input, Picture, Token};
use crate::skills::ExerciseFormat;
use crate::spelling::{self, SpellingCorrection, SpokenWord};

pub const PLACEHOLDER: &str = "?";

/// One exercise of a series, whatever its family (calculation, numeration, doubles and halves,
/// fractions, problems). The expected answer stays in the application core: the front receives
/// the statement, the picture, the kind of input and, for a pick, the proposals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exercise {
    /// Position in the series, from 0.
    pub index: usize,
    pub format: ExerciseFormat,
    /// Sentence shown above the statement (« Quel est le double de 35 ? »), or a problem.
    pub question: Option<String>,
    /// The written statement, with the answer slots.
    pub statement: Vec<Token>,
    pub picture: Option<Picture>,
    pub input: Input,
    pub expected: Answer,
    /// Proposals including the right one, for [`Input::Pick`].
    pub choices: Option<Vec<Answer>>,
    /// French text read aloud (numbers are converted to words by the speech layer).
    pub spoken: String,
    /// Correction shown and read after a mistake.
    pub explanation: String,
    /// Picture shown with the correction (decomposition, bar model).
    pub help: Option<Picture>,
    /// Two exercises of a series never share a key (no duplicate).
    pub key: String,
    /// The calculation, for the calculation families.
    pub calculation: Option<CalculationExercise>,
    /// Spelling: the word read by the voice (ADR 0004) and the letters around the blank.
    pub word: Option<SpokenWord>,
}

impl Exercise {
    pub fn is_correct(&self, answer: &Answer) -> bool {
        self.expected.accepts(answer)
    }

    /// The expected answer as written in the correction (« 3/4 », « < », « 300 et 400 »).
    pub fn expected_label(&self) -> String {
        self.expected.label()
    }

    /// Spelling: the child's whole word, letter by letter against the expected one (the letters
    /// typed or picked put back in the word for the missing letters). `None` for the other families.
    pub fn correction(&self, given: &Answer) -> Option<SpellingCorrection> {
        let (Some(word), Answer::Text(text)) = (&self.word, given) else {
            return None;
        };

        Some(spelling::correct(&word.word, &word.completed(text)))
    }

    /// Written form of the whole exercise, with `?` for the answer: the question, then the
    /// statement (reviews of the samples, accessibility). A spelling exercise never shows the word
    /// to find: « Dictée », « bat_ ».
    pub fn prompt(&self) -> String {
        match (self.format, &self.word) {
            (ExerciseFormat::Dictation, _) => return "Dictée".to_owned(),
            (ExerciseFormat::ChooseSpelling, _) => return "Choisis la bonne écriture".to_owned(),
            (
                _,
                Some(SpokenWord {
                    blank: Some((before, after)),
                    ..
                }),
            ) => return format!("{before}_{after}"),
            _ => {}
        }

        let statement = self
            .statement
            .iter()
            .map(|t| match t {
                Token::Number(n) => n.to_string(),
                Token::Sign(s) | Token::Text(s) => s.clone(),
                Token::Fraction(f) => f.to_string(),
                Token::Slot(_) => PLACEHOLDER.to_owned(),
            })
            .collect::<Vec<_>>()
            .join(" ");
        let only_slot = matches!(self.statement.as_slice(), [Token::Slot(_)]);
        match (&self.question, statement.is_empty() || only_slot) {
            (Some(question), true) => question.clone(),
            (Some(question), false) => format!("{question} {statement}"),
            (None, _) => statement,
        }
    }
}
