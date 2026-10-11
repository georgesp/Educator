---
type: Issue
title: Exercices CP — Doubles et moitiés
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-4-cp.md"
order: 2570
---
## User story

En tant qu'enfant de CP, je veux m'entraîner à connaître les doubles et les moitiés afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CP, faits numériques : doubles de 1 à 10 et de 20, 30, 40, 50 ; moitiés des nombres pairs de 2 à 20 et de 40, 60, 80, 100 (8 en une minute en fin de CP).
- Procédure de calcul mental : moitié d'un nombre pair (46 → 40 + 6 → 20 + 3 = 23).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cp-doubles-halves` | 3 | Doubles et moitiés à connaître | Doubles de 1 à 10 et de 20 à 50 ; moitiés des pairs de 2 à 20 et de 40, 60, 80, 100 | `phrase`, `result` |
| `cp-half-even` | 5 | Moitié d'un nombre pair | Moitié d'un nombre pair ≤ 100 dont les dizaines et les unités sont paires (46, 84) ; puis tous les pairs ≤ 60 | `phrase` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Le double de 7 → 14.
- La moitié de 16 → 8.
- La moitié de 46 → 23.

## Évolutions du moteur et de l'écran de série

- Format `phrase` (lot 1, CE1) avec des consignes adaptées au CP, lues par Piper.

## Distracteurs (QCM)

Double et moitié confondus, + 2 au lieu de × 2.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CP — Doubles et moitiés » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
