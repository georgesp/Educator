//! The mascot (SUB001417): an owl who presents the lessons and congratulates or encourages at the
//! end of a series. Her pictures and phrases are content (`content/mascot/`); this module picks the
//! expression of each situation and draws a phrase, never twice in a row the same.

use crate::random::RandomSource;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

/// The four expressions of the mascot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Expression {
    Happy,
    Thinking,
    Encouraging,
    Party,
}

impl Expression {
    pub const ALL: [Expression; 4] = [
        Expression::Happy,
        Expression::Thinking,
        Expression::Encouraging,
        Expression::Party,
    ];

    /// File of the picture, in the mascot folder.
    pub fn file_name(self) -> &'static str {
        match self {
            Expression::Happy => "content.svg",
            Expression::Thinking => "reflechit.svg",
            Expression::Encouraging => "encourage.svg",
            Expression::Party => "fete.svg",
        }
    }
}

/// When the mascot speaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Situation {
    /// At the start of a lesson: « Aujourd'hui, on apprend… » (`{title}` replaced by the lesson).
    LessonStart,
    /// End of a series without any mistake.
    SeriesPerfect,
    /// End of a series with two thirds or more right.
    SeriesWell,
    /// End of a harder series.
    SeriesEncourage,
    ExamEnd,
    TimedEnd,
    /// A new level (lot 3).
    LevelUp,
}

impl Situation {
    pub const ALL: [Situation; 7] = [
        Situation::LessonStart,
        Situation::SeriesPerfect,
        Situation::SeriesWell,
        Situation::SeriesEncourage,
        Situation::ExamEnd,
        Situation::TimedEnd,
        Situation::LevelUp,
    ];

    /// Name in `content/mascot/mascot.json`.
    pub fn name(self) -> &'static str {
        match self {
            Situation::LessonStart => "lessonStart",
            Situation::SeriesPerfect => "seriesPerfect",
            Situation::SeriesWell => "seriesWell",
            Situation::SeriesEncourage => "seriesEncourage",
            Situation::ExamEnd => "examEnd",
            Situation::TimedEnd => "timedEnd",
            Situation::LevelUp => "levelUp",
        }
    }

    /// The expression of the mascot in this situation.
    pub fn expression(self) -> Expression {
        match self {
            Situation::LessonStart | Situation::SeriesWell => Expression::Happy,
            Situation::SeriesPerfect | Situation::LevelUp | Situation::TimedEnd => {
                Expression::Party
            }
            Situation::SeriesEncourage => Expression::Encouraging,
            Situation::ExamEnd => Expression::Thinking,
        }
    }

    /// The end of a series of exercises or of a quiz: a party without any mistake, happy from two
    /// thirds right, encouraging below.
    pub fn after_series(correct: usize, total: usize) -> Situation {
        if total > 0 && correct == total {
            Situation::SeriesPerfect
        } else if correct * 3 >= total * 2 && total > 0 {
            Situation::SeriesWell
        } else {
            Situation::SeriesEncourage
        }
    }
}

/// The mascot as loaded from the content.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Mascot {
    pub name: String,
    /// SVG source of each expression.
    pub pictures: HashMap<Expression, String>,
    /// Phrases of each situation (at least three each).
    pub phrases: HashMap<Situation, Vec<String>>,
}

/// A phrase said by the mascot, with its expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MascotPhrase {
    pub text: String,
    pub expression: Expression,
}

/// Draws the phrases of the mascot, never twice in a row the same in a situation.
pub struct MascotService {
    mascot: Mascot,
    random: Arc<dyn RandomSource>,
    last: Mutex<HashMap<Situation, usize>>,
}

impl fmt::Debug for MascotService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MascotService")
            .field("name", &self.mascot.name)
            .finish_non_exhaustive()
    }
}

impl MascotService {
    pub fn new(mascot: Mascot, random: Arc<dyn RandomSource>) -> Self {
        Self {
            mascot,
            random,
            last: Mutex::new(HashMap::new()),
        }
    }

    pub fn mascot(&self) -> &Mascot {
        &self.mascot
    }

    /// A phrase of the situation, `{title}` replaced by `title`; `None` when the situation has none.
    pub fn phrase(&self, situation: Situation, title: Option<&str>) -> Option<MascotPhrase> {
        let phrases = self
            .mascot
            .phrases
            .get(&situation)
            .filter(|p| !p.is_empty())?;
        let mut last = self.last.lock().unwrap_or_else(|e| e.into_inner());
        let previous = last.get(&situation).copied();
        let count = u32::try_from(phrases.len()).unwrap_or(u32::MAX);
        let mut index = self.random.next(count) as usize;
        if phrases.len() > 1 && Some(index) == previous {
            index = (index + 1) % phrases.len();
        }
        last.insert(situation, index);
        Some(MascotPhrase {
            text: phrases[index].replace("{title}", title.unwrap_or_default().trim()),
            expression: situation.expression(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::ScriptedRandom;

    fn mascot() -> Mascot {
        Mascot {
            name: "Plume".to_owned(),
            pictures: HashMap::new(),
            phrases: HashMap::from([
                (
                    Situation::LessonStart,
                    vec![
                        "On apprend : {title}.".to_owned(),
                        "C'est parti pour {title} !".to_owned(),
                        "Aujourd'hui : {title}.".to_owned(),
                    ],
                ),
                (Situation::SeriesPerfect, vec!["Bravo !".to_owned()]),
            ]),
        }
    }

    #[test]
    fn a_phrase_never_comes_twice_in_a_row() {
        // The random source draws 0 every time: the second draw moves to the next phrase.
        let service = MascotService::new(mascot(), Arc::new(ScriptedRandom::new([0])));
        let first = service
            .phrase(Situation::LessonStart, Some("Le son [o]"))
            .unwrap();
        let second = service
            .phrase(Situation::LessonStart, Some("Le son [o]"))
            .unwrap();
        assert_eq!("On apprend : Le son [o].", first.text);
        assert_eq!("C'est parti pour Le son [o] !", second.text);
        assert_eq!(Expression::Happy, first.expression);
        // A single phrase may come back.
        assert_eq!(
            "Bravo !",
            service.phrase(Situation::SeriesPerfect, None).unwrap().text
        );
        assert_eq!(
            "Bravo !",
            service.phrase(Situation::SeriesPerfect, None).unwrap().text
        );
        assert_eq!(None, service.phrase(Situation::LevelUp, None));
    }

    #[test]
    fn the_end_of_a_series_decides_the_mood() {
        assert_eq!(Situation::SeriesPerfect, Situation::after_series(10, 10));
        assert_eq!(Situation::SeriesWell, Situation::after_series(7, 10));
        assert_eq!(Situation::SeriesEncourage, Situation::after_series(6, 10));
        assert_eq!(Situation::SeriesEncourage, Situation::after_series(0, 0));
        assert_eq!(Expression::Party, Situation::SeriesPerfect.expression());
        assert_eq!(
            Expression::Encouraging,
            Situation::SeriesEncourage.expression()
        );
    }
}
