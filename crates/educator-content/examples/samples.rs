//! Writes `docs/specs/echantillons-lot1.md`: 20 exercises per skill (seeds 1 and 2, QCM mode), for
//! the manual review of the generation rules (SUB000108, SUB000112).
//!
//! `cargo run -p educator-content --example samples` (from the repository root).

use educator_content::EducatorContent;
use educator_core::exercises::{generate_series, CalculationSpace, DEFAULT_LENGTH};
use educator_core::skills::{AnswerMode, SkillCatalog, Subject};
use educator_core::Grade;
use std::fmt::Write as _;
use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let content = EducatorContent::load(&root.join("content"));
    assert!(content.problems.is_empty(), "{:?}", content.problems);

    let mut out = String::from(
        "# Échantillons des compétences du lot 1\n\n\
         Fiches SUB000108 et SUB000112 : 20 exercices générés par compétence (graines 1 et 2, mode QCM), pour la revue manuelle.\n\
         Régénérés avec `cargo run -p educator-content --example samples`. Entre crochets : les 4 propositions du QCM.\n",
    );
    for grade in [Grade::CP, Grade::CE1] {
        for skill in content.skills.for_grade(Subject::Math, grade) {
            let _ = write!(
                out,
                "\n## {grade} · période {} · {} (`{}`)\n\n{} exercices différents possibles.\n\n",
                skill.period,
                skill.title,
                skill.id,
                CalculationSpace::of(skill).distinct_count()
            );
            for seed in [1, 2] {
                for exercise in
                    generate_series(skill, seed, AnswerMode::Choice, DEFAULT_LENGTH).unwrap()
                {
                    let choices = exercise
                        .choices
                        .as_ref()
                        .map(|c| {
                            c.iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(", ")
                        })
                        .unwrap_or_default();
                    let _ = writeln!(
                        out,
                        "- `{}` → {} [{choices}]",
                        exercise.prompt(),
                        exercise.expected()
                    );
                }
            }
        }
    }

    let path = root.join("docs/specs/echantillons-lot1.md");
    std::fs::write(&path, out).unwrap();
    println!("{}", path.display());
}
