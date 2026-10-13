use super::{
    Answer, ArithmeticOperation, Calculation, Exercise, Input, Picture, Token, PLACEHOLDER,
};
use crate::skills::ExerciseFormat;

/// Which number of the equality the child must find.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HiddenPart {
    Left,
    Right,
    Result,
}

/// One calculation exercise of a series (addition, subtraction, multiplication). The expected
/// answer stays in the application core: the front receives the equality with the hidden number
/// removed, and the choices in QCM mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalculationExercise {
    /// Position in the series, from 0.
    pub index: usize,
    pub calculation: Calculation,
    pub hidden: HiddenPart,
    /// [`ExerciseFormat::Result`], [`ExerciseFormat::MissingOperand`] or, for a multiplication,
    /// [`ExerciseFormat::Phrase`] (repeated addition: `5 + 5 + 5 = ? × 5`).
    pub format: ExerciseFormat,
    /// The result is written first: `30 = 3 × ?` (multiplication, SUB000119).
    pub reversed: bool,
    /// Four proposals including the right one (QCM mode), otherwise `None`.
    pub choices: Option<Vec<i32>>,
}

impl CalculationExercise {
    pub fn new(index: usize, calculation: Calculation, hidden: HiddenPart) -> Self {
        let format = if hidden == HiddenPart::Result {
            ExerciseFormat::Result
        } else {
            ExerciseFormat::MissingOperand
        };
        Self {
            index,
            calculation,
            hidden,
            format,
            reversed: false,
            choices: None,
        }
    }

    pub fn format(&self) -> ExerciseFormat {
        self.format
    }

    pub fn expected(&self) -> i32 {
        match self.hidden {
            HiddenPart::Left => self.calculation.left,
            HiddenPart::Right => self.calculation.right,
            HiddenPart::Result => self.calculation.result(),
        }
    }

    /// Repeated addition: `5 + 5 + 5` for 3 × 5 (the left operand counts the terms).
    fn repeated_addition(&self) -> String {
        let c = &self.calculation;
        vec![c.right.to_string(); c.left.max(1) as usize].join(" + ")
    }

