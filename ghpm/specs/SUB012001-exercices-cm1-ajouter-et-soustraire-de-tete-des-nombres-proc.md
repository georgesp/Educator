---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZlk
rev: 2026-10-11T18:50:09Z
url: https://github.com/georgesp/Educator/issues/155
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012001 — Exercices CM1 — Ajouter et soustraire de tête des nombres proches d'une dizaine
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1340
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à ajouter ou soustraire de tête 8, 9, 18, 19, 28, 29, 38 ou 39 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1, calcul mental : ajouter ou soustraire 8, 9, 18, 19, 28, 29, 38 ou 39 à un nombre ; les procédures du cycle 2 sont réemployées.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-add-sub-near-tens` | 1 | Ajouter et soustraire 8, 9, 18, 19, 28, 29, 38 ou 39 | Premier nombre de 100 à 9 999, second dans la liste, opération mixte | `result`, `missingOperand` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 2 347 + 38 = ? → 2 385.
- 1 506 − 29 = ? → 1 477.
- … − 18 = 482 → 500.

## Évolutions du moteur et de l'écran de série

- Aucun : `operation: "mixed"` et `operands[].values` existent.

## Distracteurs (QCM)

Compensation dans le mauvais sens, oubli de la compensation, erreur de retenue sur les centaines.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Ajouter et soustraire de tête des nombres proches d'une dizaine » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
