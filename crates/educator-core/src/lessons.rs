//! Lessons (SUB001409, SUB001410, SUB001411): short animated lessons made of slides, followed by a
//! quiz. A lesson is content (`content/lessons/<subject>/*.json`, loaded and validated by
//! `educator-content`); this module holds its model, the values the animations show (computed here,
//! never in the front) and the checking of the quiz answers.

use crate::exercises::Answer;
use crate::skills::Subject;
use crate::Grade;

/// A lesson: slides to watch and manipulate, then a short quiz, then the linked exercises.
#[derive(Debug, Clone, PartialEq)]
pub struct Lesson {
    pub id: String,
    pub subject: Subject,
    pub grade: Grade,
    /// School period (1 to 5) where the lesson is offered.
    pub period: u8,
    /// Skills practised after the lesson (« Je m'entraîne »), the first one being the main one.
    pub skill_ids: Vec<String>,
    pub title: String,
    pub duration_minutes: u8,
    pub slides: Vec<Slide>,
    pub quiz: Vec<QuizQuestion>,
    pub sources: Vec<LessonSource>,
}

/// One slide: a short text shown, a longer text read aloud, an animation.
#[derive(Debug, Clone, PartialEq)]
pub struct Slide {
    /// Short heading (« Rappel », « À toi »).
    pub title: String,
    /// Text shown to the child: one or two short sentences.
    pub text: String,
    /// Text read by the voice (may say more than `text`).
    pub speech: String,
    pub animation: Animation,
}

impl Slide {
    /// Whether the child has to do something before going on (click, match).
    pub fn is_interactive(&self) -> bool {
        self.animation.is_interactive()
    }
}

/// The animations of the library (SUB000117), with their parameters. Only the ones used by the
/// written lessons exist for now; the others of the catalogue are added with their first lesson.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Animation {
    /// Title screen: checkered strip, title, subtitle.
    TitleCard {
        title: String,
        subtitle: Option<String>,
    },
    /// « Je retiens »: the rule in one sentence, with an example.
    RecapCard {
        text: String,
        example: Option<String>,
    },
    /// Mental calculation as chains of arrows (47 → + 3 → 50 → + 5 → 55).
    CalcChain {
        chains: Vec<Chain>,
        /// `Show`: the chains appear one step after the other. `Match`: the child links each
        /// calculation (`start + total`) to its chain of steps.
        mode: ChainMode,
    },
    /// A graduated track: the car jumps from `start`, one jump per step.
    NumberLine {
        from: i32,
        to: i32,
        /// Gap between two graduations.
        step: i32,
        start: i32,
        jumps: Vec<i32>,
        /// The child clicks where the car lands after each jump.
        interactive: bool,
        /// On an interactive line, the value of a graduation shows when the pointer is over it. Off
        /// when finding a number on the line is what the lesson teaches.
        hover_values: bool,
    },
    /// Base ten blocks: ten bars and unit cubes for `value`; `complete_ten` shows in dotted lines
    /// the cubes missing to the next ten (19 is « presque 20 »).
    TensUnits { value: u32, complete_ten: bool },
}

impl Animation {
    pub fn is_interactive(&self) -> bool {
        match self {
            Animation::CalcChain { mode, .. } => *mode == ChainMode::Match,
            Animation::NumberLine { interactive, .. } => *interactive,
            _ => false,
        }
    }

    /// Name used in the content files (`"numberLine"`).
    pub fn type_name(&self) -> &'static str {
        match self {
            Animation::TitleCard { .. } => "titleCard",
            Animation::RecapCard { .. } => "recapCard",
            Animation::CalcChain { .. } => "calcChain",
            Animation::NumberLine { .. } => "numberLine",
            Animation::TensUnits { .. } => "tensUnits",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainMode {
    Show,
    Match,
}

/// `start`, then each step (+ 20, − 1). A `crossed` chain is a classic mistake, shown crossed out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chain {
    pub start: i32,
    pub steps: Vec<i32>,
    pub crossed: bool,
}

impl Chain {
    /// The value after each step: 46, [+20, −1] → [66, 65].
    pub fn values(&self) -> Vec<i32> {
        landings(self.start, &self.steps)
    }