    /// The equality as shown, e.g. `7 + ? = 12`, `30 = 3 × ?`, `5 + 5 + 5 = ? × 5`.
    pub fn prompt(&self) -> String {
        self.tokens()
            .iter()
            .map(|t| match t {
                Token::Number(n) => n.to_string(),
                Token::Sign(s) | Token::Text(s) => s.clone(),
                Token::Fraction(f) => f.to_string(),
                Token::Slot(_) => PLACEHOLDER.to_owned(),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn tokens(&self) -> Vec<Token> {
        let c = &self.calculation;
        let part = |part: HiddenPart, value: i32| {
            if part == self.hidden {
                Token::Slot(0)
            } else {
                Token::Number(value)
            }
        };
        let sign = |s: &str| Token::Sign(s.to_owned());
        if self.format == ExerciseFormat::Phrase {
            let mut tokens = Vec::new();
            for i in 0..c.left.max(1) {
                if i > 0 {
                    tokens.push(sign("+"));
                }
                tokens.push(Token::Number(c.right));
            }
            tokens.push(sign("="));
            tokens.push(part(HiddenPart::Left, c.left));
            tokens.push(sign("×"));
            tokens.push(part(HiddenPart::Right, c.right));
            return tokens;
        }

        let operation = vec![
            part(HiddenPart::Left, c.left),
            sign(c.symbol()),
            part(HiddenPart::Right, c.right),
        ];
        let result = part(HiddenPart::Result, c.result());
        if self.reversed {
            let mut tokens = vec![result, sign("=")];
            tokens.extend(operation);
            tokens
        } else {
            let mut tokens = operation;
            tokens.push(sign("="));
            tokens.push(result);
            tokens
        }
    }

    /// French question read aloud (numbers are converted to words by the speech layer).
    pub fn spoken_prompt(&self) -> String {
        let (left, right, result) = (
            self.calculation.left,
            self.calculation.right,
            self.calculation.result(),
        );
        match (self.calculation.operation, self.format, self.hidden) {
            (ArithmeticOperation::Multiplication, ExerciseFormat::Phrase, HiddenPart::Left) => {
                format!(
                    "{} égale combien de fois {right} ?",
                    self.repeated_addition()
                )
            }
            (ArithmeticOperation::Multiplication, ExerciseFormat::Phrase, _) => {
                format!("{} égale {left} fois combien ?", self.repeated_addition())
            }
            (ArithmeticOperation::Multiplication, _, HiddenPart::Result) => {
                format!("Combien font {left} fois {right} ?")
            }
            (ArithmeticOperation::Multiplication, _, HiddenPart::Left) => {
                format!("Combien de fois {right} pour faire {result} ?")
            }
            (ArithmeticOperation::Multiplication, _, HiddenPart::Right) => {
                format!("{left} fois combien égale {result} ?")
            }
            (ArithmeticOperation::Addition, _, HiddenPart::Result) => {
                format!("Combien font {left} plus {right} ?")
            }
            (ArithmeticOperation::Subtraction, _, HiddenPart::Result) => {
                format!("Combien font {left} moins {right} ?")
            }
            (ArithmeticOperation::Addition, _, HiddenPart::Right) => {
                format!("Combien faut-il ajouter à {left} pour obtenir {result} ?")
            }
            (ArithmeticOperation::Subtraction, _, HiddenPart::Right) => {
                format!("Combien faut-il enlever à {left} pour obtenir {result} ?")
            }
            (ArithmeticOperation::Addition, _, HiddenPart::Left) => {
                format!("Quel nombre plus {right} égale {result} ?")
            }
            (ArithmeticOperation::Subtraction, _, HiddenPart::Left) => {
                format!("Quel nombre moins {right} égale {result} ?")
            }
        }
    }

    /// Correction shown and read after a mistake, e.g. `7 + 5 = 12`; for a product of a number
    /// from 11 to 19, the decomposition `13 × 7 = 10 × 7 + 3 × 7 = 70 + 21 = 91` (SUB000120).
    pub fn explanation(&self) -> String {
        let c = &self.calculation;
        if self.format == ExerciseFormat::Phrase {
            return format!("{} = {} × {}", self.repeated_addition(), c.left, c.right);
        }

        if let Some((big, small)) = self.decomposable() {
            let (tens, units) = (big / 10 * 10, big % 10);
            // Same order as shown: 13 × 7 = 10 × 7 + 3 × 7, 7 × 13 = 7 × 10 + 7 × 3.
            let (shown, parts) = if c.left == big {
                (
                    format!("{big} × {small}"),
                    format!("{tens} × {small} + {units} × {small}"),
                )
            } else {
                (
                    format!("{small} × {big}"),
                    format!("{small} × {tens} + {small} × {units}"),
                )
            };
            return format!(
                "{shown} = {parts} = {} + {} = {}",
                tens * small,
                units * small,
                c.result()
            );
        }

        if self.reversed {
            format!("{} = {} {} {}", c.result(), c.left, c.symbol(), c.right)
        } else {
            c.to_string()
        }
    }

    /// For a product of a number from 11 to 19 by a number below 10: (that number, the other one).
    fn decomposable(&self) -> Option<(i32, i32)> {
        let c = &self.calculation;
        if c.operation != ArithmeticOperation::Multiplication {
            return None;
        }

        let in_teens = |n: i32| (11..=19).contains(&n);
        let small = |n: i32| (2..=9).contains(&n);
        if in_teens(c.left) && small(c.right) {
            Some((c.left, c.right))
        } else if in_teens(c.right) && small(c.left) {
            Some((c.right, c.left))
        } else {
            None
        }
    }

    /// Help picture after a mistake: the decomposed array (`arrayGrid`) for 13 × 7.
    pub fn help(&self) -> Option<Picture> {
        self.decomposable().map(|(big, small)| Picture::ArrayGrid {
            rows: small.unsigned_abs(),
            columns: big.unsigned_abs(),
            split: Some(10),
        })
    }

    pub fn is_correct(&self, answer: i32) -> bool {
        answer == self.expected()
    }

    /// The exercise as shown to the child. `picture`: rows of objects for the meaning of × (SUB000119).
    pub fn to_exercise(&self, picture: Option<Picture>) -> Exercise {
        let choices = self
            .choices
            .as_ref()
            .map(|c| c.iter().map(|v| Answer::Number(*v)).collect::<Vec<_>>());
        let c = &self.calculation;
        Exercise {
            index: self.index,
            format: self.format,
            question: None,
            statement: self.tokens(),
            picture,
            input: if choices.is_some() {
                Input::Pick
            } else {
                Input::Number { slots: 1 }
            },
            expected: Answer::Number(self.expected()),
            choices,
            spoken: self.spoken_prompt(),
            explanation: self.explanation(),
            help: self.help(),
            key: format!(
                "{:?}:{:?}:{}:{}",
                self.format,
                c.operation,
                c.duplicate_key().1,
                c.duplicate_key().2
            ),
            calculation: Some(self.clone()),
            word: None,
        }
    }
}
