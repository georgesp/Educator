---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZJA
rev: 2026-10-11T18:49:34Z
url: https://github.com/georgesp/Educator/issues/133
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011911 — Exercices CE2 — Multiplier par 10 et par 100
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1090
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à multiplier un nombre par 10 ou par 100 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2, calcul mental : multiplier un nombre entier par 10 ou par 100 (ex. 10 × 724 = 7 240).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-mul-10-100` | 2 | Multiplier par 10 et par 100 | n × 10 (n ≤ 999) et n × 100 (n ≤ 99), résultat ≤ 10 000, ordre au hasard ; formats facteur manquant | `result`, `missingOperand` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 10 × 724 = ? → 7 240.
- 36 × 100 = ? → 3 600.
- … × 10 = 450 → 45.

## Évolutions du moteur et de l'écran de série

- Aucun après l'opération `multiplication`.

## Distracteurs (QCM)

Zéro oublié ou en trop (724, 72 400), × 10 et × 100 confondus.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Multiplier par 10 et par 100 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
