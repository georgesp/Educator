---
type: Issue
title: Exercices CM1 — Les grands nombres jusqu'à 999 999
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-2-cm1.md"
order: 1710
---
## User story

En tant qu'enfant de CM1, je veux m'entraîner à lire, écrire et décomposer les nombres jusqu'à 999 999 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : deux nouveaux rangs (dizaines et centaines de milliers) ; attendus de fin d'année sur des nombres d'au plus six chiffres.
- Pendant les deux premières périodes, nombres d'au plus quatre chiffres ; nombres de cinq ou six chiffres à partir de la période 3 ou du début de la période 4.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-num-read-write` | 3 | Lire et écrire les nombres jusqu'à 999 999 | Nombre de 10 000 à 999 999 dicté par Piper → chiffres ; chiffres → écriture en lettres (QCM, orthographe traditionnelle ; distracteurs avec erreurs d'accord de « cent » et « mille », jamais la graphie de 1990 comme mauvaise réponse) | `writeNumber`, `readNumber` |
| `cm1-num-decompose` | 3 | Décomposer les grands nombres | Décomposition additive, classes (milliers / unités), chiffre et nombre de milliers, de centaines | `decompose`, `recompose`, `placeValue` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Écris en chiffres : « deux cent quatre mille trente » → 204 030.
- Combien de milliers dans 538 214 ? → 538.
- 300 000 + 5 000 + 70 + 2 = ? → 305 072.

## Évolutions du moteur et de l'écran de série

- Formats de numération du CE2 étendus jusqu'au million ; conversion nombre → français écrit jusqu'au million (SUB001612), orthographe traditionnelle, graphie rectifiée de 1990 acceptée en saisie.

## Distracteurs (QCM)

Zéros oubliés dans une classe (20 430), confusion chiffre / nombre de milliers, classes mal séparées.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Les grands nombres jusqu'à 999 999 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
