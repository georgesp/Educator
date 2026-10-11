---
type: Issue
title: Exercices CM1 — Calculer avec les nombres décimaux
status: Todo
size: L
parent: "specs/2026-10-11_lot-1-2-cm1.md"
order: 1970
---
## User story

En tant qu'enfant de CM1, je veux m'entraîner à additionner, soustraire et multiplier des nombres décimaux afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : poser additions et soustractions de décimaux ; multiplier un décimal par un entier inférieur à 10 ; ajouter ou soustraire un entier inférieur à 10 à un décimal sans retenue ; multiplier et diviser un décimal par 10.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-dec-add-sub` | 4 | Additions et soustractions de décimaux | Décimaux jusqu'aux centièmes, posées (alignement des virgules) ; calcul mental : décimal ± entier < 10 sans retenue | `result`, affichage posé |
| `cm1-dec-mul-div-10` | 4 | Multiplier et diviser un décimal par 10 | d × 10 et d ÷ 10, d jusqu'aux centièmes | `result`, `missingOperand` |
| `cm1-dec-mul-int` | 5 | Multiplier un décimal par un entier | d × n avec n de 2 à 9, d jusqu'aux centièmes, posé | `result`, affichage posé |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 12,5 + 3,75 = ? → 16,25.
- 8,4 − 2,65 = ? → 5,75.
- 3,6 × 10 = ? → 36.
- 4,25 × 3 = ? → 12,75.

## Évolutions du moteur et de l'écran de série

- Opérations sur le type décimal exact ; composant d'opération posée avec virgules alignées (et zéros ajoutés à droite affichés en gris).

## Distracteurs (QCM)

Virgules non alignées (12,5 + 3,75 = 4,00), × 10 en « ajoutant un zéro » (3,60), virgule mal placée au produit.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Calculer avec les nombres décimaux » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
