---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZ1U
url: https://github.com/georgesp/Educator/issues/167
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012013 — Exercices CM1 — Multiplier par 4, par 8 et par 5"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1810
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à multiplier de tête par 4, 8 et 5 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1, calcul mental : multiplier un entier par 4 ou par 8 ; multiplier un entier par 5.
- Livret d'accompagnement CM1 (2026) : séquence « multiplier par 5 un entier inférieur à 200 » (ex. 5 × 64, 5 × 186).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-mul-by-4-8` | 2 | Multiplier par 4 ou par 8 | n × 4 (n ≤ 500) et n × 8 (n ≤ 250) | `result` |
| `cm1-mul-by-5` | 3 | Multiplier par 5 | 5 × n avec n de 11 à 199, hors faits connus (5 × 20, 5 × 25, 5 × 100) ; ordre au hasard | `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 5 × 64 = ? → 320.
- 186 × 5 = ? → 930.
- 45 × 8 = ? → 360.

## Évolutions du moteur et de l'écran de série

- Règle d'exclusion de valeurs (`excludeValues`) dans la compétence pour écarter les faits connus, comme le préconise le livret.

## Distracteurs (QCM)

Moitié prise avant le × 10 sur un nombre impair mal gérée (5 × 139), × 4 calculé comme un seul double.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Multiplier par 4, par 8 et par 5 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
