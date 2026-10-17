---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACazE
rev: 2026-10-17T13:46:41Z
url: https://github.com/georgesp/Educator/issues/218
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012206 — Exercices CP — Doubles et moitiés
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 1980
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
| `cp-half-even` | 5 | Moitié d'un nombre pair | Moitié d'un nombre pair ≤ 100 qui se décompose en moitiés connues : dizaines paires et unités paires (46 → 40 + 6 → 20 + 3 = 23 ; 84), puis dizaine paire et reste pair ≤ 20 (52 → 40 + 12 → 20 + 6 = 26 ; 38 → 20 + 18 → 10 + 9 = 19). Jamais la moitié d'une dizaine impaire seule (moitié de 30 = 15), qui n'est pas un fait à connaître au CP | `phrase` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Le double de 7 → 14.
- La moitié de 16 → 8.
- La moitié de 46 → 23.
- La moitié de 52 → 26 (40 + 12 → 20 + 6).

## Évolutions du moteur et de l'écran de série

- Format `phrase` (lot 1, CE1) avec des consignes adaptées au CP, lues par Piper.

## Distracteurs (QCM)

Double et moitié confondus, + 2 au lieu de × 2, moitié des seules dizaines (moitié de 46 → 20).

## Critères d'acceptation

- [x] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [x] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [x] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md` (nouveau fichier ; les 11 compétences du lot 1 restent dans `echantillons-lot1.md`), relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement (décision du 2026-10-15).
- [ ] Exercices jouables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [x] La compétence est liée à la fiche « Leçon CP — Doubles et moitiés » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Remarques et décisions

- Implémentation (2026-10-17) : au CP, la correction décompose en moitiés connues (52 = 40 + 12) ; le CE1 garde la décomposition par rang (56 = 50 + 6).
