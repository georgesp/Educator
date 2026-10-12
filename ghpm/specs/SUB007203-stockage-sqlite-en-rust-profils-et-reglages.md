---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAv1A
rev: 2026-10-11T15:22:56Z
url: https://github.com/georgesp/Educator/issues/80
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB007203 — Stockage SQLite en Rust (profils et réglages)
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#72"
order: 280
---

## User story

En tant qu'enfant, je veux retrouver mon profil, ma classe et mes points à chaque ouverture de l'application.

## Comportement attendu

- Crate `crates/educator-data` avec `rusqlite` (SQLite embarqué, `bundled`) et un mécanisme de migrations versionnées (`rusqlite_migration` ou équivalent).
- Portage de `SqliteProfileStore`, `SqliteAppSettingsStore`, `DatabaseInitializer`, des horodatages (`TimestampsInterceptor`) et de l'emplacement des données (`AppDataPaths`).
- Tables `Profile` et `AppSetting` : mêmes colonnes et contraintes qu'aujourd'hui.

## Décision à prendre

Reprendre la base existante (lire le schéma créé par EF Core, ignorer `__EFMigrationsHistory`) ou repartir d'une base neuve. L'application n'étant pas encore diffusée, repartir d'une base neuve est acceptable ; la décision est notée dans l'ADR de migration.

## Critères d'acceptation

- [x] Portage des tests de `Educator.Data.Tests` (SQLite réel en fichier temporaire).
- [x] Une base créée à la première ouverture, migrée aux ouvertures suivantes.
- [x] Les horodatages sont enregistrés en millisecondes Unix, comme aujourd'hui.

## Notes de réalisation

- Crate `crates/educator-data` (`rusqlite` 0.37 `bundled`). Migrations maison (liste `MIGRATIONS`, version dans
  `PRAGMA user_version`, une transaction par migration) plutôt que `rusqlite_migration` : une vingtaine de lignes.
- **Décision : la base existante est reprise.** La migration `0001_initial` recrée le schéma EF Core à l'identique
  avec `IF NOT EXISTS` ; une base .NET est sauvegardée (`VACUUM INTO`) puis adoptée avec ses profils (test
  `database_of_the_dotnet_version_is_adopted_with_its_profiles`). Guid en texte majuscule comme EF Core. Voir ADR 0003.
- `UpdatedAt` ne bouge que si une colonne change (profils et réglages), comme l'intercepteur EF Core.
