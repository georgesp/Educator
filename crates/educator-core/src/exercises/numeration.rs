//! Numeration (SUB000118, SUB000121, SUB000122): hundreds, tens and units, reading and writing the
//! numbers, comparing, ordering, framing, the graduated line, even and odd numbers.

use super::family::{answer_choices, number_choices, pick_value, Draw, Position};
use super::{Answer, Comparison, Exercise, Input, Picture, Token};
use crate::numbers::{misspellings, spell, Spelling, MAX_SPELLED};
use crate::random::SeededRng;
use crate::skills::{AnswerMode, ExerciseFormat, Skill};
use rand::seq::{IndexedRandom, SliceRandom};
use rand::Rng;

pub fn draw(
    skill: &Skill,
    format: ExerciseFormat,
    mode: AnswerMode,
    _position: Position,
    random: &mut SeededRng,
) -> Option<Draw> {
    let values = skill
        .params
        .numbers
        .as_ref()
        .map(|rule| rule.allowed_values())
        .unwrap_or_default();
    let n = pick_value(&values, random)?;
    let exercise = match format {
        ExerciseFormat::Recompose => recompose(n, mode, random),
        ExerciseFormat::Decompose => decompose(n, mode, random),
        ExerciseFormat::PlaceValue => place_value(n, mode, random),
        ExerciseFormat::WriteNumber => write_number(n, mode, random),
        ExerciseFormat::NumberDictation => number_dictation(n, mode, random),
        ExerciseFormat::ReadNumber => read_number(&values, n, random),
        ExerciseFormat::Compare => compare(&values, n, random),
        ExerciseFormat::Order => order(&values, n, random),
        ExerciseFormat::Frame => frame(n, mode, random),
        ExerciseFormat::NumberLinePoint => {
            number_line(&values, &skill.params.line_steps, mode, random)
        }
        ExerciseFormat::YesNo => parity_question(n, random),
        ExerciseFormat::MultiChoice => parity_pick(&values, random),
        _ => None,
    }?;
    Some(Draw {
        exercise,
        difficulty: 0,
    })
}

fn digits(n: i32) -> (i32, i32, i32) {
    (n / 100, n / 10 % 10, n % 10)
}

/// A bare exercise: the index is set by the series.
#[allow(clippy::too_many_arguments)]
fn exercise(
    format: ExerciseFormat,
    question: Option<String>,
    statement: Vec<Token>,
    picture: Option<Picture>,
    input: Input,
    expected: Answer,
    choices: Option<Vec<Answer>>,
    spoken: String,
    explanation: String,
    key: String,
) -> Exercise {
    Exercise {
        index: 0,
        format,
        question,
        statement,
        picture,
        input,
        expected,
        choices,
        spoken,
        explanation,
        help: None,
        key,
        calculation: None,
        word: None,
    }
}

/// Number input, or four proposals in QCM mode.
fn number_input(
    expected: i32,
    mistakes: &[i32],
    mode: AnswerMode,
    random: &mut SeededRng,
) -> (Input, Option<Vec<Answer>>) {
    match mode {
        AnswerMode::Keyboard => (Input::Number { slots: 1 }, None),
        AnswerMode::Choice => (
            Input::Pick,
            Some(number_choices(expected, mistakes, random)),
        ),
    }
}

/// The mistakes of a two-digit number kept below 100, completed with close numbers below 100
/// (CP: no proposal with three digits, SUB012205).
fn below_hundred(expected: i32, mistakes: &[i32]) -> Vec<i32> {
    let mut kept: Vec<i32> = mistakes.iter().copied().filter(|m| *m < 100).collect();
    kept.extend(
        [-1, 1, -10, 10, -2, 2]
            .into_iter()
            .map(|d| expected + d)
            .filter(|m| (0..100).contains(m)),
    );
    kept
}

fn plural(count: i32, singular: &str) -> String {
    if count > 1 {
        format!("{count} {singular}s")
    } else {
        format!("{count} {singular}")
    }
}

