use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArithmeticOperation {
    Addition,
    Subtraction,
}

/// Key used to avoid duplicates in a series: an addition and its commuted form are the same exercise.
pub type DuplicateKey = (ArithmeticOperation, i32, i32);

/// A calculation `left op right = result`, with natural numbers only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Calculation {
    pub operation: ArithmeticOperation,
    pub left: i32,
    pub right: i32,
}

impl Calculation {
    pub const fn new(operation: ArithmeticOperation, left: i32, right: i32) -> Self {
        Self {
            operation,
            left,
            right,
        }
    }

    pub const fn addition(left: i32, right: i32) -> Self {
        Self::new(ArithmeticOperation::Addition, left, right)
    }

    pub const fn subtraction(left: i32, right: i32) -> Self {
        Self::new(ArithmeticOperation::Subtraction, left, right)
    }

    pub fn result(&self) -> i32 {
        match self.operation {
            ArithmeticOperation::Addition => self.left + self.right,
            ArithmeticOperation::Subtraction => self.left - self.right,
        }
    }

    /// Symbol shown on screen (the true minus sign U+2212 for subtractions).
    pub fn symbol(&self) -> &'static str {
        match self.operation {
            ArithmeticOperation::Addition => "+",
            ArithmeticOperation::Subtraction => "−",
        }
    }

    /// 7 + 5 and 5 + 7 are the same exercise, whatever the format.
    pub fn duplicate_key(&self) -> DuplicateKey {
        match self.operation {
            ArithmeticOperation::Addition => (
                self.operation,
                self.left.min(self.right),
                self.left.max(self.right),
            ),
            ArithmeticOperation::Subtraction => (self.operation, self.left, self.right),
        }
    }

    /// Carry (addition) or borrow (subtraction) in at least one column.
    pub fn has_carry(&self) -> bool {
        let (mut left, mut right, mut carry) = (self.left, self.right, 0);
        while left > 0 || right > 0 {
            let subtraction = self.operation == ArithmeticOperation::Subtraction;
            let top = left % 10 - if subtraction { carry } else { 0 };
            let bottom = right % 10;
            carry = if subtraction {
                i32::from(top < bottom)
            } else {
                i32::from(top + bottom + carry >= 10)
            };

            if carry == 1 {
                return true;
            }

            left /= 10;
            right /= 10;
        }

        false
    }
}

/// Complete equality, e.g. `7 + 5 = 12`.
impl fmt::Display for Calculation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} = {}",
            self.left,
            self.symbol(),
            self.right,
            self.result()
        )
    }
}
