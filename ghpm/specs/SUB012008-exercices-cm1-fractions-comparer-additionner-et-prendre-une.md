---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZuU
url: https://github.com/georgesp/Educator/issues/162
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012008 — Exercices CM1 — Fractions : comparer, additionner et prendre une fraction d'une quantité"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1910
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à comparer et additionner des fractions et calculer la fraction d'une quantité afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : comparer, additionner et soustraire des fractions ; déterminer une fraction d'une quantité.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-frac-compare-add` | 2 | Comparer, additionner et soustraire des fractions | Même dénominateur ou dénominateurs multiples (≤ 20) ; résultat éventuellement > 1 | `compare`, `result` |
| `cm1-frac-of-quantity` | 3 | Fraction d'une quantité | a/b de N avec N multiple de b, N ≤ 200 (3/4 de 20, 2/5 de 35) | `result`, `phrase` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 3/4 de 20 = ? → 15.
- 2/5 + 4/5 = ? → 6/5.
- 3/10 … 1/5 → >.

## Évolutions du moteur et de l'écran de série

- Réponse fraction : toute fraction égale acceptée (simplification non attendue), forme attendue rappelée dans le retour.

## Distracteurs (QCM)

Diviser sans multiplier (3/4 de 20 → 5), ajouter les dénominateurs.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Fractions : comparer, additionner et prendre une fraction d'une quantité » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
