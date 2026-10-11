---
type: Issue
title: Exercices CP — L'addition posée
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-4-cp.md"
order: 2590
---
## User story

En tant qu'enfant de CP, je veux m'entraîner à poser une addition en colonnes afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CP : addition posée en colonnes (ex. 45 + 37 ou 28 + 8 + 56), introduite en période 4 ou 5.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cp-add-column` | 5 | Additions posées | Deux ou trois termes ≤ 99, résultat ≤ 100, avec ou sans retenue | `result`, affichage posé |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 45 + 37 = ? → 82.
- 28 + 8 + 56 = ? → 92.

## Évolutions du moteur et de l'écran de série

- Composant d'opération posée (lot 1.1) dans une version CP : grandes cases, une colonne à la fois, retenue pré-dessinée en pointillé.

## Distracteurs (QCM)

Oubli de la retenue (72), chiffres mal alignés (8 sous les dizaines).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CP — L'addition posée » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
