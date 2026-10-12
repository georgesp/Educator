---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaZc
rev: 2026-10-11T18:50:57Z
url: https://github.com/georgesp/Educator/issues/197
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012109 — Exercices CM2 — Multiples, diviseurs et diviseurs communs"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 1670
---

## User story

En tant qu'enfant de CM2, je veux m'entraîner à trouver les multiples et les diviseurs d'un nombre et les diviseurs communs afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM2 : critères de divisibilité seulement par 2, 5 et 10 ; diviseurs des nombres jusqu'à 100, tous les diviseurs d'un nombre jusqu'à 30, diviseurs et multiples communs.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm2-divisors` | 2 | Diviseurs d'un nombre | Tous les diviseurs d'un nombre ≤ 30 ; « d est-il un diviseur de n ? » pour n ≤ 100 | `choice` (multiple), `yesNo` |
| `cm2-common-divisors` | 3 | Multiples et diviseurs communs | Diviseurs communs de deux nombres ≤ 60, multiples communs de deux nombres ≤ 12 (jusqu'à 100) | `choice` (multiple) |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Tous les diviseurs de 24 : 1, 2, 3, 4, 6, 8, 12, 24.
- Diviseurs communs de 18 et 24 : 1, 2, 3, 6.
- Un multiple commun de 4 et 6 : 12, 24…

## Évolutions du moteur et de l'écran de série

- Format `choice` à plusieurs bonnes réponses (lot 1, CE1) ; validation : toutes les bonnes et aucune fausse, retour détaillé.

## Distracteurs (QCM)

Oubli de 1 et du nombre lui-même, multiple donné pour diviseur.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM2 — Multiples, diviseurs et diviseurs communs » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
