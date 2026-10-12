use super::{ArithmeticOperation, Calculation, DuplicateKey};
use crate::skills::{CarryRule, OperandRule, Skill, SkillConstraints, SkillOperation, TrivialCase};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};

/// Every calculation a skill allows (operands in their rules, constraints respected), enumerated
/// once and cached. Sampling from this list makes every generated exercise valid by construction,
/// and makes the "rule too narrow" check exact ([`CalculationSpace::distinct_count`]).
#[derive(Debug)]
pub struct CalculationSpace {
    calculations: Vec<Calculation>,
    distinct_count: usize,
}

impl CalculationSpace {
    pub fn of(skill: &Skill) -> Arc<CalculationSpace> {
        static CACHE: OnceLock<Mutex<HashMap<Skill, Arc<CalculationSpace>>>> = OnceLock::new();
        let cache = CACHE.get_or_init(Default::default);
        if let Some(found) = cache.lock().unwrap_or_else(|e| e.into_inner()).get(skill) {
            return Arc::clone(found);
        }

        // Enumerated outside the lock: a wide rule takes a moment.
        let space = Arc::new(Self::enumerate(skill));
        let mut guard = cache.lock().unwrap_or_else(|e| e.into_inner());
        Arc::clone(guard.entry(skill.clone()).or_insert(space))
    }

    /// Valid calculations, in a stable order.
    pub fn calculations(&self) -> &[Calculation] {
        &self.calculations
    }

    /// Number of different exercises (commuted additions counted once).
    pub fn distinct_count(&self) -> usize {
        self.distinct_count
    }

    /// Whether `calculation` respects every rule of `skill`.
    /// A multiplication may be shown with its factors swapped (`7 × 13` for the rule `13 × 7`).
    pub fn is_valid(skill: &Skill, calculation: &Calculation) -> bool {
        let in_rules = |c: &Calculation| skill.left.allows(c.left) && skill.right.allows(c.right);
        let commuted = calculation.operation == ArithmeticOperation::Multiplication
            && in_rules(&calculation.commuted());
        is_allowed_operation(skill.operation, calculation.operation)
            && (in_rules(calculation) || commuted)
            && respects_constraints(&skill.constraints, calculation)
    }

    fn enumerate(skill: &Skill) -> Self {
        let lefts = skill.left.allowed_values();
        let rights = skill.right.allowed_values();
        let operations: &[ArithmeticOperation] = match skill.operation {
            SkillOperation::Addition => &[ArithmeticOperation::Addition],
            SkillOperation::Subtraction => &[ArithmeticOperation::Subtraction],
            SkillOperation::Mixed => &[
                ArithmeticOperation::Addition,
                ArithmeticOperation::Subtraction,
            ],
            SkillOperation::Multiplication => &[ArithmeticOperation::Multiplication],
            // Not calculations: see `family`.
            _ => &[],
        };

        let mut calculations = Vec::new();
        for &operation in operations {
            for &left in &lefts {
                for &right in &rights {
                    let calculation = Calculation::new(operation, left, right);
                    if respects_constraints(&skill.constraints, &calculation) {
                        calculations.push(calculation);
                    }
                }
            }
        }

        let distinct_count = calculations
            .iter()
            .map(Calculation::duplicate_key)
            .collect::<HashSet<DuplicateKey>>()
            .len();
        Self {
            calculations,
            distinct_count,
        }
    }
}

impl Skill {
    /// Pairs of operands the rule combines (both operations for a mixed skill), without listing
    /// them; 0 for the families that are not calculations.
    pub fn combination_count(&self) -> i64 {
        if !self.operation.is_calculation() {
            return 0;
        }

        let factor = if self.operation == SkillOperation::Mixed {
            2
        } else {
            1
        };
        self.left.allowed_count() * self.right.allowed_count() * factor
    }
}

fn is_allowed_operation(allowed: SkillOperation, operation: ArithmeticOperation) -> bool {
    match allowed {
        SkillOperation::Addition => operation == ArithmeticOperation::Addition,
        SkillOperation::Subtraction => operation == ArithmeticOperation::Subtraction,
        SkillOperation::Mixed => operation != ArithmeticOperation::Multiplication,
        SkillOperation::Multiplication => operation == ArithmeticOperation::Multiplication,
        _ => false,
    }
}

fn respects_constraints(constraints: &SkillConstraints, calculation: &Calculation) -> bool {
    if calculation.left < 0 || calculation.right < 0 {
        return false;
    }

    let result = calculation.result();
    if result < 0
        || constraints.min_result.is_some_and(|min| result < min)
        || constraints.max_result.is_some_and(|max| result > max)
        || constraints
            .exact_result
            .is_some_and(|exact| result != exact)
        || constraints
            .result_multiple_of
            .is_some_and(|m| m > 0 && result % m != 0)
    {
        return false;
    }

    let carry = calculation.has_carry();
    if (constraints.carry == CarryRule::Forbidden && carry)
        || (constraints.carry == CarryRule::Required && !carry)
    {
        return false;
    }

    !constraints
        .exclude_trivial
        .iter()
        .any(|trivial| match trivial {
            TrivialCase::PlusZero => {
                calculation.operation == ArithmeticOperation::Addition
                    && (calculation.left == 0 || calculation.right == 0)
            }
            TrivialCase::MinusZero => {
                calculation.operation == ArithmeticOperation::Subtraction && calculation.right == 0
            }
            TrivialCase::MinusSelf => {
                calculation.operation == ArithmeticOperation::Subtraction
                    && calculation.left == calculation.right
            }
        })
}

impl OperandRule {
    /// Shortcut for the tests and the content: exact values.
    pub fn values(min: i32, max: i32, values: Vec<i32>) -> Self {
        Self {
            values: Some(values),
            ..Self::range(min, max)
        }
    }
}
