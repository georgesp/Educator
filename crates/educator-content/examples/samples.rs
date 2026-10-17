//! Writes the samples of the generation rules, for the manual review (20 exercises per skill,
//! seeds 1 and 2, QCM mode when the skill has it):
//! - `docs/specs/echantillons-lot1.md`: additions and subtractions of CP and CE1 (SUB000108, SUB000112);
//! - `docs/specs/echantillons-ce1.md`: the CE1 complements (SUB000118 to SUB000124);
//! - `docs/specs/echantillons-cp.md`: the CP complements (lot 7, US0122).
//!
//! `cargo run -p educator-content --example samples` (from the repository root).

use educator_content::EducatorContent;
use educator_core::exercises::{
    distinct_count, generate_series, Answer, Exercise, Input, Picture, DEFAULT_LENGTH,
};
use educator_core::skills::{AnswerMode, Skill, SkillCatalog, SkillOperation, Subject};
use educator_core::Grade;
use std::fmt::Write as _;
use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let content = EducatorContent::load(&root.join("content"));
    assert!(content.problems.is_empty(), "{:?}", content.problems);

    let additive = |s: &Skill| {
        matches!(
            s.operation,
            SkillOperation::Addition | SkillOperation::Subtraction | SkillOperation::Mixed
        )
    };

    let lot1 = samples(
        &content,
        "# Échantillons des compétences du lot 1\n\n\
         Fiches SUB000108 et SUB000112 : 20 exercices générés par compétence (graines 1 et 2, mode QCM), pour la revue manuelle.\n\
         Régénérés avec `cargo run -p educator-content --example samples`. Entre crochets : les 4 propositions du QCM.\n",
        &[Grade::CP, Grade::CE1],
        &additive,
        false,
    );
    write(&root.join("docs/specs/echantillons-lot1.md"), &lot1);

    let ce1 = samples(
        &content,
        "# Échantillons des compléments du CE1\n\n\
         Fiches SUB000118 à SUB000124 : 20 exercices générés par compétence (graines 1 et 2, mode QCM quand la compétence \
         le propose), pour la revue manuelle.\n\
         Régénérés avec `cargo run -p educator-content --example samples`. `?` marque la réponse attendue de l'enfant ; \
         entre crochets : les propositions du QCM ; entre parenthèses : l'image ou la saisie particulière ; \
         après « Correction » : ce qui est affiché après une erreur.\n",
        &[Grade::CE1],
        &|s: &Skill| !additive(s),
        true,
    );
    write(&root.join("docs/specs/echantillons-ce1.md"), &ce1);

    let cp = samples(
        &content,
        "# Échantillons des compléments du CP\n\n\
         Lot 7 (US0122) : 20 exercices générés par compétence (graines 1 et 2, mode QCM quand la compétence le propose), \
         pour la revue manuelle. Les 11 compétences d'addition et de soustraction du lot 1 sont dans `echantillons-lot1.md`.\n\
         Régénérés avec `cargo run -p educator-content --example samples`. `?` marque la réponse attendue de l'enfant ; \
         entre crochets : les propositions du QCM ; entre parenthèses : l'image, la saisie particulière ou ce que \
         l'enfant entend au clic ; après « Correction » : ce qui est affiché après une erreur.\n",
        &[Grade::CP],
        &|s: &Skill| !additive(s),
        true,
    );
    write(&root.join("docs/specs/echantillons-cp.md"), &cp);
}

fn write(path: &Path, text: &str) {
    std::fs::write(path, text).unwrap();
    println!("{}", path.display());
}

