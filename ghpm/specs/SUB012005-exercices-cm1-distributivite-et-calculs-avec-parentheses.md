---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZq0
rev: 2026-10-11T18:50:19Z
url: https://github.com/georgesp/Educator/issues/159
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012005 — Exercices CM1 — Distributivité et calculs avec parenthèses
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1400
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à décomposer un produit et calculer avec des parenthèses afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : utiliser la distributivité dans des cas simples ; calculer avec des parenthèses ; estimer un résultat.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-distributive` | 4 | Multiplier en décomposant | a × b avec a de 11 à 99 et b de 2 à 9, puis 101 à 999 × 2 à 5 ; aide par décomposition | `result` |
| `cm1-parentheses` | 4 | Calculer avec des parenthèses | Expressions à 3 nombres avec une paire de parenthèses : (a + b) × c, a × (b − c), a − (b + c), nombres ≤ 100 | `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 23 × 6 = ? → 138.
- (12 + 8) × 5 = ? → 100.
- 50 − (18 + 7) = ? → 25.

## Évolutions du moteur et de l'écran de série

- Format `expression` : le générateur produit une expression avec parenthèses (arbre de calcul dans `educator-core`) et sa valeur ; affichage et lecture Piper (« ouvrez la parenthèse » non lu, structure annoncée « la somme de 12 et 8, multipliée par 5 »).

## Distracteurs (QCM)

Calcul de gauche à droite sans tenir compte des parenthèses, oubli d'une partie de la décomposition.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Distributivité et calculs avec parenthèses » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
