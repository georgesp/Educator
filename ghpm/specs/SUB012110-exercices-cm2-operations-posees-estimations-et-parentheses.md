---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaac
rev: 2026-10-12T19:26:22Z
url: https://github.com/georgesp/Educator/issues/198
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012110 — Exercices CM2 — Opérations posées, estimations et parenthèses"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 1710
---

## User story

En tant qu'enfant de CM2, je veux m'entraîner à poser les opérations sur les entiers, estimer un résultat et calculer avec des parenthèses afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM2 : opérations posées utilisées dès le début de l'année selon les nombres en jeu ; estimer ; calculer avec une ou deux paires de parenthèses.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm2-column-integers` | 1 | Opérations posées sur les entiers | +, − sur des nombres ≤ 999 999 999 ; × de nombres à 3 ou 4 chiffres par 2 ou 3 chiffres | `result`, affichage posé |
| `cm2-estimate` | 2 | Estimer un ordre de grandeur | Choisir l'ordre de grandeur d'un calcul parmi 3 propositions (498 × 21 ≈ 10 000) | `choice` |
| `cm2-parentheses` | 3 | Calculs avec parenthèses | Expressions à 3 ou 4 nombres avec une ou deux paires de parenthèses | `expression` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 4 058 × 307 = ?
- 498 × 21 ≈ ? (100, 1 000, 10 000)
- (45 − 15) × (3 + 7) = ? → 300.

## Évolutions du moteur et de l'écran de série

- Format `expression` (lot 5) étendu à deux paires de parenthèses.

## Distracteurs (QCM)

Zéro du multiplicateur mal géré (4 058 × 307), parenthèses ignorées, ordre de grandeur décalé d'un facteur 10.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM2 — Opérations posées, estimations et parenthèses » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
