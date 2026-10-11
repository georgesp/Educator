---
type: Issue
title: Exercices CM2 — La division posée d'un entier ou d'un décimal
status: Todo
size: L
parent: "specs/2026-10-11_lot-1-3-cm2.md"
order: 2190
---
## User story

En tant qu'enfant de CM2, je veux m'entraîner à poser une division d'un entier ou d'un décimal par un nombre à un chiffre afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM2 : diviser un entier ou un décimal par un diviseur à un chiffre.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm2-div-integer` | 1 | Division euclidienne posée | Dividende de 4 ou 5 chiffres, diviseur de 2 à 9, quotient et reste | `quotientRemainder`, affichage posé |
| `cm2-div-decimal` | 3 | Division décimale | Entier ou décimal ÷ nombre de 2 à 9, quotient décimal exact (au plus 2 décimales) : 45 ÷ 4 = 11,25 ; 7,2 ÷ 3 = 2,4 | `result`, affichage posé |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 5 728 ÷ 7 : q = 818, r = 2.
- 45 ÷ 4 = ? → 11,25.
- 7,2 ÷ 3 = ? → 2,4.

## Évolutions du moteur et de l'écran de série

- Division décimale sur le type décimal exact : le générateur ne choisit que des divisions qui tombent juste (au plus deux décimales) ; potence avec virgule au quotient au moment où l'on abaisse les dixièmes.

## Distracteurs (QCM)

Virgule oubliée au quotient (1 125), zéro intermédiaire oublié, reste donné à la place des décimales.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM2 — La division posée d'un entier ou d'un décimal » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)

## Fiches existantes reprises

SUB001604 (Division) : division décimale simple (CM2).
