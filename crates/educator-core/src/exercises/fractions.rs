//! First fractions (SUB000123): 1/2, 1/3, 1/4, 1/5, 1/6, 1/8, 1/10, then a/b ≤ 1; pictures, words,
//! comparisons and sums or differences with the same denominator. Any equal fraction is accepted.

use super::family::{answer_choices, Draw, Position};
use super::{Answer, Comparison, Exercise, Fraction, FractionShape, Input, Picture, Token};
use crate::numbers::fraction_words;
use crate::random::SeededRng;
use crate::skills::{AnswerMode, ExerciseFormat, Skill};
use rand::Rng;

pub fn draw(
    skill: &Skill,
    format: ExerciseFormat,
    mode: AnswerMode,
    _position: Position,
    random: &mut SeededRng,
) -> Option<Draw> {
    let denominators = &skill.params.denominators;
    if denominators.is_empty() {
        return None;
    }
    let b = denominators[random.random_range(0..denominators.len())];
    let unit_only = skill.params.unit_fractions_only;
    let a = if unit_only || b < 2 {
        1
    } else {
        random.random_range(1..b)
    };

    let exercise = match format {
        ExerciseFormat::FractionPicture => picture(Fraction::new(a, b), mode, random),
        ExerciseFormat::FractionWords => words(Fraction::new(a, b), mode, random),
        ExerciseFormat::Compare => compare(denominators, b, unit_only, random),
        ExerciseFormat::Result => sum_or_difference(b, mode, random),
        _ => None,
    }?;
    Some(Draw {
        exercise,
        difficulty: 0,
    })
}

fn bare(
    format: ExerciseFormat,
    question: Option<String>,
    statement: Vec<Token>,
    expected: Answer,
    spoken: String,
    explanation: String,
    key: String,
) -> Exercise {
    Exercise {
        index: 0,
        format,
        question,
        statement,
        picture: None,
        input: Input::Fraction,
        expected,
        choices: None,
        spoken,
        explanation,
        help: None,
        key,
        calculation: None,
        word: None,
    }
}

/// Fraction input, or four fraction proposals in QCM mode.
fn fraction_input(
    exercise: &mut Exercise,
    mistakes: Vec<Fraction>,
    mode: AnswerMode,
    random: &mut SeededRng,
) -> Option<()> {
    if mode == AnswerMode::Choice {
        let expected = exercise.expected.clone();
        let mut candidates: Vec<Answer> = mistakes
            .into_iter()
            .filter(|f| f.denominator > 0 && f.numerator > 0)
            .map(Answer::Fraction)
            .collect();
        // Neighbours to complete.
        if let Answer::Fraction(f) = expected {
            for delta in 1..=3 {
                candidates.push(Answer::Fraction(Fraction::new(
                    f.numerator + delta,
                    f.denominator,
                )));
                if f.numerator > delta {
                    candidates.push(Answer::Fraction(Fraction::new(
                        f.numerator - delta,
                        f.denominator,
                    )));
                }
            }
        }
        exercise.choices = Some(answer_choices(&expected, candidates, 4, random)?);
        exercise.input = Input::Pick;
    }

    Some(())
}

fn picture(f: Fraction, mode: AnswerMode, random: &mut SeededRng) -> Option<Exercise> {
    let shape = if random.random_bool(0.5) {
        FractionShape::Bar
    } else {
        FractionShape::Disc
    };
    let (what, of) = match shape {
        FractionShape::Bar => ("La bande", "de la bande"),
        FractionShape::Disc => ("Le disque", "du disque"),
    };
    let question = format!("Quelle fraction {of} est coloriée ?");
    let parts = if f.numerator > 1 {
        "parts sont coloriées"
    } else {
        "part est coloriée"
    };
    let mut exercise = bare(
        ExerciseFormat::FractionPicture,
        Some(question.clone()),
        vec![Token::Slot(0)],
        Answer::Fraction(f),
        question,
        format!(
            "{what} est partagé{} en {} parts égales et {} {parts} : {} ({}).",
            if shape == FractionShape::Bar { "e" } else { "" },
            f.denominator,
            f.numerator,
            f,
            f.words()
        ),
        format!("picture:{shape:?}:{f}"),
    );
    exercise.picture = Some(Picture::FractionShape {
        shape,
        parts: f.denominator,
        colored: f.numerator,
    });
    // Inverted, the colored parts over the others, the uncolored part.
    let mistakes = vec![
        Fraction::new(f.denominator, f.numerator),
        Fraction::new(f.numerator, f.denominator - f.numerator),
        Fraction::new(f.denominator - f.numerator, f.denominator),
    ];
    fraction_input(&mut exercise, mistakes, mode, random)?;
    Some(exercise)
}

/// « trois quatrièmes » for 3/4: the naive ordinal (credible mistake for 2, 3 and 4).
fn naive_words(f: Fraction) -> Option<String> {
    let ordinal = match f.denominator {
        2 => "deuxième",
        3 => "troisième",
        4 => "quatrième",
        _ => return None,
    };
    let count = crate::speech::number_to_words(f.numerator);
    Some(format!(
        "{count} {ordinal}{}",
        if f.numerator > 1 { "s" } else { "" }
    ))
}

