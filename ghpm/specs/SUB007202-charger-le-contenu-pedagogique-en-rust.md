---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAvzE
rev: 2026-10-11T15:22:55Z
url: https://github.com/georgesp/Educator/issues/79
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB007202 — Charger le contenu pédagogique en Rust
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#72"
order: 270
---

## User story

En tant que développeur, je veux que le contenu de `content/` soit chargé et validé en Rust afin qu'une règle mal écrite fasse échouer les tests, jamais planter devant l'enfant.

## Comportement attendu

- Crate `crates/educator-content`, portage de `Educator.Content` hors Piper : `ContentDirectory`, `EducatorContent`, `SkillLoader`, `SkillCatalog`, `CharacterCatalog`, `SmallContentLoader` (palette, calendrier, points).
- Les fichiers JSON/SVG et les schémas de `content/` sont réutilisés sans modification.
- Validation par schéma JSON dans les tests (crate `jsonschema`), comme aujourd'hui.
- Le contenu est embarqué comme ressource Tauri (même mécanisme qu'aujourd'hui pour le host).

## Critères d'acceptation

- [x] Portage des tests de `Educator.Content.Tests` hors Piper : chaque fichier de `content/` est validé par son schéma.
- [x] Une règle invalide (schéma ou contrainte métier) fait échouer `cargo test`.
- [x] Les 5 personnages et leurs SVG sont chargés comme avant.

## Notes de réalisation

- Crate `crates/educator-content` : `EducatorContent::load`, `load_skills` / `parse_skill`, `Characters`
  (SVG vérifiés avec `roxmltree`), `load_calendar`, `load_points`, `find_content_directory`.
- Validation par schéma JSON 2020-12 dans les tests (`jsonschema`), propriété « 10 000 tirages par compétence »
  et règles « trop étroite » / « trop large » reprises.
- Contenu embarqué comme ressource Tauri (`bundle.resources` de `tauri.conf.json`) ; en développement, Tauri en copie
  une version à côté de l'exécutable, et `EDUCATOR_CONTENT_DIR` reste possible.
