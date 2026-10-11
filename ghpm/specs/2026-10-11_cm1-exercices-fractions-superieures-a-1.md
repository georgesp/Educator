---
type: Issue
title: "Exercices CM1 — Les fractions : écrire, placer et dépasser 1"
status: Todo
size: L
parent: "specs/2026-10-11_lot-1-2-cm1.md"
order: 1890
---
## User story

En tant qu'enfant de CM1, je veux m'entraîner à écrire, placer et décomposer des fractions, y compris supérieures à 1 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : fractions dès la période 1 ; extension aux fractions supérieures à 1 ; dénominateurs ≤ 20 (sauf fractions décimales, dénominateur 100).
- Écrire, lire, encadrer entre deux entiers, placer sur une demi-droite graduée ; écrire une fraction comme somme d'un entier et d'une fraction inférieure à 1.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-frac-read-place` | 1 | Lire, écrire et placer des fractions | a/b avec b ≤ 20, a ≤ 3b ; repérer sur une demi-droite graduée | `fractionWords`, `numberLinePoint` |
| `cm1-frac-integer-part` | 2 | Fractions plus grandes que 1 | a/b > 1 ↔ entier + fraction < 1 (7/4 = 1 + 3/4) ; encadrer entre deux entiers | `decompose`, `frame` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 7/4 = 1 + …/4 → 3.
- Entre quels entiers se trouve 11/3 ? → 3 et 4.
- Quelle fraction repère le point A ? → 5/4.

## Évolutions du moteur et de l'écran de série

- Fractions supérieures à 1 et dénominateurs jusqu'à 20 dans le type fraction (lot 1, CE1) ; `numberLinePoint` gradué en fractions.

## Distracteurs (QCM)

Entier et numérateur confondus (7/4 = 7 + 1/4), mauvais entier d'encadrement.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Les fractions : écrire, placer et dépasser 1 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
