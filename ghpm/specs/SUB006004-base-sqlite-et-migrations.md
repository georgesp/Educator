---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fOI
rev: 2026-10-11T07:56:45Z
url: https://github.com/georgesp/Educator/issues/64
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006004 — Base SQLite et migrations
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#60"
order: 5
---

## User story

En tant que parent, je veux que les profils et les progrès de mes enfants soient conservés sur l'ordinateur, sans compte en ligne.

## Comportement attendu

- Crate `educator-data` : SQLite embarqué (`rusqlite`, `bundled`).
- Emplacement : `~/Library/Application Support/Educator/educator.db` (macOS), `%APPDATA%\Educator\educator.db` (Windows).
- Migrations versionnées au démarrage (`PRAGMA user_version`), sauvegarde `VACUUM INTO` avant migration d'une base existante, mode WAL, dates UTC en ms, enums en texte, Guid v7, `CreatedAt` / `UpdatedAt` renseignés à partir de l'horloge injectée (`Clock`).
- Table `AppSetting` (clé / valeur) pour les réglages.

## Critères d'acceptation

- [x] Migration initiale appliquée au premier lancement.
- [x] Sauvegarde créée avant toute migration d'une base existante (testé).
- [x] Tests d'intégration sur une base SQLite réelle temporaire.

## Implémentation

Migration `0001_initial` (tables `AppSetting` et `Profile`), écrite avec `IF NOT EXISTS` pour reprendre une base créée
par la version .NET. Tests dans `crates/educator-data/tests/database.rs`. Variable `EDUCATOR_DATA_DIR` pour déplacer le
dossier de données (tests, débogage).

*Première version en .NET (2026-10-10), portée en Rust au lot 1bis (2026-10-11, ADR 0003).*
