//! Equal groups of the CP (SUB012204, SUB012207): « 3 fois 20 » as a repeated addition, a fair
//! share (« 18 images pour 3 enfants ») and groups of a given size (« 24 élèves en équipes de 4 »),
//! without the signs × and ÷ (introduced later).

use super::family::{number_choices, pick_value, Draw, Position};
use super::problems::elide;
use super::{Answer, Exercise, Input, Picture, Token};
use crate::random::SeededRng;
use crate::skills::{AnswerMode, ExerciseFormat, GroupKind, Skill};
use rand::seq::IndexedRandom;

/// Things shared between children: (things, children). Plural nouns, read as they are written.
const SHARES: [(&str, &str); 6] = [
    ("images", "enfants"),
    ("billes", "amis"),
    ("bonbons", "enfants"),
    ("crayons", "élèves"),
    ("cartes", "joueurs"),
    ("drapeaux", "pilotes"),
];

/// Things put in groups of the same size: (things, the group in the singular, its plural).
const GROUPINGS: [(&str, &str, &str); 5] = [
    ("élèves", "équipe", "équipes"),
    ("œufs", "boîte", "boîtes"),
    ("voitures", "garage", "garages"),
    ("fleurs", "bouquet", "bouquets"),
    ("casques", "étagère", "étagères"),
];

pub fn draw(
    skill: &Skill,
    format: ExerciseFormat,
    mode: AnswerMode,
    _position: Position,
    random: &mut SeededRng,
) -> Option<Draw> {
    let rule = skill.params.groups.as_ref()?;
    let kind = *rule.kinds.choose(random)?;
    // `count`: the number the child is given (« 3 fois », 3 enfants, équipes de 3), `size`: the other.
    let count = pick_value(&rule.count.allowed_values(), random)?;
    let size = pick_value(&rule.size.allowed_values(), random)?;
    if count * size > rule.max_total {
        return None;
    }

    let exercise = match kind {
        GroupKind::Times => times(count, size, format, mode, random),
        GroupKind::Share => share(count, size, format, mode, random),
        // Groups of `count`: « 24 élèves en équipes de 4 » makes `size` groups.
        GroupKind::GroupBy => group_by(size, count, format, mode, random),
    }?;
    Some(Draw {
        exercise,
        difficulty: 0,
    })
}

/// `5 + 5 + 5 + 5` for 4 groups of 5.
fn repeated(count: i32, size: i32) -> String {
    vec![size.to_string(); count.max(1) as usize].join(" + ")
}

fn repeated_tokens(count: i32, size: i32) -> Vec<Token> {
    let mut tokens = Vec::new();
    for i in 0..count {
        if i > 0 {
            tokens.push(Token::Sign("+".to_owned()));
        }
        tokens.push(Token::Number(size));
    }
    tokens
}

/// The groups drawn: boxes of dots up to 10 per box, bars of ten beyond (3 fois 20: 6 bars).
fn groups_picture(count: i32, size: i32) -> Picture {
    if size > 10 && size % 10 == 0 {
        Picture::BaseTen {
            hundreds: 0,
            tens: (count * size / 10).unsigned_abs(),
            units: 0,
        }
    } else {
        Picture::Groups {
            groups: count.unsigned_abs(),
            per_group: size.unsigned_abs(),
        }
    }
}

fn input(
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

#[allow(clippy::too_many_arguments)]
fn exercise(
    format: ExerciseFormat,
    question: Option<String>,
    statement: Vec<Token>,
    picture: Option<Picture>,
    help: Option<Picture>,
    (input, choices): (Input, Option<Vec<Answer>>),
    expected: i32,
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
        expected: Answer::Number(expected),
        choices,
        spoken,
        explanation,
        help,
        key,
        calculation: None,
        word: None,
    }
}

