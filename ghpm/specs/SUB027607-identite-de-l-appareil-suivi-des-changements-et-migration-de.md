---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhA4_Ig
url: https://github.com/georgesp/Educator/issues/283
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB027607 — Identité de l'appareil, suivi des changements et migration de la base"
status: Todo
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

- [ ] Une base existante (migrations 1 à 3) est migrée sans perte ; test d'intégration sur une base de l'état actuel.
- [ ] Toute modification d'un profil, d'une séance, d'une réponse ou d'une progression de leçon est visible dans le suivi ; la suppression d'un profil laisse une pierre tombale.
- [ ] L'identifiant d'appareil est stable après relance et n'est jamais copié par une sauvegarde de la base d'un appareil à l'autre (cas à documenter dans l'ADR).
- [ ] Tests unitaires et d'intégration (`cargo test --workspace`), `clippy` sans avertissement.
