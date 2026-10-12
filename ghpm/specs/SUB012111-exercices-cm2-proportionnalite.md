---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACabA
url: https://github.com/georgesp/Educator/issues/199
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012111 — Exercices CM2 — Proportionnalité
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 2310
---

## User story

En tant qu'enfant de CM2, je veux m'entraîner à résoudre des problèmes de proportionnalité en deux étapes afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM2 : identifier une situation de proportionnalité et résoudre des problèmes de proportionnalité.
- Livret d'accompagnement CM2 : problèmes à deux étapes en linéarité multiplicative, puis additive, puis procédures mixtes (200 feuilles font 24 mm : épaisseur de 250 feuilles ?) ; le passage par l'unité n'est pas attendu avant la 6e.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm2-proportion` | 3 | Problèmes de proportionnalité | Gabarits en deux étapes : linéarité multiplicative (÷ 4 puis × 5), additive (200 + 50), mixte ; tableaux à compléter | `problem`, `proportionTable` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 200 feuilles font 24 mm. Quelle épaisseur pour 250 feuilles ? → 30 mm.
- 4 pneus coûtent 180 €. Combien coûtent 10 pneus ? → 450 €.

## Évolutions du moteur et de l'écran de série

- Gabarits de proportionnalité à deux étapes ; le générateur choisit des nombres qui évitent le passage par l'unité.

## Distracteurs (QCM)

Raisonnement additif (250 feuilles → 74 mm), coefficient appliqué à la mauvaise colonne.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM2 — Proportionnalité » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)

## Remarques et décisions

- Les pourcentages ne sont pas des attendus explicites du CM2 dans le programme : la fiche ne les traite pas (20 % = 1/5 seulement en lien avec les fractions, si Georges le souhaite).
