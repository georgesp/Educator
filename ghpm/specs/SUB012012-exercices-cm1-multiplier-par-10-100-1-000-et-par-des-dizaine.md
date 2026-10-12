---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZz4
rev: 2026-10-11T18:50:13Z
url: https://github.com/georgesp/Educator/issues/166
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012012 — Exercices CM1 — Multiplier par 10, 100, 1 000 et par des dizaines ou des centaines"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1360
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à multiplier de tête par 10, 100, 1 000 et par des dizaines ou des centaines afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1, calcul mental : multiplier un entier par 10, 100 ou 1 000 ; multiplier un entier inférieur à 10 par un nombre de dizaines ou de centaines.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-mul-10-100-1000` | 1 | Multiplier par 10, 100 et 1 000 | n × 10, × 100, × 1 000 avec résultat ≤ 999 999 ; facteur manquant | `result`, `missingOperand` |
| `cm1-mul-tens-hundreds` | 2 | Multiplier par des dizaines ou des centaines | a × b0 et a × b00 avec a et b de 2 à 9 (7 × 300, 60 × 8) | `result`, `missingOperand` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 38 × 1 000 = ? → 38 000.
- 7 × 300 = ? → 2 100.
- … × 100 = 4 500 → 45.

## Évolutions du moteur et de l'écran de série

- Aucun : opération `multiplication` créée au lot 1 (CE1).

## Distracteurs (QCM)

Nombre de zéros erroné, zéros de 7 × 300 oubliés (21), zéro de 5 × 400 « en trop » mal compris (200 au lieu de 2 000).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Multiplier par 10, 100, 1 000 et par des dizaines ou des centaines » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
