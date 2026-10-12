---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZoE
url: https://github.com/georgesp/Educator/issues/157
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012003 — Exercices CM1 — Comparer, ranger et encadrer les grands nombres"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1730
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à comparer, ranger, encadrer et placer de grands nombres afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : comparer, ordonner, encadrer, placer sur une demi-droite graduée des nombres d'au plus six chiffres.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-num-compare` | 4 | Comparer et ranger les grands nombres | Deux à cinq nombres de 5 ou 6 chiffres, proches (mêmes chiffres de tête) | `compare`, `order` |
| `cm1-num-frame` | 4 | Encadrer et placer les grands nombres | Encadrer au millier, à la dizaine de mille, à la centaine de mille ; lire une graduation | `frame`, `numberLinePoint` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 452 310 … 452 301 → >.
- Encadre 538 214 entre deux dizaines de mille : 530 000 < 538 214 < 540 000.

## Évolutions du moteur et de l'écran de série

- Formats `compare`, `order`, `frame`, `numberLinePoint` du CE2, plages étendues.

## Distracteurs (QCM)

Comparaison sur le dernier chiffre, encadrement à la mauvaise unité.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Comparer, ranger et encadrer les grands nombres » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