/// « 3 centaines, 4 dizaines et 7 unités » (the parts that are not zero, in the given order).
fn parts_text(parts: &[(i32, &str)]) -> String {
    let named: Vec<String> = parts
        .iter()
        .filter(|(count, _)| *count > 0)
        .map(|(count, name)| plural(*count, name))
        .collect();
    match named.as_slice() {
        [] => "0 unité".to_owned(),
        [only] => only.clone(),
        [rest @ .., last] => format!("{} et {last}", rest.join(", ")),
    }
}

fn recompose(n: i32, mode: AnswerMode, random: &mut SeededRng) -> Option<Exercise> {
    let (h, t, u) = digits(n);
    if n >= 1000 {
        return None;
    }

    let variant = random.random_range(0..3);
    let mut parts = vec![(h, "centaine"), (t, "dizaine"), (u, "unité")];
    let (question, picture, key, written_order) = match variant {
        0 => (
            "Quel nombre est représenté ?".to_owned(),
            Some(Picture::BaseTen {
                hundreds: h as u32,
                tens: t as u32,
                units: u as u32,
            }),
            format!("recompose:picture:{n}"),
            n,
        ),
        1 => (
            format!("Quel nombre fait {} ?", parts_text(&parts)),
            None,
            format!("recompose:text:{n}"),
            n,
        ),
        _ => {
            // Parts in another order: the child must place each digit at its rank.
            parts.shuffle(random);
            if parts[0].1 == "centaine" && parts[1].1 == "dizaine" {
                parts.swap(0, 2);
            }
            let written: i32 = parts
                .iter()
                .filter(|(count, _)| *count > 0)
                .fold(0, |acc, (count, _)| acc * 10 + count);
            (
                format!("Quel nombre fait {} ?", parts_text(&parts)),
                None,
                format!("recompose:shuffled:{n}"),
                written,
            )
        }
    };

    let mistakes = [
        written_order,
        match (h, t) {
            // Two digits (CP): the digits swapped (34 → 43), never a hundred too many.
            (0, _) if u > 0 && u != t => u * 10 + t,
            (0, _) => n + 1,
            (_, 0) => h * 10 + u,
            _ => n + 100,
        },
        n + 10,
        n - 10,
    ];
    let mistakes = if n < 100 {
        below_hundred(n, &mistakes)
    } else {
        mistakes.to_vec()
    };
    let (input, choices) = number_input(n, &mistakes, mode, random);
    let explanation = format!(
        "{}, c'est {n}.",
        capitalize(&parts_text(&[
            (h, "centaine"),
            (t, "dizaine"),
            (u, "unité")
        ]))
    );
    Some(exercise(
        ExerciseFormat::Recompose,
        Some(question.clone()),
        vec![Token::Slot(0)],
        picture,
        input,
        Answer::Number(n),
        choices,
        question,
        explanation,
        key,
    ))
}

fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn decompose(n: i32, mode: AnswerMode, random: &mut SeededRng) -> Option<Exercise> {
    let (h, t, u) = digits(n);
    let parts: Vec<i32> = [h * 100, t * 10, u]
        .into_iter()
        .filter(|p| *p > 0)
        .collect();
    if parts.len() < 2 || n >= 1000 {
        return None;
    }

    // The hidden number: one of the parts, or the number itself.
    let hidden = random.random_range(0..=parts.len());
    let mut statement = Vec::new();
    statement.push(if hidden == parts.len() {
        Token::Slot(0)
    } else {
        Token::Number(n)
    });
    statement.push(Token::Sign("=".to_owned()));
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            statement.push(Token::Sign("+".to_owned()));
        }
        statement.push(if i == hidden {
            Token::Slot(0)
        } else {
            Token::Number(*part)
        });
    }

    let expected = if hidden == parts.len() {
        n
    } else {
        parts[hidden]
    };
    // The digit instead of its value (4 for 40), ten times too much or too little.
    let digit = expected
        .to_string()
        .chars()
        .next()
        .map_or(0, |c| c.to_digit(10).map_or(0, |d| d as i32));
    let mistakes = if n < 100 {
        // Two digits (CP): the digit, the digits swapped, one ten or one unit off.
        let swapped = expected % 10 * 10 + expected / 10;
        below_hundred(expected, &[digit, swapped, expected + 10, expected + 1])
    } else {
        vec![digit, expected * 10, expected / 10, expected + 1]
    };
    let (input, choices) = number_input(expected, &mistakes, mode, random);
    let written = parts
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" + ");
    Some(exercise(
        ExerciseFormat::Decompose,
        None,
        statement.clone(),
        None,
        input,
        Answer::Number(expected),
        choices,
        spoken_statement(&statement),
        format!("{n} = {written}"),
        format!("decompose:{n}:{hidden}"),
    ))
}

