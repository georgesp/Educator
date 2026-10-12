---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACX4k
rev: 2026-10-12T19:49:11Z
url: https://github.com/georgesp/Educator/issues/88
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000120 — Exercices CE1 — Multiplier en décomposant
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 750
---

## User story

En tant qu'enfant de CE1, je veux m'entraîner à multiplier de tête un nombre de 11 à 19 par un petit nombre afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE1, calcul mental : produit d'un nombre de 11 à 19 par un nombre inférieur à 10 en décomposant le plus grand facteur (ex. 13 × 7 = 70 + 21).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce1-mul-distributive` | 5 | Multiplier en décomposant | a × b avec a de 11 à 19 et b de 2 à 9, ordre au hasard ; d'abord b ≤ 5 | `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 13 × 7 = ? → 91.
- 4 × 16 = ? → 64.
- 12 × 5 = ? → 60.

## Évolutions du moteur et de l'écran de série

- Après une erreur, l'aide montre la décomposition (10 × 7 + 3 × 7) avec `arrayGrid`.

## Distracteurs (QCM)

Oubli d'une partie (70 au lieu de 91), seules les unités multipliées, mauvaise somme des produits.

## Critères d'acceptation

- [x] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [x] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [x] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE1 — Multiplier en décomposant » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Réalisation (2026-10-11)

- Après une erreur : la décomposition dans l'ordre affiché (`7 × 13 = 7 × 10 + 7 × 3 = 70 + 21 = 91`) et le quadrillage coupé après 10 colonnes ; exercices du plus facile (× 2, × 5) au plus difficile (× 6 à × 9).
- Restent ouverts : relecture des échantillons (`docs/specs/echantillons-ce1.md`), lecture par Piper (voix coupée dans les maths pour l'instant), lien avec la leçon (format des leçons au lot 2).
