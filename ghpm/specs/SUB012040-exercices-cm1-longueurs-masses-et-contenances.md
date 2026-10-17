---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXvEQ
url: https://github.com/georgesp/Educator/issues/319
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012040 — Exercices CM1 — Longueurs, masses et contenances"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 5160
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à choisir la bonne unité, comparer des mesures et passer d'une unité à l'autre afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : longueurs du millimètre au kilomètre ; masses du milligramme au kilogramme, et la tonne ; contenances du millilitre à l'hectolitre.
- Choisir l'unité adaptée, comparer, estimer des mesures ; les mesures sont réinvesties dans la résolution de problèmes, notamment de proportionnalité.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-measure-units` | 5 | Choisir l'unité, estimer et comparer | Unité adaptée à un objet ou une distance (QCM), ordre de grandeur réaliste (masse d'une voiture ≈ 1 t, d'un casque ≈ 1 kg), comparer deux mesures de même unité ou avec 1 km = 1 000 m, 1 kg = 1 000 g, 1 L = 100 cL | `choice`, `compare` |
| `cm1-measure-convert` | 5 | Passer d'une unité à l'autre | Nombres entiers : km ↔ m, m ↔ cm, cm ↔ mm, t ↔ kg, kg ↔ g, g ↔ mg, hL ↔ L, L ↔ cL ↔ mL ; mesures composées (2 m 45 cm = 245 cm, 4 500 g = 4 kg 500 g) ; résultat ≤ 999 999 | `result`, `missingOperand`, `compare` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Quelle unité pour la longueur d'un circuit : mm, m ou km ? → km.
- 3 km … 2 900 m → >.
- 2 m 45 cm = … cm → 245.
- 1 t = … kg → 1 000.
- Combien de verres de 25 cL remplit une bouteille de 1 L ? → 4.

## Évolutions du moteur et de l'écran de série

- Unité affichée après le champ de réponse ; réponse en deux champs pour les mesures composées (m et cm, kg et g).
- Tableau de conversion affiché en aide après une erreur (variante « unités de mesure » de `placeValueTable`).
- Lecture Piper des unités en toutes lettres (« centilitres », « kilogrammes »), vérifiée par un test sur toutes les abréviations.

## Distracteurs (QCM)

Facteur 10 au lieu de 100 ou 1 000 (2 m = 20 cm), unités voisines confondues (cL et mL), comparaison des nombres sans les unités (3 km < 2 900 m), unité irréaliste (un circuit de 5 m).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Longueurs, masses et contenances » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
