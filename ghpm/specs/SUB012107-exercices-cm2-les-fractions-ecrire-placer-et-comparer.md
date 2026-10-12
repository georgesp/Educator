---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaXk
url: https://github.com/georgesp/Educator/issues/195
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012107 — Exercices CM2 — Les fractions : écrire, placer et comparer"
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#121"
order: 2210
---

## User story

En tant qu'enfant de CM2, je veux m'entraîner à écrire, encadrer, placer et comparer des fractions afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM2 : fractions dès la période 1 ; dénominateurs ≤ 60 (sauf fractions décimales : 100 ou 1 000) ; écrire, encadrer, placer, comparer.
- Livret d'accompagnement CM2 : écrire une fraction supérieure à 1 comme somme d'un entier et d'une fraction inférieure à 1, et inversement ; placer sur une demi-droite graduée.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm2-frac-place-frame` | 1 | Placer et encadrer des fractions | a/b, b ≤ 60 ; entier + fraction ↔ fraction ; encadrer entre deux entiers ; repérer sur une demi-droite | `decompose`, `frame`, `numberLinePoint` |
| `cm2-frac-compare` | 2 | Comparer des fractions | Dénominateurs multiples l'un de l'autre, ou comparaison à 1 ou à 1/2 | `compare`, `order` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 23/6 = 3 + …/6 → 5.
- 5/12 … 1/2 → <.
- 2 + 3/8 = …/8 → 19.

## Évolutions du moteur et de l'écran de série

- Dénominateurs jusqu'à 60 et fractions décimales en millièmes dans le type fraction.

## Distracteurs (QCM)

Comparaison sur les dénominateurs seuls, entier mal converti (2 + 3/8 = 5/8).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM2 — Les fractions : écrire, placer et comparer » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
