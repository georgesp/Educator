---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZpo
rev: 2026-10-11T18:50:36Z
url: https://github.com/georgesp/Educator/issues/158
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012004 — Exercices CM1 — Comparer, ranger et arrondir les nombres décimaux"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1520
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à comparer, ranger, placer, encadrer, intercaler et arrondir des nombres décimaux afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : placer, comparer, encadrer, intercaler, ordonner des décimaux ; donner la partie entière et l'arrondi à l'unité.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-dec-compare` | 3 | Comparer, ranger et placer des décimaux | Décimaux jusqu'aux centièmes, paires pièges (2,5 et 2,45 ; 3,1 et 3,09) ; placer et repérer un décimal sur une demi-droite graduée en dixièmes ou en centièmes | `compare`, `order`, `numberLinePoint` |
| `cm1-dec-round` | 3 | Encadrer, intercaler et arrondir à l'unité | Encadrer entre deux entiers consécutifs, intercaler un décimal entre deux décimaux (2,4 < … < 2,5), arrondir à l'unité la plus proche | `frame`, `between`, `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 2,5 … 2,45 → >.
- Arrondi à l'unité de 7,62 → 8.
- Encadre 12,3 entre deux entiers → 12 < 12,3 < 13.
- Intercale un nombre entre 2,4 et 2,5 → 2,45 (toute réponse juste est acceptée).
- Place 3,7 sur la demi-droite graduée de 3 à 4.

## Évolutions du moteur et de l'écran de série

- Formats `compare`, `order`, `frame` pour les décimaux ; le générateur produit des paires pièges (plus de chiffres ≠ plus grand).
- `numberLinePoint` gradué en dixièmes et centièmes.
- Format `between` : toute réponse strictement comprise entre les deux bornes, écrite au plus aux centièmes, est acceptée.

## Distracteurs (QCM)

« Plus de chiffres = plus grand » (2,45 > 2,5), arrondi par troncature (7,62 → 7), nombre intercalé hors de l'intervalle (2,6 entre 2,4 et 2,5).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Comparer, ranger et arrondir les nombres décimaux » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
