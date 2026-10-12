//! Small integer expressions of the problem templates (SUB000124): `a + b`, `a * b - c`, `(a + b) / 2`,
//! checks such as `a > b`. Parsed when the content is loaded, evaluated by the generator. Natural
//! numbers only: a division must be exact, otherwise the draw is rejected.

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Number(i64),
    Variable(String),
    Binary(Operator, Box<Expression>, Box<Expression>),
}

impl Expression {
    /// Parses `a + b * (c - 2)`, `a > b`, `a != b`. Variables are lowercase words; `×` and `÷`
    /// are accepted for `*` and `/`.
    pub fn parse(text: &str) -> Result<Expression, String> {
        let tokens = tokenize(text)?;
        let mut parser = Parser {
            tokens: &tokens,
            position: 0,
        };
        let expression = parser.comparison()?;
        if parser.position != tokens.len() {
            return Err(format!(
                "Unexpected '{}' in '{text}'.",
                tokens[parser.position]
            ));
        }

        Ok(expression)
    }

    /// Value for the given variables; `None` when a variable is missing or a division is not exact
    /// (or by zero). A comparison is 1 when true, 0 otherwise.
    pub fn evaluate(&self, values: &HashMap<String, i64>) -> Option<i64> {
        match self {
            Expression::Number(n) => Some(*n),
            Expression::Variable(name) => values.get(name).copied(),
            Expression::Binary(operator, left, right) => {
                let (l, r) = (left.evaluate(values)?, right.evaluate(values)?);
                match operator {
                    Operator::Add => l.checked_add(r),
                    Operator::Subtract => l.checked_sub(r),
                    Operator::Multiply => l.checked_mul(r),
                    Operator::Divide => (r != 0 && l % r == 0).then(|| l / r),
                    Operator::Less => Some(i64::from(l < r)),
                    Operator::LessOrEqual => Some(i64::from(l <= r)),
                    Operator::Greater => Some(i64::from(l > r)),
                    Operator::GreaterOrEqual => Some(i64::from(l >= r)),
                    Operator::Equal => Some(i64::from(l == r)),
                    Operator::NotEqual => Some(i64::from(l != r)),
                }
            }
        }
    }

    /// Every variable used, without duplicates.
    pub fn variables(&self) -> Vec<String> {
        let mut found = Vec::new();
        self.collect_variables(&mut found);
        found
    }

    fn collect_variables(&self, found: &mut Vec<String>) {
        match self {
            Expression::Number(_) => {}
            Expression::Variable(name) => {
                if !found.contains(name) {
                    found.push(name.clone());
                }
            }
            Expression::Binary(_, left, right) => {
                left.collect_variables(found);
                right.collect_variables(found);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Lexeme {
    Number(i64),
    Name(String),
    Symbol(&'static str),
}

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lexeme::Number(n) => write!(f, "{n}"),
            Lexeme::Name(name) => write!(f, "{name}"),
            Lexeme::Symbol(symbol) => write!(f, "{symbol}"),
        }
    }
}

fn tokenize(text: &str) -> Result<Vec<Lexeme>, String> {
    let chars: Vec<char> = text.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            let digits: String = chars[start..i].iter().collect();
            tokens.push(Lexeme::Number(
                digits
                    .parse()
                    .map_err(|_| format!("Number too large in '{text}'."))?,
            ));
        } else if c.is_ascii_lowercase() {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_lowercase() || chars[i].is_ascii_digit()) {
                i += 1;
            }
            tokens.push(Lexeme::Name(chars[start..i].iter().collect()));
        } else {
            let next = chars.get(i + 1).copied();
            let (symbol, width) = match (c, next) {
                ('<', Some('=')) => ("<=", 2),
                ('>', Some('=')) => (">=", 2),
                ('=', Some('=')) => ("==", 2),
                ('!', Some('=')) => ("!=", 2),
                ('<', _) => ("<", 1),
                ('>', _) => (">", 1),
                ('+', _) => ("+", 1),
                ('-' | '−', _) => ("-", 1),
                ('*' | '×', _) => ("*", 1),
                ('/' | '÷', _) => ("/", 1),
                ('(', _) => ("(", 1),
                (')', _) => (")", 1),
                _ => return Err(format!("Unexpected character '{c}' in '{text}'.")),
            };
            tokens.push(Lexeme::Symbol(symbol));
            i += width;
        }
    }

    if tokens.is_empty() {
        return Err("Empty expression.".to_owned());
    }

    Ok(tokens)
}

struct Parser<'a> {
    tokens: &'a [Lexeme],
    position: usize,
}

impl Parser<'_> {
    fn peek_symbol(&self) -> Option<&'static str> {
        match self.tokens.get(self.position) {
            Some(Lexeme::Symbol(symbol)) => Some(symbol),
            _ => None,
        }
    }

    fn comparison(&mut self) -> Result<Expression, String> {
        let left = self.sum()?;
        let operator = match self.peek_symbol() {
            Some("<") => Operator::Less,
            Some("<=") => Operator::LessOrEqual,
            Some(">") => Operator::Greater,
            Some(">=") => Operator::GreaterOrEqual,
            Some("==") => Operator::Equal,
            Some("!=") => Operator::NotEqual,
            _ => return Ok(left),
        };
        self.position += 1;
        let right = self.sum()?;
        Ok(Expression::Binary(
            operator,
            Box::new(left),
            Box::new(right),
        ))
    }

    fn sum(&mut self) -> Result<Expression, String> {
        let mut left = self.product()?;
        loop {
            let operator = match self.peek_symbol() {
                Some("+") => Operator::Add,
                Some("-") => Operator::Subtract,
                _ => return Ok(left),
            };
            self.position += 1;
            let right = self.product()?;
            left = Expression::Binary(operator, Box::new(left), Box::new(right));
        }
    }

    fn product(&mut self) -> Result<Expression, String> {
        let mut left = self.atom()?;
        loop {
            let operator = match self.peek_symbol() {
                Some("*") => Operator::Multiply,
                Some("/") => Operator::Divide,
                _ => return Ok(left),
            };
            self.position += 1;
            let right = self.atom()?;
            left = Expression::Binary(operator, Box::new(left), Box::new(right));
        }
    }

    fn atom(&mut self) -> Result<Expression, String> {
        let token = self
            .tokens
            .get(self.position)
            .cloned()
            .ok_or_else(|| "Unexpected end of expression.".to_owned())?;
        self.position += 1;
        match token {
            Lexeme::Number(n) => Ok(Expression::Number(n)),
            Lexeme::Name(name) => Ok(Expression::Variable(name)),
            Lexeme::Symbol("(") => {
                let inner = self.sum()?;
                if self.peek_symbol() != Some(")") {
                    return Err("Missing ')'.".to_owned());
                }
                self.position += 1;
                Ok(inner)
            }
            other => Err(format!("Unexpected '{other}'.")),
        }
    }
}