/// « 3 fois 20 = ? » (result) or « 4 fois 5 = 5 + 5 + 5 + 5 = ? » (phrase), with the groups drawn.
fn times(
    count: i32,
    size: i32,
    format: ExerciseFormat,
    mode: AnswerMode,
    random: &mut SeededRng,
) -> Option<Exercise> {
    let total = count * size;
    let fois = || Token::Text("fois".to_owned());
    let equals = || Token::Sign("=".to_owned());
    let mut statement = vec![Token::Number(count), fois(), Token::Number(size), equals()];
    let spoken = if format == ExerciseFormat::Phrase {
        statement.extend(repeated_tokens(count, size));
        statement.push(equals());
        format!(
            "{count} fois {size}, c'est {} : combien ça fait ?",
            repeated(count, size)
        )
    } else {
        format!("Combien font {count} fois {size} ?")
    };
    statement.push(Token::Slot(0));
    // The two numbers added (3 fois 20 → 23), one group too many or too few.
    let mistakes = [count + size, total + size, total - size, total + 1];
    Some(exercise(
        format,
        None,
        statement,
        Some(groups_picture(count, size)),
        None,
        input(total, &mistakes, mode, random),
        total,
        spoken,
        format!(
            "{count} fois {size}, c'est {} = {total}.",
            repeated(count, size)
        ),
        format!("times:{count}:{size}"),
    ))
}

/// A fair share: « 18 images pour 3 enfants : combien chacun ? » (phrase) or « 3 fois ? = 18 »
/// (result); the groups are drawn after a mistake.
fn share(
    parts: i32,
    each: i32,
    format: ExerciseFormat,
    mode: AnswerMode,
    random: &mut SeededRng,
) -> Option<Exercise> {
    let total = parts * each;
    let (things, children) = *SHARES.choose(random)?;
    let (question, statement, spoken, key) = if format == ExerciseFormat::Phrase {
        let question = format!(
            "{total} {things} pour {parts} {children}, autant pour chacun. Combien chacun en a-t-il ?"
        );
        (
            Some(question.clone()),
            vec![Token::Slot(0)],
            question,
            format!("share:{total}:{parts}"),
        )
    } else {
        (
            Some(format!("Partage {total} en {parts} parts égales.")),
            vec![
                Token::Number(parts),
                Token::Text("fois".to_owned()),
                Token::Slot(0),
                Token::Sign("=".to_owned()),
                Token::Number(total),
            ],
            format!(
                "Partage {total} en {parts} parts égales : {parts} fois combien font {total} ?"
            ),
            format!("share:{total}:{parts}"),
        )
    };
    // Taken away instead of shared, the number of parts, one too many or too few.
    let mistakes = [total - parts, parts, each + 1, each - 1];
    Some(exercise(
        format,
        question,
        statement,
        None,
        Some(Picture::Groups {
            groups: parts.unsigned_abs(),
            per_group: each.unsigned_abs(),
        }),
        input(each, &mistakes, mode, random),
        each,
        spoken,
        format!(
            "{total} partagé en {parts} parts égales : {} = {total}, donc {each} dans chaque part.",
            repeated(parts, each)
        ),
        key,
    ))
}

/// Groups of a given size: « 24 élèves en équipes de 4 : combien d'équipes ? » (phrase) or
/// « ? fois 4 = 24 » (result); the groups are drawn after a mistake.
fn group_by(
    groups: i32,
    size: i32,
    format: ExerciseFormat,
    mode: AnswerMode,
    random: &mut SeededRng,
) -> Option<Exercise> {
    let total = groups * size;
    let (things, group, plural) = *GROUPINGS.choose(random)?;
    let (question, statement, spoken, key) = if format == ExerciseFormat::Phrase {
        let question = elide(&format!(
            "{total} {things}, {size} par {group}. Combien de {plural} ?"
        ));
        (
            Some(question.clone()),
            vec![Token::Slot(0)],
            question,
            format!("group:{total}:{size}"),
        )
    } else {
        (
            Some(format!("Combien de groupes de {size} dans {total} ?")),
            vec![
                Token::Slot(0),
                Token::Text("fois".to_owned()),
                Token::Number(size),
                Token::Sign("=".to_owned()),
                Token::Number(total),
            ],
            format!("Combien de fois {size} pour faire {total} ?"),
            format!("group:{total}:{size}"),
        )
    };
    // The size of a group, taken away instead of grouped, one group too many or too few.
    let mistakes = [size, total - size, groups + 1, groups - 1];
    Some(exercise(
        format,
        question,
        statement,
        None,
        Some(Picture::Groups {
            groups: groups.unsigned_abs(),
            per_group: size.unsigned_abs(),
        }),
        input(groups, &mistakes, mode, random),
        groups,
        spoken,
        format!(
            "{} = {total} : {groups} groupes de {size}.",
            repeated(groups, size)
        ),
        key,
    ))
}
