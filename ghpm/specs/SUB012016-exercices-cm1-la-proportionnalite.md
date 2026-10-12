---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZ5c
url: https://github.com/georgesp/Educator/issues/170
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012016 — Exercices CM1 — La proportionnalité
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 2010
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à résoudre des problèmes de proportionnalité afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : reconnaître une situation de proportionnalité et résoudre des problèmes de proportionnalité (linéarité).
- Livret d'accompagnement CM1 : réactivation par des problèmes du type « 5 T-shirts identiques à 16 € pièce ».

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-proportion` | 5 | Problèmes de proportionnalité | Gabarits : prix, recettes, distances ; linéarité multiplicative (× 2, × 3, × 10) et additive (prix de 5 = prix de 2 + prix de 3) | `problem`, `proportionTable` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 3 casques coûtent 45 €. Combien coûtent 6 casques ? → 90 €.
- Complète le tableau : 2 → 8 ; 5 → 20 ; 7 → ?

## Évolutions du moteur et de l'écran de série

- Format `proportionTable` : tableau à compléter dans l'exercice (réutilise l'animation `proportionTable`) ; gabarits de problèmes.

## Distracteurs (QCM)

Raisonnement additif (3 → 45, donc 6 → 48), mauvaise colonne utilisée.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — La proportionnalité » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
