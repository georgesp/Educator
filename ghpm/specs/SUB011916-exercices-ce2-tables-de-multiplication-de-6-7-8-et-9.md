---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZOM
rev: 2026-10-11T18:49:28Z
url: https://github.com/georgesp/Educator/issues/138
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011916 — Exercices CE2 — Tables de multiplication de 6, 7, 8 et 9"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1050
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à connaître les tables de 6, 7, 8 et 9 dans les deux sens afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2 : toutes les tables de multiplication connues dans les deux sens en fin de CE2.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-mul-tables-6-9` | 3 | Tables de 6, 7, 8 et 9 | a × b avec a ∈ {6, 7, 8, 9}, b de 1 à 10, ordre au hasard | `result`, `missingOperand` |
| `ce2-mul-tables-all` | 4 | Toutes les tables | a × b de 2 à 10, faits les moins sûrs plus fréquents quand la révision espacée (lot 3) existera | `result`, `missingOperand` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 7 × 8 = ?
- 9 × … = 54
- 42 = … × 7

## Évolutions du moteur et de l'écran de série

- Aucun en plus de la fiche « Exercices CE2 — Tables de multiplication de 2, 3, 4, 5 et 10 ».

## Distracteurs (QCM)

Table voisine, ± un facteur, inversion des chiffres (63 / 36 pour 9 × 7 et 9 × 4).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Tables de multiplication de 6, 7, 8 et 9 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