fn words(f: Fraction, mode: AnswerMode, random: &mut SeededRng) -> Option<Exercise> {
    let words = f.words();
    if random.random_bool(0.5) {
        // Words → digits.
        let question = format!("Écris en chiffres : {words}.");
        let mut exercise = bare(
            ExerciseFormat::FractionWords,
            Some("Écris cette fraction en chiffres.".to_owned()),
            vec![
                Token::Text(words.clone()),
                Token::Sign("→".to_owned()),
                Token::Slot(0),
            ],
            Answer::Fraction(f),
            question,
            format!("« {words} » s'écrit {f}."),
            format!("words-to-digits:{f}"),
        );
        let mistakes = vec![
            Fraction::new(f.denominator, f.numerator),
            Fraction::new(f.numerator, f.denominator + 1),
        ];
        fraction_input(&mut exercise, mistakes, mode, random)?;
        return Some(exercise);
    }

    // Digits → words (always a pick).
    let mut candidates: Vec<Answer> = Vec::new();
    if f.numerator != f.denominator && f.numerator > 1 {
        candidates.push(Answer::Text(fraction_words(f.denominator, f.numerator)));
    }
    if let Some(naive) = naive_words(f) {
        candidates.push(Answer::Text(naive));
    }
    for other in [
        f.denominator + 1,
        f.denominator.saturating_sub(1),
        f.denominator * 2,
    ] {
        if other >= 2 && other != f.denominator && other <= 10 {
            candidates.push(Answer::Text(fraction_words(f.numerator, other)));
        }
    }
    candidates.push(Answer::Text(fraction_words(f.numerator + 1, f.denominator)));
    let expected = Answer::Text(words.clone());
    let choices = answer_choices(&expected, candidates, 3, random)?;
    let mut exercise = bare(
        ExerciseFormat::FractionWords,
        Some("Comment lit-on cette fraction ?".to_owned()),
        vec![Token::Fraction(f)],
        expected,
        "Comment lit-on cette fraction ?".to_owned(),
        format!("{f} se lit « {words} »."),
        format!("digits-to-words:{f}"),
    );
    exercise.input = Input::Pick;
    exercise.choices = Some(choices);
    Some(exercise)
}

fn compare(
    denominators: &[u32],
    b: u32,
    unit_only: bool,
    random: &mut SeededRng,
) -> Option<Exercise> {
    let same_denominator = !unit_only && b > 2 && random.random_bool(0.5);
    let (left, right) = if same_denominator {
        let x = random.random_range(1..b);
        let y = if random.random_range(0..8) == 0 {
            x
        } else {
            random.random_range(1..b)
        };
        (Fraction::new(x, b), Fraction::new(y, b))
    } else {
        let d = denominators[random.random_range(0..denominators.len())];
        if d == b {
            return None;
        }
        (Fraction::new(1, b), Fraction::new(1, d))
    };

    let expected = left.compare(&right);
    let sign = expected.symbol();
    let explanation = if left.denominator == right.denominator {
        let words = match expected {
            Comparison::Less => "moins",
            Comparison::Greater => "plus",
            Comparison::Equal => "autant",
        };
        format!(
            "{left} {sign} {right} : même dénominateur, {} part{} c'est {words} que {} part{}.",
            left.numerator,
            if left.numerator > 1 { "s" } else { "" },
            right.numerator,
            if right.numerator > 1 { "s" } else { "" },
        )
    } else {
        format!(
            "{left} {sign} {right} : quand on partage en plus de parts, chaque part est plus petite."
        )
    };
    let mut exercise = bare(
        ExerciseFormat::Compare,
        Some("Écris le bon signe : <, > ou =.".to_owned()),
        vec![
            Token::Fraction(left),
            Token::Slot(0),
            Token::Fraction(right),
        ],
        Answer::Comparison(expected),
        format!("Compare {} et {}.", left.words(), right.words()),
        explanation,
        format!("compare:{left}:{right}"),
    );
    exercise.input = Input::Comparison;
    Some(exercise)
}

fn sum_or_difference(b: u32, mode: AnswerMode, random: &mut SeededRng) -> Option<Exercise> {
    if b < 3 {
        return None;
    }

    let addition = random.random_bool(0.6);
    let (x, y, result) = if addition {
        let x = random.random_range(1..b);
        let y = random.random_range(1..=b - x);
        (x, y, x + y)
    } else {
        let x = random.random_range(2..=b);
        let y = random.random_range(1..x);
        (x, y, x - y)
    };
    let (left, right, total) = (
        Fraction::new(x, b),
        Fraction::new(y, b),
        Fraction::new(result, b),
    );
    let (sign, word) = if addition {
        ("+", "plus")
    } else {
        ("−", "moins")
    };
    let name = |n: u32| {
        let words = fraction_words(n, b);
        words
            .split_once(' ')
            .map(|(_, w)| w.to_owned())
            .unwrap_or(words)
    };
    let explanation = format!(
        "{left} {sign} {right} = {total} : {x} {} {sign} {y} {} = {result} {}.",
        name(x),
        name(y),
        name(result)
    );
    let mut exercise = bare(
        ExerciseFormat::Result,
        None,
        vec![
            Token::Fraction(left),
            Token::Sign(sign.to_owned()),
            Token::Fraction(right),
            Token::Sign("=".to_owned()),
            Token::Slot(0),
        ],
        Answer::Fraction(total),
        format!("Combien font {} {word} {} ?", left.words(), right.words()),
        explanation,
        format!("{sign}:{left}:{right}"),
    );
    // Denominators added too (2/8 + 3/8 = 5/16), the other operation.
    let other = if addition { x.abs_diff(y) } else { x + y };
    let mistakes = vec![
        Fraction::new(result, if addition { 2 * b } else { b + b }),
        Fraction::new(other, b),
        Fraction::new(result + 1, b),
    ];
    fraction_input(&mut exercise, mistakes, mode, random)?;
    Some(exercise)
}
