---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhA4_Ig
rev: 2026-10-15T05:31:33Z
url: https://github.com/georgesp/Educator/issues/283
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB027607 — Identité de l'appareil, suivi des changements et migration de la base"
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#276"
order: 5020
---

## User story

En tant que parent, je veux que chaque appareil sache qui il est et ce qu'il a déjà partagé, pour que la synchronisation n'envoie que ce qui a changé.

## Travail

- Migration `0004_sync` dans `educator-data` (jamais de modification d'une migration publiée) : identifiant d'appareil (créé au premier démarrage, dans `AppSetting`), table des appareils appairés (nom affiché, identifiant, empreinte du jeton, date d'appairage, dernière synchronisation, révoqué), curseurs de synchronisation par appareil, pierres tombales des suppressions, et suivi des changements retenu par l'ADR 0006 (numéro de séquence ou version par ligne).
- Les écritures existantes (profil, séance, réponse, progression de leçon, suppression de profil) alimentent ce suivi dans la **même transaction**.
- Les profils, séances et réponses déjà présents à la migration sont marqués « à envoyer ».
- Traits de stockage dans `educator-core` (synchrones, comme les autres), implémentés dans `educator-data`.

## Critères d'acceptation

- [x] Une base existante (migrations 1 à 3) est migrée sans perte ; test d'intégration sur une base de l'état actuel.
- [x] Toute modification d'un profil, d'une séance, d'une réponse ou d'une progression de leçon est visible dans le suivi ; la suppression d'un profil laisse une pierre tombale.
- [x] L'identifiant d'appareil est stable après relance et n'est jamais copié par une sauvegarde de la base d'un appareil à l'autre (cas à documenter dans l'ADR).
- [x] Tests unitaires et d'intégration (`cargo test --workspace`), `clippy` sans avertissement.

## Réalisation (2026-10-15)

- Migration `0005_sync` (`crates/educator-data/src/migrations.rs`) : `SyncState`, `SyncRow`, `SyncCounter`,
  `SyncPeer`, et des **déclencheurs** sur chaque table synchronisée : le suivi est écrit dans la même transaction que
  la donnée, sans toucher aux stores existants. L'horloge hybride vient de la fonction SQL `educator_hlc()`
  (`educator-data::database`).
- Identifiant d'appareil créé au premier démarrage après la migration ; tout l'existant est marqué « à envoyer ».
- La sauvegarde n'emporte aucune table de synchronisation ; une restauration n'envoie aucune suppression.
- Tests : `crates/educator-data/tests/sync.rs` (base du lot 3 migrée, suivi de chaque écriture, identité, sauvegarde).
