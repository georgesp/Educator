---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaSQ
rev: 2026-10-12T19:26:20Z
url: https://github.com/georgesp/Educator/issues/190
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012102 — Exercices CM2 — Calculer avec les décimaux
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#121"
order: 1810
---

## User story

En tant qu'enfant de CM2, je veux m'entraîner à calculer avec des nombres décimaux, de tête et posé afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM2, calcul mental : ajouter un entier à un décimal avec ou sans retenue ; multiplier et diviser un décimal par 10, 100 ou 1 000 ; ajouter deux décimaux inférieurs à 10 ayant au plus un chiffre après la virgule ; double et moitié d'un décimal (cas simples) ; multiplier un décimal par 5 ou par 50.
- Opérations : multiplier un décimal par un entier.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm2-dec-mental` | 1 | Calcul mental sur les décimaux | d + n, d1 + d2 (< 10, un chiffre après la virgule), double / moitié (2,4 → 4,8 / 1,2) | `result`, `phrase` |
| `cm2-dec-mul-div-powers` | 2 | Multiplier et diviser par 10, 100, 1 000 | d × 10ⁿ et d ÷ 10ⁿ (n = 1 à 3), jusqu'aux millièmes | `result`, `missingOperand` |
| `cm2-dec-mul-5-50` | 3 | Multiplier un décimal par 5 ou par 50 | d × 5 et d × 50 (d jusqu'aux dixièmes) | `result` |
| `cm2-dec-mul-int` | 3 | Multiplier un décimal par un entier (posé) | d (jusqu'aux centièmes) × entier de 2 à 99 | `result`, affichage posé |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 3,7 + 5,6 = ? → 9,3.
- 4,5 ÷ 100 = ? → 0,045.
- 2,4 × 50 = ? → 120.
- 12,35 × 24 = ? → 296,4.

## Évolutions du moteur et de l'écran de série

- Aucun après le lot 5 (type décimal, composant posé avec virgules).

## Distracteurs (QCM)

Virgule déplacée dans le mauvais sens, zéros mal placés (0,45), virgule du produit mal placée.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM2 — Calculer avec les décimaux » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
