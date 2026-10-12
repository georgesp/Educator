---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaUU
rev: 2026-10-11T18:51:15Z
url: https://github.com/georgesp/Educator/issues/192
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012104 — Exercices CM2 — Les nombres décimaux jusqu'aux millièmes
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 1790
---

## User story

En tant qu'enfant de CM2, je veux m'entraîner à lire, écrire, comparer et arrondir des décimaux jusqu'aux millièmes afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM2 : étude étendue aux millièmes ; décimaux rencontrés dès la période 1 ; lire, écrire, passer d'une écriture à l'autre, placer, comparer, ordonner, partie entière, arrondi à l'unité.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm2-dec-read-write` | 1 | Écrire les décimaux jusqu'aux millièmes | Fractions décimales (/10, /100, /1 000) ↔ écriture à virgule ; décomposition ; chiffre d'un rang | `decompose`, `recompose`, `placeValue` |
| `cm2-dec-compare-round` | 2 | Comparer, ranger, arrondir | Décimaux jusqu'aux millièmes, paires pièges ; encadrer, arrondir à l'unité, au dixième | `compare`, `order`, `frame`, `result` |
| `cm2-dec-usual-facts` | 4 | Fractions usuelles, décimaux et moitiés | Écritures décimales des fractions usuelles (1/5 = 0,2 ; 3/4 = 0,75 ; 1/8 = 0,125) ; moitié des nombres impairs jusqu'à 15 (7 → 3,5 ; 15 → 7,5) | `result`, `phrase` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 4 + 7/100 + 3/1 000 = ? → 4,073.
- 0,7 … 0,689 → >.
- Arrondi au dixième de 3,456 → 3,5.
- La moitié de 15 → 7,5.
- 1/5 = ? → 0,2.

## Évolutions du moteur et de l'écran de série

- Type décimal étendu aux millièmes ; lecture Piper des millièmes.

## Distracteurs (QCM)

4,73 au lieu de 4,073 ; « plus de chiffres = plus grand » ; arrondi par troncature.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM2 — Les nombres décimaux jusqu'aux millièmes » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
