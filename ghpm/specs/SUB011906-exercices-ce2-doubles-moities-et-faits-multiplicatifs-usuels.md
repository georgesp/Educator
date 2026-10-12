---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACY_I
url: https://github.com/georgesp/Educator/issues/128
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011906 — Exercices CE2 — Doubles, moitiés et faits multiplicatifs usuels"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1450
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à connaître les doubles, les moitiés et les multiples de 25 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2, faits numériques : doubles de 1 à 20 et de nombres usuels (ex. 75, 600), moitiés des nombres pairs de 2 à 40, multiples de 25 (1 × 25 à 4 × 25), décompositions multiplicatives de 60.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-double-half` | 2 | Doubles et moitiés | Doubles de 1 à 20 et de nombres usuels (25, 50, 75, 100, 150, 200…, 600) ; moitiés des pairs de 2 à 40 et de ces nombres | `phrase` (« le double de … »), `result` |
| `ce2-mul-facts-25-60` | 3 | Multiples de 25 et décompositions de 60 | 1 × 25 à 4 × 25 ; 60 = 1 × 60 = 2 × 30 = 3 × 20 = 4 × 15 = 5 × 12 = 6 × 10 | `result`, `missingOperand` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Le double de 17 → 34.
- La moitié de 36 → 18.
- 3 × 25 = ? → 75.
- 60 = 4 × … → 15.

## Évolutions du moteur et de l'écran de série

- Format `phrase` (lot 1, CE1) : consigne en mots générée (« Quel est le double de 17 ? »).
- Liste des nombres usuels du programme dans le fichier de la compétence (`operands[].values`).

## Distracteurs (QCM)

Double et moitié confondus, + 2 au lieu de × 2, moitié d'un nombre à deux chiffres prise chiffre par chiffre (36 → 13).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Doubles, moitiés et faits multiplicatifs usuels » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
