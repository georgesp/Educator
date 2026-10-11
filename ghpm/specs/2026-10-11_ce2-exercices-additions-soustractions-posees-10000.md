---
type: Issue
title: Exercices CE2 — Additions et soustractions posées jusqu'à 10 000
status: Todo
size: L
parent: "specs/2026-10-11_lot-1-1-ce2.md"
order: 1390
---
## User story

En tant qu'enfant de CE2, je veux m'entraîner à poser des additions et des soustractions avec de grands nombres afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2 : poser et effectuer des additions et des soustractions en colonnes jusqu'à 10 000.
- Vocabulaire : termes, somme, différence.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-add-4-digits` | 1 | Additions posées jusqu'à 10 000 | Deux ou trois termes, résultat ≤ 10 000, au moins une retenue | `result`, affichage posé |
| `ce2-sub-4-digits` | 2 | Soustractions posées jusqu'à 10 000 | Deux nombres ≤ 10 000, résultat positif, au moins une retenue (zéros intermédiaires compris : 5 003 − 1 278) | `result`, affichage posé |
| `ce2-money-add-sub` | 5 | Additions et soustractions avec les euros et les centimes | Prix en euros et centimes (12,35 € + 4,80 €, 20 € − 7,45 €), posés, virgules alignées ; calcul interne en centimes | `result`, affichage posé |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 2 456 + 3 789 = ?
- 1 235 + 487 + 2 060 = ?
- 5 003 − 1 278 = ?
- 12,35 € + 4,80 € = ? → 17,15 €.

## Évolutions du moteur et de l'écran de série

- Composant d'**opération posée** dans l'écran de série : l'enfant écrit le résultat chiffre par chiffre, de droite à gauche, avec les retenues (repris de SUB001603) ; correction ciblée sur la colonne fausse.
- Prix : montants stockés en centimes (entiers) et affichés avec la virgule et « € » ; saisie avec la virgule. Le type décimal du lot 1.2 pourra les reprendre sans changer les exercices.
- Champ `display: "column"` dans la compétence pour demander l'affichage posé.
- Soustraction posée selon **les deux techniques** : cassage (chiffres barrés, dizaine cassée) ou compensation (10 unités ajoutées en haut, 1 dizaine en bas). La technique suit le réglage du profil (choisie à la première soustraction posée, modifiable dans les Options) ; le résultat est juste quelle que soit la technique, seules les marques d'aide changent.

## Distracteurs (QCM)

Oubli de retenue, retenue mal placée, soustraction du plus petit chiffre au plus grand dans chaque colonne (erreur classique), décalage d'une colonne.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Additions et soustractions posées jusqu'à 10 000 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001601 (Addition et soustraction du CE2 au CM2) : partie CE2 ; SUB001603 : composant d'opération posée.