    /// What the whole chain adds: [+20, −1] → 19.
    pub fn total(&self) -> i32 {
        self.steps.iter().sum()
    }

    /// The final value.
    pub fn result(&self) -> i32 {
        self.start + self.total()
    }
}

/// Where the car lands after each jump.
pub fn landings(start: i32, jumps: &[i32]) -> Vec<i32> {
    jumps
        .iter()
        .scan(start, |at, jump| {
            *at += jump;
            Some(*at)
        })
        .collect()
}

/// A question of the end-of-lesson quiz: typed (`choices` empty) or picked among `choices`.
#[derive(Debug, Clone, PartialEq)]
pub struct QuizQuestion {
    /// Shown, e.g. « 46 + 19 = ? ».
    pub question: String,
    /// Read aloud, e.g. « Combien font 46 plus 19 ? ».
    pub spoken: String,
    pub answer: Answer,
    pub choices: Vec<Answer>,
    /// Shown and read after the answer, right or wrong.
    pub explanation: String,
}

/// A reference consulted to write the lesson.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonSource {
    pub title: String,
    pub url: String,
}

/// Result of a quiz answer.
#[derive(Debug, Clone, PartialEq)]
pub struct QuizOutcome {
    pub correct: bool,
    pub expected: Answer,
    pub explanation: String,
}

impl Lesson {
    /// Checks the answer to question `index`; `None` when there is no such question.
    pub fn check_quiz(&self, index: usize, given: &Answer) -> Option<QuizOutcome> {
        let question = self.quiz.get(index)?;
        Some(QuizOutcome {
            correct: question.answer.accepts(given),
            expected: question.answer.clone(),
            explanation: question.explanation.clone(),
        })
    }

    /// Number of slides where the child manipulates.
    pub fn interactive_slides(&self) -> usize {
        self.slides.iter().filter(|s| s.is_interactive()).count()
    }
}

/// The lessons loaded at start-up.
pub trait LessonCatalog: Send + Sync {
    fn all(&self) -> &[Lesson];

    fn find(&self, id: &str) -> Option<&Lesson> {
        self.all().iter().find(|l| l.id == id)
    }

    /// The lessons of a grade, by period then in file order.
    fn for_grade(&self, subject: Subject, grade: Grade) -> Vec<&Lesson> {
        let mut lessons: Vec<&Lesson> = self
            .all()
            .iter()
            .filter(|l| l.subject == subject && l.grade == grade)
            .collect();
        lessons.sort_by_key(|l| l.period);
        lessons
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chain(start: i32, steps: &[i32]) -> Chain {
        Chain {
            start,
            steps: steps.to_vec(),
            crossed: false,
        }
    }

    #[test]
    fn a_chain_gives_the_value_after_each_step() {
        let c = chain(46, &[20, -1]);
        assert_eq!(vec![66, 65], c.values());
        assert_eq!(19, c.total());
        assert_eq!(65, c.result());
    }

    #[test]
    fn landings_follow_the_jumps() {
        assert_eq!(vec![77, 76], landings(57, &[20, -1]));
        assert!(landings(3, &[]).is_empty());
    }

    #[test]
    fn quiz_answers_are_checked_by_the_core() {
        let lesson = Lesson {
            id: "l".into(),
            subject: Subject::Math,
            grade: Grade::CE1,
            period: 2,
            skill_ids: vec!["s".into()],
            title: "t".into(),
            duration_minutes: 4,
            slides: Vec::new(),
            quiz: vec![QuizQuestion {
                question: "28 + 9 = ?".into(),
                spoken: "Combien font 28 plus 9 ?".into(),
                answer: Answer::Number(37),
                choices: Vec::new(),
                explanation: "28 + 10 = 38, puis 38 − 1 = 37.".into(),
            }],
            sources: Vec::new(),
        };

        assert!(lesson.check_quiz(0, &Answer::Number(37)).unwrap().correct);
        let wrong = lesson.check_quiz(0, &Answer::Number(39)).unwrap();
        assert!(!wrong.correct);
        assert_eq!(Answer::Number(37), wrong.expected);
        assert!(lesson.check_quiz(1, &Answer::Number(37)).is_none());
    }
}
