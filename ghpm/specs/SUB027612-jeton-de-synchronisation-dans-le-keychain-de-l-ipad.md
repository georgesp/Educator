---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBLzfw
url: https://github.com/georgesp/Educator/issues/289
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB027612 — Jeton de synchronisation dans le Keychain de l'iPad
status: Todo
size: S
estimate:
startDate:
endDate:
parent: "#276"
order: 5120
---

## User story

En tant que parent, je veux que le jeton qui permet à l'iPad de synchroniser soit rangé dans le stockage sécurisé
du système.

## Contexte

Suivi du lot tablette 2 (ADR 0007, section 8, SUB027601). Aujourd'hui le jeton est dans la base de l'application sur
l'iPad (dossier protégé de l'application, jamais dans la sauvegarde Educator). Le Keychain demande du Swift ou une
dépendance à tester sur l'appareil.

## Critères d'acceptation

- [ ] Le jeton est lu et écrit dans le Keychain ; la base ne le contient plus (migration : déplacé au premier démarrage).
- [ ] Réinstaller l'application (profil gratuit de 7 jours) ne casse pas l'association.
- [ ] Tests du contrat côté Rust ; essai sur l'iPad consigné dans la recette.
