---
type: Issue
title: Exercices CE2 — Ajouter 8, 9, 18, 19, 28, 29, 38 ou 39
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-1-ce2.md"
order: 1350
---
## User story

En tant qu'enfant de CE2, je veux m'entraîner à ajouter de tête 8, 9, 18, 19, 28, 29, 38 ou 39 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2, calcul mental : ajouter 8, 9, 18, 19, 28, 29, 38 ou 39 (ex. ajouter 40 puis retirer 2 pour 38).
- Fluence attendue en fin de CE2 : 15 résultats en 3 minutes, nombres ≤ 10 000.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-add-near-tens` | 1 | Ajouter 8, 9, 18, 19, 28, 29, 38 ou 39 | Premier nombre de 20 à 999, second dans {8, 9, 18, 19, 28, 29, 38, 39} | `result`, `missingOperand` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 146 + 38 = ? → 184.
- 57 + 19 = ? → 76.
- 235 + … = 264 → 29.

## Évolutions du moteur et de l'écran de série

- Aucun : `operands[].values` existe déjà.

## Distracteurs (QCM)

Erreur de compensation (+ 2 au lieu de − 2 : 188), oubli de la compensation (186), erreur d'une dizaine.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] Les tables d'addition du CE1 (`ce1-add-tables`) sont aussi proposées au CE2 en période 1 (`alsoOffered`), pour la fluence attendue en fin de CE2 (15 égalités à trou en une minute).
- [ ] La compétence est liée à la fiche « Leçon CE2 — Ajouter 8, 9, 18, 19, 28, 29, 38 ou 39 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
