---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXvGs
url: https://github.com/georgesp/Educator/issues/320
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012041 — Exercices CM1 — Périmètres et aires
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 5170
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à calculer le périmètre d'un polygone et à trouver une aire sur un quadrillage afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1, longueurs : déterminer le périmètre d'un polygone (mesures à la règle graduée).
- Aires : comparer des aires sans les mesurer, puis les déterminer avec une unité et un quadrillage, en centimètres carrés ; aucune formule à mémoriser au CM1.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-perimeter` | 4 | Périmètre d'un polygone | Triangle, carré, rectangle, pentagone : côtés cotés en cm ou en m, ou lus sur une règle graduée affichée ; périmètre = somme des côtés, sans formule ; problèmes (tour d'un circuit rectangulaire) | `result`, `problem` |
| `cm1-area-grid` | 5 | Aires sur quadrillage | Figure sur quadrillage : compter les carreaux (unité : le carreau, puis le cm²) et les demi-carreaux ; comparer deux aires ; deux figures de même aire et de périmètres différents | `result`, `compare`, `choice` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Un circuit rectangulaire mesure 12 m sur 7 m. Quel est son périmètre ? → 38 m.
- Un triangle a des côtés de 5 cm, 4 cm et 6 cm. Son périmètre ? → 15 cm.
- Combien de carreaux mesure l'aire de la figure ? (12 carreaux et 4 demi-carreaux) → 14.
- Quelle figure a la plus grande aire ?

## Évolutions du moteur et de l'écran de série

- Figures SVG générées à partir de la compétence : polygone coté (longueurs des côtés), figure sur quadrillage (liste de carreaux et de demi-carreaux), dessinées avec les variables du thème.
- Règle graduée affichée pour lire une longueur ; jamais de mesure de l'écran réel (l'échelle de l'écran n'est pas garantie).
- Propriété testée : les figures générées sont fermées, sans carreau isolé, et l'aire attendue correspond au compte des carreaux.

## Distracteurs (QCM)

Aire et périmètre confondus (12 × 7 = 84), côté oublié (12 + 7 = 19 pour le rectangle), demi-carreaux comptés comme des carreaux entiers, « même aire donc même périmètre ».

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Périmètres et aires » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
