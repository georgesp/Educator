---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACab8
rev: 2026-10-11T18:51:26Z
url: https://github.com/georgesp/Educator/issues/200
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012112 — Exercices CM2 — Résoudre des problèmes
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#121"
order: 1870
---

## User story

En tant qu'enfant de CM2, je veux m'entraîner à résoudre des problèmes complexes en plusieurs étapes afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM2 : quatre phases et au moins 10 problèmes par semaine ; additifs en une ou plusieurs étapes, multiplicatifs parties-tout, mixtes en plusieurs étapes, comparaison multiplicative, dénombrement, optimisation, problèmes préparant aux algorithmes ; entiers jusqu'à 999 999 999, décimaux et fractions.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm2-problems-multi-step` | 1 | Problèmes en plusieurs étapes | Gabarits mixtes en 2 ou 3 étapes avec entiers, décimaux (prix, durées) et fractions | `problem` |
| `cm2-problems-optimization` | 5 | Dénombrement, optimisation et algorithmes | Combinaisons, meilleur choix, suites d'instructions (déplacer la voiture sur une grille) | `problem` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Un pilote fait 12 tours de 3,45 km puis 2/3 d'un tour. Quelle distance ?
- Pack de 4 pneus à 210 € ou pneu à 56 € : le moins cher pour 10 pneus ?

## Évolutions du moteur et de l'écran de série

- Gabarits de problèmes avec décimaux et fractions ; problèmes « algorithmes » sur grille (réutilise `arrayGrid`).

## Distracteurs (QCM)

Étape oubliée, unité oubliée ou erronée, résultat intermédiaire.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM2 — Résoudre des problèmes » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
