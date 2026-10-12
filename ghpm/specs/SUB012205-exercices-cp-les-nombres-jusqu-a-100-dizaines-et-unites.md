---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaxg
rev: 2026-10-11T18:51:31Z
url: https://github.com/georgesp/Educator/issues/217
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012205 — Exercices CP — Les nombres jusqu'à 100 : dizaines et unités"
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#122"
order: 1920
---

## User story

En tant qu'enfant de CP, je veux m'entraîner à comprendre, lire et écrire les nombres avec les dizaines et les unités afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CP : base dix et valeur positionnelle dès la période 1 ; quantités jusqu'à 59 au plus tard en période 2, jusqu'à 100 au plus tard en période 3 ; écrire les nombres en lettres jusqu'à cinquante en fin de CP.
- Écrire un nombre dicté, lire un nombre écrit en chiffres ; expliquer pourquoi 23 et 32 sont différents (3 dizaines et 4 unités = 34).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cp-num-to-59` | 2 | Dizaines et unités jusqu'à 59 | Nombre de 10 à 59 : collection de barres et cubes → nombre ; nombre dicté → chiffres ; « 3 dizaines et 4 unités » → 34 | `writeNumber`, `recompose`, `decompose` |
| `cp-num-to-100` | 3 | Dizaines et unités jusqu'à 100 | Même chose de 10 à 100, dont les nombres « difficiles » à dire (soixante-dix à quatre-vingt-dix-neuf) | `writeNumber`, `recompose`, `decompose` |
| `cp-num-words-50` | 5 | Écrire les nombres en lettres jusqu'à cinquante | Nombre de 0 à 50 → choisir la bonne écriture en lettres (QCM, orthographe traditionnelle) ; saisie au clavier quand l'orthographe du lot 2 sera disponible (SUB001405), graphie de 1990 alors acceptée | `readNumber` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Combien de cubes ? (3 barres et 4 cubes) → 34.
- Écris le nombre que tu entends : « soixante-douze » → 72.
- 4 dizaines et 7 unités → 47.
- Dans 52, combien de dizaines ? → 5.
- Choisis la bonne écriture de 21 : vingt et un / vingt un / vingt-un → vingt et un (la graphie de 1990 « vingt-et-un », juste elle aussi, n'est jamais proposée comme mauvaise réponse).

## Évolutions du moteur et de l'écran de série

- Famille **numération** (lot 1, CE1) avec une **image** dans l'exercice : barres de dix et cubes dessinés (réutilise `tensUnits`).
- Consigne lue avant l'affichage pour la dictée de nombres ; bouton Réécouter.

## Distracteurs (QCM)

Chiffres inversés (43 pour 34), 70 + 2 mal entendu (612), dizaines et unités confondues.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CP — Les nombres jusqu'à 100 : dizaines et unités » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001612 (Numération et comparaison des nombres) : partie CP.
