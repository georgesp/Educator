---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZMI
rev: 2026-10-11T18:49:18Z
url: https://github.com/georgesp/Educator/issues/136
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011914 — Exercices CE2 — Soustraire 9, 19, 29 ou 39"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 990
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à soustraire de tête 9, 19, 29 ou 39 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2, calcul mental : soustraire 9, 19, 29 ou 39 à un nombre.
- Procédures du CE1 (soustraire 9) réinvesties.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-sub-near-tens` | 2 | Soustraire 9, 19, 29 ou 39 | Premier nombre de 40 à 999, second dans {9, 19, 29, 39}, résultat positif | `result`, `missingOperand` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 84 − 29 = ? → 55.
- 312 − 39 = ? → 273.
- 150 − … = 131 → 19.

## Évolutions du moteur et de l'écran de série

- Aucun.

## Distracteurs (QCM)

Compensation dans le mauvais sens (53 au lieu de 55), oubli de la compensation (54), erreur d'une dizaine.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Soustraire 9, 19, 29 ou 39 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
