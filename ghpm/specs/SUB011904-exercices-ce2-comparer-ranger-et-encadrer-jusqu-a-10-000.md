---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACY7E
rev: 2026-10-11T18:49:13Z
url: https://github.com/georgesp/Educator/issues/126
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011904 — Exercices CE2 — Comparer, ranger et encadrer jusqu'à 10 000"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 950
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à comparer, ranger et placer des nombres jusqu'à 10 000 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2 : comparer, encadrer, intercaler, ordonner jusqu'à cinq nombres (ex. 6 234, 6 243, 6 239, 6 300, 5 229).
- Placer des nombres sur une demi-droite graduée de 1 en 1, 10 en 10, 100 en 100 ou 1 000 en 1 000.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-num-compare` | 2 | Comparer et ranger les nombres jusqu'à 10 000 | Deux nombres de 1 000 à 10 000 → `<`, `>` ou `=` ; ranger 3 à 5 nombres proches (mêmes chiffres de tête) | `compare`, `order` |
| `ce2-num-frame` | 3 | Encadrer et placer les nombres jusqu'à 10 000 | Encadrer à la dizaine, à la centaine, au millier ; lire la graduation d'une demi-droite (pas de 10, 100, 1 000) | `frame`, `numberLinePoint` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 6 234 … 6 243 → <.
- Range du plus petit au plus grand : 6 239, 6 300, 5 229.
- Encadre 4 635 entre deux milliers : 4 000 < 4 635 < 5 000.
- Quel nombre est indiqué par la flèche ? (graduation de 100 en 100).

## Évolutions du moteur et de l'écran de série

- Formats `compare`, `order`, `frame` et `numberLinePoint` créés au lot 1 (fiche « Exercices CE1 — Comparer, ranger et placer les nombres jusqu'à 1 000 »), plages étendues jusqu'à 10 000 et graduations de 1 000 en 1 000.
- Le générateur choisit des nombres proches (mêmes chiffres de tête) pour que la comparaison porte sur les bons rangs.

## Distracteurs (QCM)

Signe inversé ; ordre selon le dernier chiffre ; encadrement à la mauvaise unité.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Comparer, ranger et encadrer jusqu'à 10 000 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001612 (comparaison) : partie CE2.