/// The statement read aloud: `347 = 300 + ? + 7` (the speech layer reads the signs and `?`).
fn spoken_statement(statement: &[Token]) -> String {
    statement
        .iter()
        .map(|t| match t {
            Token::Number(n) => n.to_string(),
            Token::Sign(s) | Token::Text(s) => s.clone(),
            Token::Fraction(f) => f.words(),
            Token::Slot(_) => "?".to_owned(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn place_value(n: i32, mode: AnswerMode, random: &mut SeededRng) -> Option<Exercise> {
    let (h, t, u) = digits(n);
    if !(10..1000).contains(&n) {
        return None;
    }

    // Below 100 (CP, SUB012205): no hundreds; « combien de dizaines » is the digit of the tens.
    let variant = if n < 100 {
        [0, 1, 3][random.random_range(0..3)]
    } else {
        random.random_range(0..4)
    };
    let (question, expected, mistakes, explanation) = match variant {
        0 => (
            format!("Quel est le chiffre des unités de {n} ?"),
            u,
            vec![t, h, n % 100],
            format!("Dans {n}, le chiffre des unités est {u}."),
        ),
        1 => (
            format!("Quel est le chiffre des dizaines de {n} ?"),
            t,
            vec![n / 10, u, h],
            format!("Dans {n}, le chiffre des dizaines est {t}."),
        ),
        2 => (
            format!("Quel est le chiffre des centaines de {n} ?"),
            h,
            vec![t, u, h * 100],
            format!("Dans {n}, le chiffre des centaines est {h}."),
        ),
        _ => (
            format!("Combien y a-t-il de dizaines dans {n} ?"),
            n / 10,
            vec![t, h, n / 100 * 10],
            format!(
                "{n} = {} : il y a {} dans {n}.",
                parts_text(&[(n / 10, "dizaine"), (u, "unité")]),
                plural(n / 10, "dizaine")
            ),
        ),
    };
    // Two digits: the other digit, the number itself, the digit taken as tens (52 → 50).
    let mistakes = match (n < 100, variant) {
        (true, 0) => below_hundred(expected, &[t, n, u * 10]),
        (true, _) => below_hundred(expected, &[u, n, t * 10]),
        _ => mistakes,
    };
    let (input, choices) = number_input(expected, &mistakes, mode, random);
    Some(exercise(
        ExerciseFormat::PlaceValue,
        Some(question.clone()),
        vec![Token::Slot(0)],
        None,
        input,
        Answer::Number(expected),
        choices,
        question,
        explanation,
        format!("place:{variant}:{n}"),
    ))
}

/// Typical mistakes when writing a number heard or read in words: a zero forgotten or added
/// (608 → 68, 6008), digits swapped (680).
fn writing_mistakes(n: i32) -> Vec<i32> {
    let (h, t, u) = digits(n);
    let mut mistakes = Vec::new();
    if n >= 100 && t == 0 {
        mistakes.push(h * 10 + u);
        mistakes.push(h * 1000 + u);
    }
    if n >= 100 && u == 0 && t > 0 {
        mistakes.push(h * 10 + t);
    }
    if n >= 10 && t != u {
        mistakes.push(h * 100 + u * 10 + t);
    }
    if n >= 100 {
        mistakes.push(h * 1000 + t * 10 + u);
    }
    if h == 0 && (t == 7 || t == 9) {
        // « soixante-douze » heard as 60 and 12: 612; « quatre-vingt-quinze »: 815.
        mistakes.push((t - 1) * 100 + 10 + u);
    }
    mistakes.push(n + 10);
    mistakes
}

fn write_number(n: i32, mode: AnswerMode, random: &mut SeededRng) -> Option<Exercise> {
    if n > MAX_SPELLED {
        return None;
    }

    let words = spell(n, Spelling::Traditional);
    let (input, choices) = number_input(n, &writing_mistakes(n), mode, random);
    Some(exercise(
        ExerciseFormat::WriteNumber,
        Some("Écris ce nombre en chiffres.".to_owned()),
        vec![
            Token::Text(words.clone()),
            Token::Sign("→".to_owned()),
            Token::Slot(0),
        ],
        None,
        input,
        Answer::Number(n),
        choices,
        format!("Écris en chiffres le nombre {words}."),
        format!("{words} s'écrit {n}."),
        format!("write:{n}"),
    ))
}

/// « Écoute le nombre et écris-le en chiffres » (SUB012205): the number is never written, it is
/// heard on click of the loudspeaker (nothing is read by itself, decision of 2026-10-15).
fn number_dictation(n: i32, mode: AnswerMode, random: &mut SeededRng) -> Option<Exercise> {
    if n > MAX_SPELLED {
        return None;
    }

    let words = spell(n, Spelling::Traditional);
    // Below 100 (CP): no proposal of three digits but the heard mistake (612 for « soixante-douze »).
    let mistakes = if n < 100 {
        let (t, u) = (n / 10, n % 10);
        let heard = (t == 7 || t == 9).then(|| (t - 1) * 100 + 10 + u);
        heard
            .into_iter()
            .chain(below_hundred(n, &writing_mistakes(n)))
            .collect()
    } else {
        writing_mistakes(n)
    };
    let (input, choices) = number_input(n, &mistakes, mode, random);
    Some(exercise(
        ExerciseFormat::NumberDictation,
        Some("Écoute le nombre et écris-le en chiffres.".to_owned()),
        vec![Token::Slot(0)],
        None,
        input,
        Answer::Number(n),
        choices,
        format!("Écris en chiffres le nombre : {words}."),
        format!("{words} s'écrit {n}."),
        format!("dictation:{n}"),
    ))
}

fn read_number(values: &[i32], n: i32, random: &mut SeededRng) -> Option<Exercise> {
    if n > MAX_SPELLED {
        return None;
    }

    // The tricky numbers (71, 80, 81, 200, 280…) are drawn more often: retry once or twice.
    let mut n = n;
    for _ in 0..2 {
        if misspellings(n).len() >= 2 {
            break;
        }
        n = pick_value(values, random)?;
    }

    let mut candidates: Vec<Answer> = misspellings(n).into_iter().map(Answer::Text).collect();
    candidates.truncate(3);
    candidates.extend(
        writing_mistakes(n)
            .into_iter()
            .filter(|m| (0..=MAX_SPELLED).contains(m))
            .map(Answer::Words),
    );
    let expected = Answer::Words(n);
    let choices = answer_choices(&expected, candidates, 3, random)?;
    let words = spell(n, Spelling::Traditional);
    Some(exercise(
        ExerciseFormat::ReadNumber,
        Some("Choisis la bonne écriture de ce nombre.".to_owned()),
        vec![Token::Number(n)],
        None,
        Input::Pick,
        expected,
        Some(choices),
        format!("Choisis la bonne écriture du nombre {n}."),
        format!("{n} s'écrit « {words} »."),
        format!("read:{n}"),
    ))
}

/// Another number close to `n`, so that the comparison is decided at the right rank: same digits in
/// another order (438 / 483), one digit changed, or rarely (1 in 10) the same number.
fn close_number(values: &[i32], n: i32, random: &mut SeededRng) -> Option<i32> {
    let (min, max) = (*values.first()?, *values.last()?);
    let allowed = |other: i32| other >= min && other <= max && values.binary_search(&other).is_ok();
    if random.random_range(0..10) == 0 {
        return allowed(n).then_some(n);
    }

    let (h, t, u) = digits(n);
    // Ranks that exist in the numbers worked on (no hundreds up to 100, CP).
    let ranks: Vec<i32> = [1, 10, 100]
        .into_iter()
        .filter(|rank| *rank < max.max(10))
        .collect();
    // Tried until another number fits: a draw that gives `n` again or leaves the numbers worked on
    // must not make the equality more frequent.
    for _ in 0..20 {
        let other = if random.random_range(0..6) < 4 {
            let mut d = [h, t, u];
            let start = usize::from(n < 100);
            d[start..].shuffle(random);
            d[0] * 100 + d[1] * 10 + d[2]
        } else {
            let rank = *ranks.choose(random)?;
            let delta = random.random_range(1..=4) * rank;
            if random.random_bool(0.5) {
                n + delta
            } else {
                n - delta
            }
        };
        if other != n && allowed(other) {
            return Some(other);
        }
    }
    None
}

/// Why `a` compares so to `b`, at the first rank that differs.
fn comparison_reason(a: i32, b: i32) -> String {
    let sign = Comparison::of(a, b).symbol();
    if a == b {
        return format!("{a} = {b} : c'est le même nombre.");
    }

    if a.to_string().len() != b.to_string().len() {
        let (fewer, more) = if a < b { (a, b) } else { (b, a) };
        return format!("{a} {sign} {b} : {fewer} a moins de chiffres que {more}.");
    }

    let ranks = [(100, "centaines"), (10, "dizaines"), (1, "unités")];
    let mut same = Vec::new();
    for (rank, name) in ranks {
        let (da, db) = (a / rank % 10, b / rank % 10);
        if a < rank && b < rank {
            continue;
        }
        if da != db {
            let words = if da < db { "moins" } else { "plus" };
            let prefix = if same.is_empty() {
                String::new()
            } else {
                format!("mêmes {} ; ", same.join(" et "))
            };
            let singular = name.trim_end_matches('s');
            return format!(
                "{a} {sign} {b} : {prefix}{}, c'est {words} que {}.",
                plural(da, singular),
                plural(db, singular)
            );
        }
        same.push(name);
    }

    format!("{a} {sign} {b}")
}

fn compare(values: &[i32], n: i32, random: &mut SeededRng) -> Option<Exercise> {
    let other = close_number(values, n, random)?;
    let (a, b) = if random.random_bool(0.5) {
        (n, other)
    } else {
        (other, n)
    };
    let expected = Comparison::of(a, b);
    let statement = vec![Token::Number(a), Token::Slot(0), Token::Number(b)];
    Some(exercise(
        ExerciseFormat::Compare,
        Some("Écris le bon signe : <, > ou =.".to_owned()),
        statement,
        None,
        Input::Comparison,
        Answer::Comparison(expected),
        None,
        format!("Compare {a} et {b}."),
        comparison_reason(a, b),
        format!("compare:{a}:{b}"),
    ))
}

fn order(values: &[i32], n: i32, random: &mut SeededRng) -> Option<Exercise> {
    let count = random.random_range(3..=5);
    let mut items = vec![n];
    for _ in 0..40 {
        if items.len() == count {
            break;
        }
        if let Some(other) = close_number(values, n, random) {
            if !items.contains(&other) {
                items.push(other);
            }
        }
    }
    if items.len() < 3 {
        return None;
    }

    let ascending = random.random_range(0..4) != 0;
    let mut sorted = items.clone();
    sorted.sort_unstable();
    if !ascending {
        sorted.reverse();
    }
    // Shown in a mixed order, never already ranked.
    for _ in 0..10 {
        items.shuffle(random);
        if items != sorted {
            break;
        }
    }
    if items == sorted {
        items.reverse();
    }

    let (question, sign) = if ascending {
        ("Range ces nombres du plus petit au plus grand.", "<")
    } else {
        ("Range ces nombres du plus grand au plus petit.", ">")
    };
    let explanation = sorted
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(&format!(" {sign} "));
    let mut key_items = sorted.clone();
    key_items.sort_unstable();
    Some(exercise(
        ExerciseFormat::Order,
        Some(question.to_owned()),
        Vec::new(),
        None,
        Input::Order { items, ascending },
        Answer::Sequence(sorted),
        None,
        question.to_owned(),
        explanation,
        format!("order:{ascending}:{key_items:?}"),
    ))
}

fn frame(n: i32, mode: AnswerMode, random: &mut SeededRng) -> Option<Exercise> {
    let by_hundreds = n >= 100 && random.random_bool(0.5);
    let unit = if by_hundreds { 100 } else { 10 };
    if n % unit == 0 || n > 1000 {
        return None;
    }

    let lower = n / unit * unit;
    let upper = lower + unit;
    let expected = Answer::Sequence(vec![lower, upper]);
    let what = if by_hundreds {
        "deux centaines"
    } else {
        "deux dizaines"
    };
    let question = format!("Encadre {n} entre {what}.");
    let (input, choices) = match mode {
        AnswerMode::Keyboard => (Input::Number { slots: 2 }, None),
        AnswerMode::Choice => {
            // The other unit, one step too low or too high.
            let other_unit = if by_hundreds { 10 } else { 100 };
            let other_lower = n / other_unit * other_unit;
            let candidates = vec![
                Answer::Sequence(vec![other_lower, other_lower + other_unit]),
                Answer::Sequence(vec![lower - unit, lower]),
                Answer::Sequence(vec![upper, upper + unit]),
                Answer::Sequence(vec![lower, upper + unit]),
            ]
            .into_iter()
            .filter(|a| matches!(a, Answer::Sequence(v) if v.iter().all(|x| *x >= 0)))
            .collect();
            (
                Input::Pick,
                Some(answer_choices(&expected, candidates, 4, random)?),
            )
        }
    };
    let less = || Token::Sign("<".to_owned());
    Some(exercise(
        ExerciseFormat::Frame,
        Some(question.clone()),
        vec![
            Token::Slot(0),
            less(),
            Token::Number(n),
            less(),
            Token::Slot(1),
        ],
        None,
        input,
        expected,
        choices,
        question,
        format!("{lower} < {n} < {upper}"),
        format!("frame:{unit}:{n}"),
    ))
}

/// Graduations of a number line: 11 graduations (10 intervals), numbers at both ends.
const LINE_TICKS: u32 = 11;

/// Steps of the lines when the skill does not choose them.
const LINE_STEPS: [i32; 3] = [1, 10, 100];

fn number_line(
    values: &[i32],
    steps: &[i32],
    mode: AnswerMode,
    random: &mut SeededRng,
) -> Option<Exercise> {
    let (min, max) = (*values.first()?, *values.last()?);
    let steps = if steps.is_empty() {
        &LINE_STEPS[..]
    } else {
        steps
    };
    let step = *steps.choose(random)?;
    let span = step * (LINE_TICKS as i32 - 1);
    // A start that is a round number for the step: 340 for a line of 1 in 1, 300 for 10 in 10, 0 for 100.
    let starts: Vec<i32> = (0..=max)
        .step_by(span as usize)
        .filter(|s| *s >= min - span && s + span <= max.max(span))
        .collect();
    let start = pick_value(&starts, random)?;
    let k = random.random_range(1..LINE_TICKS - 1);
    let value = start + step * k as i32;
    if value < min || value > max {
        return None;
    }

    let place = random.random_bool(0.5);
    let line = |arrow: Option<u32>| Picture::NumberLine {
        start,
        step,
        ticks: LINE_TICKS,
        labeled: vec![0, LINE_TICKS - 1],
        arrow,
    };
    let explanation = format!(
        "La droite est graduée de {step} en {step} à partir de {start} : la flèche montre {value}."
    );
    if place {
        let explanation = format!(
            "La droite est graduée de {step} en {step} à partir de {start} : {value} est {} après {start}.",
            if k == 1 {
                "à la 1re graduation".to_owned()
            } else {
                format!("à la {k}e graduation")
            }
        );
        let question = format!("Place {value} sur la droite graduée.");
        return Some(exercise(
            ExerciseFormat::NumberLinePoint,
            Some(question.clone()),
            Vec::new(),
            Some(line(None)),
            Input::NumberLine,
            Answer::Number(value),
            None,
            question,
            explanation,
            format!("line:place:{start}:{step}:{k}"),
        ));
    }

    // Wrong step, one graduation off, the number of graduations.
    let mistakes = [
        start + k as i32 * if step == 1 { 10 } else { 1 },
        value + step,
        value - step,
        start + (k as i32 + 1) * step,
    ];
    // Within the numbers worked on: never 130 on a line of the CP.
    let mistakes: Vec<i32> = mistakes.into_iter().filter(|m| *m <= max).collect();
    let (input, choices) = number_input(value, &mistakes, mode, random);
    let question = "Quel nombre la flèche montre-t-elle ?".to_owned();
    Some(exercise(
        ExerciseFormat::NumberLinePoint,
        Some(question.clone()),
        vec![Token::Slot(0)],
        Some(line(Some(k))),
        input,
        Answer::Number(value),
        choices,
        question,
        explanation,
        format!("line:read:{start}:{step}:{k}"),
    ))
}

fn parity_reason(n: i32) -> String {
    let unit = n % 10;
    if n % 2 == 0 {
        format!("{n} se termine par {unit} : c'est un nombre pair.")
    } else {
        format!("{n} se termine par {unit} : c'est un nombre impair.")
    }
}

fn parity_question(n: i32, random: &mut SeededRng) -> Option<Exercise> {
    let ask_even = random.random_bool(0.6);
    let word = if ask_even { "pair" } else { "impair" };
    let expected = (n % 2 == 0) == ask_even;
    let question = format!("{n} est-il un nombre {word} ?");
    Some(exercise(
        ExerciseFormat::YesNo,
        Some(question.clone()),
        Vec::new(),
        None,
        Input::YesNo,
        Answer::YesNo(expected),
        None,
        question,
        parity_reason(n),
        format!("parity:{word}:{n}"),
    ))
}

fn parity_pick(values: &[i32], random: &mut SeededRng) -> Option<Exercise> {
    let (min, max) = (*values.first()?, *values.last()?);
    let ask_even = random.random_bool(0.7);
    let interval = random.random_bool(0.5) && max - min >= 10;
    let items: Vec<i32> = if interval {
        // « les nombres pairs entre 767 et 774 »
        let first = random.random_range(min..=max - 7);
        (first..first + 8).collect()
    } else {
        let mut items: Vec<i32> = Vec::new();
        for _ in 0..60 {
            if items.len() == 6 {
                break;
            }
            let candidate = pick_value(values, random)?;
            if !items.contains(&candidate) {
                items.push(candidate);
            }
        }
        items
    };

    let right: Vec<i32> = items
        .iter()
        .copied()
        .filter(|n| (n % 2 == 0) == ask_even)
        .collect();
    if right.is_empty() || right.len() == items.len() || items.len() < 6 {
        return None;
    }

    let word = if ask_even { "pairs" } else { "impairs" };
    let question = if interval {
        format!(
            "Clique sur tous les nombres {word} entre {} et {}.",
            items[0],
            items[items.len() - 1]
        )
    } else {
        format!("Clique sur tous les nombres {word}.")
    };
    let endings = if ask_even {
        "0, 2, 4, 6 ou 8"
    } else {
        "1, 3, 5, 7 ou 9"
    };
    let explanation = format!(
        "Les nombres {word} sont {} : ils se terminent par {endings}.",
        super::answer::join_numbers(&right)
    );
    let mut key_items = items.clone();
    key_items.sort_unstable();
    Some(exercise(
        ExerciseFormat::MultiChoice,
        Some(question.clone()),
        Vec::new(),
        None,
        Input::MultiChoice { items },
        Answer::Set(right),
        None,
        question,
        explanation,
        format!("parity-pick:{word}:{key_items:?}"),
    ))
}
