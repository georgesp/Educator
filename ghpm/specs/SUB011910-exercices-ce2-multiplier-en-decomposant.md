---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZHY
rev: 2026-10-11T18:49:40Z
url: https://github.com/georgesp/Educator/issues/132
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011910 — Exercices CE2 — Multiplier en décomposant
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1130
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à multiplier un nombre à deux chiffres par un nombre à un chiffre en décomposant afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2, calcul mental : produit d'un nombre entre 11 et 99 par un nombre inférieur à 10 en décomposant le plus grand facteur (ex. 13 × 7 = 10 × 7 + 3 × 7 = 91).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-mul-distributive` | 3 | Multiplier en décomposant | a × b avec a de 11 à 99 et b de 2 à 9, ordre au hasard ; d'abord a ≤ 30 puis toute la plage (deux compétences si besoin) | `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 13 × 7 = ? → 91.
- 6 × 24 = ? → 144.
- 45 × 3 = ? → 135.

## Évolutions du moteur et de l'écran de série

- Aucun après l'opération `multiplication`.
- Après une erreur, l'aide affiche la décomposition (10 × 7 + 3 × 7) avec l'animation `arrayGrid` (lien avec la difficulté adaptative du lot 3).

## Distracteurs (QCM)

Oubli d'une partie (70 au lieu de 91), seulement les unités multipliées (13 × 7 → 1 21), somme des produits erronée.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Multiplier en décomposant » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
