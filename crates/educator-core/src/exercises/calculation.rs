use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArithmeticOperation {
    Addition,
    Subtraction,
    Multiplication,
}

impl ArithmeticOperation {
    /// 7 + 5 = 5 + 7 and 3 × 4 = 4 × 3.
    pub fn is_commutative(&self) -> bool {
        matches!(
            self,
            ArithmeticOperation::Addition | ArithmeticOperation::Multiplication
        )
    }
}

/// Key used to avoid duplicates in a series: an addition (or a multiplication) and its commuted form
/// are the same exercise.
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
            ArithmeticOperation::Multiplication => self.left * self.right,
        }
    }

    pub const fn multiplication(left: i32, right: i32) -> Self {
        Self::new(ArithmeticOperation::Multiplication, left, right)
    }

    /// Symbol shown on screen (the true minus sign U+2212 for subtractions, × for multiplications).
    pub fn symbol(&self) -> &'static str {
        match self.operation {
            ArithmeticOperation::Addition => "+",
            ArithmeticOperation::Subtraction => "−",
            ArithmeticOperation::Multiplication => "×",
        }
    }

    /// The same calculation with its operands swapped (only meaningful when commutative).
    pub fn commuted(&self) -> Self {
        Self::new(self.operation, self.right, self.left)
    }

    /// 7 + 5 and 5 + 7 (3 × 4 and 4 × 3) are the same exercise, whatever the format.
    pub fn duplicate_key(&self) -> DuplicateKey {
        if self.operation.is_commutative() {
            (
                self.operation,
                self.left.min(self.right),
                self.left.max(self.right),
            )
        } else {
            (self.operation, self.left, self.right)
        }
    }

    /// Carry (addition) or borrow (subtraction) in at least one column. Never for a multiplication
    /// (the rule does not apply to it).
    pub fn has_carry(&self) -> bool {
        if self.operation == ArithmeticOperation::Multiplication {
            return false;
        }

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
