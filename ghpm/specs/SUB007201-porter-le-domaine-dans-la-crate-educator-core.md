---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAvxo
url: https://github.com/georgesp/Educator/issues/78
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB007201 — Porter le domaine dans la crate educator-core
status: Done
size: L
estimate:
startDate:
endDate:
parent: "#72"
order: 135
---

## User story

En tant que développeur, je veux le domaine d'Educator en Rust pur afin qu'il tourne dans le cœur de Tauri, sur bureau comme sur tablette.

## Comportement attendu

- Workspace Cargo à la racine du dépôt, crate `crates/educator-core`.
- Portage de `Educator.Core` : compétences (`Skill`, `ISkillCatalog`, `SchoolCalendar`), exercices (`Calculation`, `CalculationSpace`, `Exercise`, `SeriesGenerator`, `SeriesService`), profils (`Profile`, `ProfileService`, `IProfileStore`), points (`PointsRules`), réglages (`IAppSettingsStore`), texte vocal (`FrenchSpeechText`, `ISpeechSynthesizer`).
- Les interfaces C# deviennent des traits ; `IRandomSource` devient un trait implémenté avec `rand` (générateur à graine pour les tests).
- L'état des séries en cours (aujourd'hui un `ConcurrentDictionary`) reste en mémoire, protégé par un `Mutex`.
- Portage des tests de `Educator.Core.Tests` (~700 lignes).

## Points d'attention

- Le générateur aléatoire de Rust ne donne pas les mêmes tirages que `System.Random` : les tests qui attendent des valeurs précises pour une graine donnée sont réécrits (propriétés vérifiées ou nouvelles valeurs attendues), pas recopiés.
- `FrenchSpeechText` (nombres et signes lus en toutes lettres) est testé sur les mêmes cas qu'aujourd'hui : la sortie doit être identique.

## Critères d'acceptation

- [x] `cargo test -p educator-core` vert.
- [x] `educator-core` ne dépend d'aucune crate d'infrastructure (vérifié par un test sur `cargo metadata`).
- [x] Les règles de génération de `docs/specs/regles-de-generation.md` sont respectées, avec les mêmes cas de test qu'en C#.

## Notes de réalisation

- Workspace Cargo à la racine (`Cargo.toml`, `Cargo.lock`), crate `crates/educator-core`. Dépendances : `rand`,
  `rand_chacha`, `uuid`, `thiserror` (liste blanche de `tests/architecture.rs`, qui vérifie aussi que `data` et
  `content` dépendent de `core` et pas l'un de l'autre).
- Traits synchrones : `SkillCatalog`, `CharacterCatalog`, `ProfileStore`, `AppSettingsStore`, `SpeechSynthesizer`,
  `RandomSource`, `Clock` (instants en millisecondes Unix). Erreurs : `educator_core::Error`.
- Séries générées avec ChaCha8 à partir de la graine : les tests à valeurs précises (`new Random(3)`) vérifient
  maintenant la propriété sur 50 graines.
- `FrenchSpeechText` → `speech::normalize` / `number_to_words`, sans regex, sur les mêmes cas (sortie identique).
- `UnavailableSpeechSynthesizer` placé dans `core` (utilisé aussi sur tablette).
