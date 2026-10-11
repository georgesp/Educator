use super::{ArithmeticOperation, Calculation};
use crate::skills::ExerciseFormat;

/// Which number of the equality the child must find.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HiddenPart {
    Left,
    Right,
    Result,
}

pub const PLACEHOLDER: &str = "?";

/// One exercise of a series. The expected answer stays in the application core: the front receives
/// the equality with the hidden number removed, and the choices in QCM mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exercise {
    /// Position in the series, from 0.
    pub index: usize,
    pub calculation: Calculation,
    pub hidden: HiddenPart,
    /// Four proposals including the right one (QCM mode), otherwise `None`.
    pub choices: Option<Vec<i32>>,
}

impl Exercise {
    pub fn new(index: usize, calculation: Calculation, hidden: HiddenPart) -> Self {
        Self {
            index,
            calculation,
            hidden,
            choices: None,
        }
    }

    pub fn format(&self) -> ExerciseFormat {
        if self.hidden == HiddenPart::Result {
            ExerciseFormat::Result
        } else {
            ExerciseFormat::MissingOperand
        }
    }

    pub fn expected(&self) -> i32 {
        match self.hidden {
            HiddenPart::Left => self.calculation.left,
            HiddenPart::Right => self.calculation.right,
            HiddenPart::Result => self.calculation.result(),
        }
    }

    /// The equality as shown, e.g. `7 + ? = 12`.
    pub fn prompt(&self) -> String {
        let c = &self.calculation;
        format!(
            "{} {} {} = {}",
            self.show(HiddenPart::Left, c.left),
            c.symbol(),
            self.show(HiddenPart::Right, c.right),
            self.show(HiddenPart::Result, c.result())
        )
    }

    /// French question read aloud (numbers are converted to words by the speech layer).
    pub fn spoken_prompt(&self) -> String {
        let (left, right, result) = (
            self.calculation.left,
            self.calculation.right,
            self.calculation.result(),
        );
        let addition = self.calculation.operation == ArithmeticOperation::Addition;
        match (self.hidden, addition) {
            (HiddenPart::Result, true) => format!("Combien font {left} plus {right} ?"),
            (HiddenPart::Result, false) => format!("Combien font {left} moins {right} ?"),
            (HiddenPart::Right, true) => {
                format!("Combien faut-il ajouter à {left} pour obtenir {result} ?")
            }
            (HiddenPart::Right, false) => {
                format!("Combien faut-il enlever à {left} pour obtenir {result} ?")
            }
            (HiddenPart::Left, true) => format!("Quel nombre plus {right} égale {result} ?"),
            (HiddenPart::Left, false) => format!("Quel nombre moins {right} égale {result} ?"),
        }
    }

    /// Correction shown and read after a mistake, e.g. `7 + 5 = 12`.
    pub fn explanation(&self) -> String {
        self.calculation.to_string()
    }

    pub fn is_correct(&self, answer: i32) -> bool {
        answer == self.expected()
    }

    fn show(&self, part: HiddenPart, value: i32) -> String {
        if part == self.hidden {
            PLACEHOLDER.to_owned()
        } else {
            value.to_string()
        }
    }
}
