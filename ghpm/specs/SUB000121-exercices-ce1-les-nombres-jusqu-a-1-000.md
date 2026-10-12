---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACX6c
rev: 2026-10-12T19:48:50Z
url: https://github.com/georgesp/Educator/issues/89
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000121 — Exercices CE1 — Les nombres jusqu'à 1 000
status: In progress
size: L
estimate:
startDate:
endDate:
parent: "#1"
order: 700
---

## User story

En tant qu'enfant de CE1, je veux m'entraîner à lire, écrire et décomposer les nombres jusqu'à 1 000 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE1 : la centaine dès le début de la période 1 ; « au plus tard en période 2 », quantités et nombres jusqu'à mille.
- Écrire en chiffres un nombre dicté, lire un nombre écrit en chiffres et l'écrire en lettres ; valeur des chiffres selon leur position.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce1-num-hundreds` | 1 | Centaines, dizaines et unités | Nombre de 100 à 999 : plaques, barres, cubes → nombre ; « 3 centaines, 4 dizaines, 7 unités » → 347 ; 347 = 300 + 40 + 7 ; nombre de dizaines dans 347 (34) | `recompose`, `decompose`, `placeValue` |
| `ce1-num-read-write` | 2 | Lire et écrire les nombres jusqu'à 1 000 | Nombre ≤ 1 000 dicté par Piper → chiffres ; nombre en chiffres → choisir la bonne écriture en lettres (orthographe traditionnelle ; la graphie de 1990 n'est jamais une mauvaise réponse) | `writeNumber`, `readNumber` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 3 centaines, 4 dizaines et 7 unités → 347.
- Écris le nombre que tu entends : « six cent huit » → 608.
- Combien de dizaines dans 347 ? → 34.
- Choisis la bonne écriture de 280 : deux cent quatre-vingts / deux cents quatre-vingt / deux cent quatre-vingt → la première.

## Évolutions du moteur et de l'écran de série

- Nouvelle famille d'exercices **numération** (`operation: "numeration"`) et formats `writeNumber`, `readNumber`, `decompose`, `recompose`, `placeValue` — repris de SUB001612 ; étendus ensuite au CP (lot 1.4), au CE2 (lot 4) et au cycle 3.
- Image dans l'exercice : plaques, barres et cubes dessinés (réutilise `tensUnits`).
- Conversion nombre → français écrit, orthographe traditionnelle, jusqu'à 1 000 (cas pièges : 71, 80, 81, 200, 201, 280) ; une fonction de comparaison accepte aussi la graphie rectifiée de 1990 pour les saisies au clavier.

## Distracteurs (QCM)

Zéro oublié ou ajouté (68, 6 008), chiffres inversés, confusion chiffre des dizaines (4) / nombre de dizaines (34), « cents » mal accordé.

## Critères d'acceptation

- [x] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [x] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [x] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE1 — Les nombres jusqu'à 1 000 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001612 (Numération et comparaison des nombres) : partie CE1 et création des formats de numération.

## Remarques et décisions

- Orthographe des nombres en lettres (décision de Georges du 2026-10-11) : orthographe traditionnelle affichée ; la rectification de 1990 est présentée dans la leçon et acceptée en saisie.

## Réalisation (2026-10-11)

- La voix est coupée pour l'instant dans les exercices de maths : « écris le nombre que tu entends » affiche le nombre en lettres (la consigne lue reste prête pour quand la voix reviendra).
- Les nombres pièges (71, 80, 81, 200, 280…) sont tirés plus souvent en lecture ; la graphie de 1990 n'est jamais proposée comme mauvaise réponse.
- Restent ouverts : relecture des échantillons (`docs/specs/echantillons-ce1.md`), lecture par Piper (voix coupée dans les maths pour l'instant), lien avec la leçon (format des leçons au lot 2).
