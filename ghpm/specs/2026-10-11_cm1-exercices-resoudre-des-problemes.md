---
type: Issue
title: Exercices CM1 — Résoudre des problèmes
status: Todo
size: L
parent: "specs/2026-10-11_lot-1-2-cm1.md"
order: 2030
---
## User story

En tant qu'enfant de CM1, je veux m'entraîner à résoudre des problèmes en plusieurs étapes afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : quatre phases (comprendre, modéliser, calculer, répondre) ; au moins 10 problèmes par semaine.
- Problèmes additifs en une, deux ou trois étapes ; multiplicatifs parties-tout ; comparaison multiplicative ; mixtes ; dénombrement ; optimisation ; nombres jusqu'à 999 999, décimaux et fractions.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-problems-multi-step` | 1 | Problèmes en plusieurs étapes | Gabarits additifs et mixtes en 2 ou 3 étapes, nombres ≤ 999 999 | `problem` |
| `cm1-problems-multiplicative` | 2 | Problèmes multiplicatifs et de comparaison | Groupements, partages avec reste, « fois plus / fois moins » | `problem` |
| `cm1-problems-counting` | 4 | Dénombrement et optimisation | Combinaisons (tenues, menus), meilleur choix (lot le moins cher) | `problem` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Un circuit fait 4 250 m. Une voiture fait 3 tours puis 1 200 m. Quelle distance ?
- Un pneu coûte 4 fois moins qu'une jante à 96 €. Prix du pneu ?
- Lot de 3 casques à 42 € ou casque à 15 € : le moins cher pour 6 casques ?

## Évolutions du moteur et de l'écran de série

- Gabarits de problèmes (lot 1, CE1) étendus : décimaux (prix), restes, plusieurs questions dans un énoncé.

## Distracteurs (QCM)

Étape intermédiaire donnée comme réponse, « fois moins » traité comme soustraction.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Résoudre des problèmes » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