fn samples(
    content: &EducatorContent,
    header: &str,
    grades: &[Grade],
    keep: &dyn Fn(&Skill) -> bool,
    detailed: bool,
) -> String {
    let mut out = String::from(header);
    for &grade in grades {
        for skill in content
            .skills
            .for_grade(Subject::Math, grade)
            .into_iter()
            .filter(|s| keep(s))
        {
            let _ = write!(
                out,
                "\n## {grade} · période {} · {} (`{}`)\n\n{} exercices différents possibles.\n\n",
                skill.period,
                skill.title,
                skill.id,
                count_label(distinct_count(skill), skill.operation.is_calculation())
            );
            let mode = if skill.answer_modes.contains(&AnswerMode::Choice) {
                AnswerMode::Choice
            } else {
                AnswerMode::Keyboard
            };
            for seed in [1, 2] {
                for exercise in generate_series(skill, seed, mode, DEFAULT_LENGTH).unwrap() {
                    if detailed {
                        let _ = writeln!(out, "{}", detailed_line(&exercise));
                    } else {
                        let _ = writeln!(
                            out,
                            "- `{}` → {} [{}]",
                            exercise.prompt(),
                            exercise.expected_label(),
                            choices(&exercise)
                        );
                    }
                }
            }
        }
    }
    out
}

/// Families other than calculations count their exercises on a sample of draws: « au moins ».
fn count_label(count: usize, exact: bool) -> String {
    if exact {
        count.to_string()
    } else {
        format!("Au moins {count}")
    }
}

fn choices(exercise: &Exercise) -> String {
    exercise
        .choices
        .as_ref()
        .map(|c| c.iter().map(Answer::label).collect::<Vec<_>>().join(", "))
        .unwrap_or_default()
}

fn detailed_line(exercise: &Exercise) -> String {
    let mut line = format!("- `{}`", exercise.prompt());
    if exercise.format.needs_listening() {
        let _ = write!(line, " (entendu au clic : « {} »)", exercise.spoken);
    }
    if let Some(picture) = &exercise.picture {
        let _ = write!(line, " ({})", describe(picture));
    }
    match &exercise.input {
        Input::Order { items, .. } => {
            let _ = write!(line, " (à ranger : {items:?})");
        }
        Input::MultiChoice { items } => {
            let _ = write!(line, " (nombres : {items:?})");
        }
        Input::NumberLine => line.push_str(" (clic sur la droite)"),
        _ => {}
    }
    let _ = write!(line, " → **{}**", exercise.expected_label());
    if let Some(choices) = exercise.choices.as_ref() {
        let labels: Vec<String> = choices.iter().map(Answer::label).collect();
        let _ = write!(line, " [{}]", labels.join(" · "));
    }
    let _ = write!(line, " — Correction : {}", exercise.explanation);
    if let Some(help) = &exercise.help {
        let _ = write!(line, " ({})", describe(help));
    }
    line
}

fn describe(picture: &Picture) -> String {
    match picture {
        Picture::BaseTen {
            hundreds,
            tens,
            units,
        } => format!("image : {hundreds} plaques, {tens} barres, {units} cubes"),
        Picture::FractionShape {
            shape,
            parts,
            colored,
        } => format!("image : {shape:?} en {parts} parts, {colored} coloriée(s)").replace("Bar", "bande").replace("Disc", "disque"),
        Picture::NumberLine {
            start,
            step,
            ticks,
            arrow,
            ..
        } => match arrow {
            Some(k) => format!(
                "droite graduée de {step} en {step} à partir de {start}, {ticks} graduations, flèche sur la graduation {k}"
            ),
            None => format!("droite graduée de {step} en {step} à partir de {start}, {ticks} graduations"),
        },
        Picture::ArrayGrid {
            rows,
            columns,
            split,
        } => match split {
            Some(split) => format!("quadrillage {rows} × {columns} coupé après {split} colonnes"),
            None => format!("{rows} rangées de {columns} points"),
        },
        Picture::BarModel { bars, total } => {
            let bars: Vec<String> = bars
                .iter()
                .map(|bar| {
                    bar.iter()
                        .map(|s| s.value.map_or("?".to_owned(), |v| v.to_string()))
                        .collect::<Vec<_>>()
                        .join(" | ")
                })
                .collect();
            let total = total.map_or(String::new(), |t| format!(", total {t}"));
            format!("schéma en barres : [{}]{total}", bars.join("] ["))
        }
        Picture::Groups { groups, per_group } => {
            format!("schéma : {groups} groupes de {per_group}")
        }
    }
}
