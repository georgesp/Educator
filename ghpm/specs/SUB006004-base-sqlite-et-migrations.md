---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fOI
url: https://github.com/georgesp/Educator/issues/64
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006004 — Base SQLite et migrations
status: In progress
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

- `Educator.Data` : `AppDbContext` EF Core 10, SQLite.
- Emplacement : `~/Library/Application Support/Educator/educator.db` (macOS), `%APPDATA%\Educator\educator.db` (Windows).
- Règles de l'ADR 0001 de GitHubProjectManager : `IDbContextFactory`, `MigrateAsync` au démarrage, sauvegarde `VACUUM INTO` avant migration d'une base existante, mode WAL, dates UTC en ms, enums en texte, Guid v7, intercepteur `CreatedAt` / `UpdatedAt` alimenté par `TimeProvider`.
- Table `AppSetting` (clé / valeur) pour les réglages.

## Critères d'acceptation

- [x] Migration initiale appliquée au premier lancement.
- [x] Sauvegarde créée avant toute migration d'une base existante (testé).
- [x] Tests d'intégration sur une base SQLite réelle temporaire.

## Implémentation (2026-10-10)

Migration `InitialCreate` (table `AppSetting`), test `Migrations_AreInSyncWithTheModel`. Variable `EDUCATOR_DATA_DIR` pour déplacer le dossier de données (tests, débogage).
