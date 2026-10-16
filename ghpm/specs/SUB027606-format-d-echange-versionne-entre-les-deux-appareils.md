---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhA4_GE
rev: 2026-10-15T05:31:37Z
url: https://github.com/georgesp/Educator/issues/282
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB027606 — Format d'échange versionné entre les deux appareils
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#276"
order: 5040
---

## User story

En tant que Georges, je veux un format d'échange clair et versionné, pour que l'ordinateur et la tablette puissent évoluer sans casser la synchronisation.

## Travail

- Messages JSON décrits par un schéma (même démarche que le contenu pédagogique : une erreur de format fait échouer les tests, jamais l'application devant l'enfant) : demande de synchronisation (version du protocole, version de la base, identifiant d'appareil, curseur), lot de changements (profils, séances, réponses, progressions, pierres tombales), accusé de réception.
- Envoi **par lots** de taille bornée, avec reprise : une coupure Wi-Fi en cours de route est sans conséquence (voir idempotence de la fiche « Moteur de fusion »).
- Contrats Rust dans `contracts.rs` si exposés au front, types TypeScript **générés** (`npm run api:generate`).
- Données minimales : pas de champ superflu (pas d'adresse, pas de nom d'appareil de l'enfant), uniquement ce qui sert à la fusion.

## Critères d'acceptation

- [x] Schéma JSON versionné et exemples de messages dans le dépôt, validés par des tests.
- [x] Un message d'une version inconnue ou mal formé est refusé avec une erreur claire, sans plantage.
- [x] Lot volumineux (plusieurs milliers de réponses) découpé et reconstitué sans perte ; test de reprise après interruption.
- [x] `web/src/api/types.ts` régénéré et commité (le test `bindings::types_ts_is_up_to_date` passe).

## Réalisation (2026-10-15)

- `crates/educator-sync/src/wire.rs`, schéma `crates/educator-sync/schemas/sync-messages.schema.json` et exemples
  `schemas/examples/` (validés par `tests/schemas.rs`). Protocole 1 ; lots de 200 séances ou 2 000 éléments au plus.
- Les messages d'échange ne sont pas exposés au front ; seuls les contrats des commandes (`contracts/sync.rs`) le sont.
