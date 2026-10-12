---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACX1M
rev: 2026-10-11T18:48:49Z
url: https://github.com/georgesp/Educator/issues/86
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB000118 — Exercices CE1 — Comparer, ranger et placer les nombres jusqu'à 1 000"
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 1210
---

## User story

En tant qu'enfant de CE1, je veux m'entraîner à comparer, ranger, encadrer et placer les nombres jusqu'à 1 000 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE1 : comparer, encadrer et intercaler des nombres (=, <, >), les ordonner et les placer sur une demi-droite graduée.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce1-num-compare` | 2 | Comparer et ranger les nombres jusqu'à 1 000 | Deux nombres ≤ 1 000 → `<`, `>`, `=` ; ranger 3 à 5 nombres proches (mêmes chiffres, ordre différent : 438, 483, 348) | `compare`, `order` |
| `ce1-num-frame-line` | 3 | Encadrer et placer sur une droite graduée | Encadrer à la dizaine ou à la centaine ; lire ou placer un nombre sur une demi-droite graduée de 1 en 1, de 10 en 10 ou de 100 en 100 | `frame`, `numberLinePoint` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 438 … 483 → <.
- Range : 483, 348, 438.
- Encadre 347 entre deux centaines : 300 < 347 < 400.
- Quel nombre sous la flèche ? (graduation de 10 en 10)

## Évolutions du moteur et de l'écran de série

- Formats `compare` (trois gros boutons `<`, `>`, `=`), `order` (glisser-déposer, SUB001606), `frame` (deux champs), `numberLinePoint` (droite graduée dans l'exercice, réutilise `numberLine`) — créés ici, réutilisés par les lots 1.4, 4, 5 et 6.
- Le générateur choisit des nombres proches pour que la comparaison porte sur le bon rang.

## Distracteurs (QCM)

Signe inversé, comparaison sur le chiffre des unités, encadrement à la mauvaise unité.

## Critères d'acceptation

- [x] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [x] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [x] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE1 — Comparer, ranger et placer les nombres jusqu'à 1 000 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001612 (comparaison) et SUB001606 (ranger des nombres, glisser-déposer).

## Réalisation (2026-10-11)

- `ce1-num-compare` : nombres de 100 à 999, réponse en mode clavier seulement (trois gros boutons `<`, `=`, `>` et glisser-déposer, avec de petites flèches pour qui ne glisse pas encore) : la compétence ouvre directement la série.
- `ce1-num-frame-line` : encadrement à la dizaine ou à la centaine ; droite graduée de 11 graduations (de 1 en 1, de 10 en 10, de 100 en 100), lire le nombre sous la flèche ou cliquer la bonne graduation.
- Restent ouverts : relecture des échantillons (`docs/specs/echantillons-ce1.md`), lecture par Piper (voix coupée dans les maths pour l'instant), lien avec la leçon (format des leçons au lot 2).
