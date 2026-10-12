---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaTY
url: https://github.com/georgesp/Educator/issues/191
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012103 — Exercices CM2 — Calcul mental sur les entiers
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 2150
---

## User story

En tant qu'enfant de CM2, je veux m'entraîner à calculer de tête avec des nombres proches d'une dizaine et avec des dizaines, centaines et milliers afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM2, calcul mental : ajouter ou soustraire 8, 9, 18, 19, 28, 29, …, 98 ou 99 ; multiplier des entiers inférieurs à 10 de dizaines, centaines ou milliers ; utiliser la distributivité ; diviser un entier par 4 ou par 8.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm2-add-sub-near-tens` | 1 | Ajouter et soustraire 8, 9, 18, …, 98 ou 99 | Premier nombre ≤ 99 999, second dans {8, 9, 18, 19, …, 98, 99}, opération mixte | `result`, `missingOperand` |
| `cm2-mul-round` | 1 | Multiplier des dizaines, des centaines, des milliers | a0 × b00, a × b 000… avec a, b de 2 à 9 (30 × 400, 6 × 7 000) | `result` |
| `cm2-div-by-4-8` | 2 | Diviser par 4 ou par 8 | n ÷ 4 et n ÷ 8 exacts, n ≤ 2 000 | `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 4 562 + 98 = ? → 4 660.
- 30 × 400 = ? → 12 000.
- 360 ÷ 8 = ? → 45.

## Évolutions du moteur et de l'écran de série

- Aucun après les lots 4 et 5.

## Distracteurs (QCM)

Compensation inversée, nombre de zéros erroné, une seule moitié prise pour ÷ 4.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM2 — Calcul mental sur les entiers » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
