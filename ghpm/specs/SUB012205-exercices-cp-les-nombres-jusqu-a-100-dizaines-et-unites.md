---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaxg
rev: 2026-10-17T13:46:40Z
url: https://github.com/georgesp/Educator/issues/217
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012205 — Exercices CP — Les nombres jusqu'à 100 : dizaines et unités"
status: In progress
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
| `cp-num-to-59` | 2 | Dizaines et unités jusqu'à 59 | Nombre de 10 à 59 : collection de barres et cubes → nombre ; nombre dicté → chiffres ; « 3 dizaines et 4 unités » → 34 ; « 30 + 4 » → 34 ; dans 52, combien de dizaines ? | `numberDictation`, `recompose`, `decompose`, `placeValue` |
| `cp-num-to-100` | 3 | Dizaines et unités jusqu'à 100 | Même chose de 20 à 99, où les nombres « difficiles » à dire (soixante-dix à quatre-vingt-dix-neuf) sont fréquents ; 100 est travaillé dans la comparaison et la leçon (pas de « centaine » au CP) | `numberDictation`, `recompose`, `decompose`, `placeValue` |
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

- Famille **numération** (lot 1, CE1) avec une **image** dans l'exercice : barres de dix et cubes dessinés (réutilise `tensUnits`) ; le champ `picture` des compétences (aujourd'hui `arrayGrid` seulement) s'ouvre à `tensUnits`.
- Dictée de nombres : le nombre n'est pas affiché ; il est lu au clic sur « Écouter le nombre », jamais tout seul (décision du 2026-10-15), autant de fois que l'enfant veut.

## Distracteurs (QCM)

Chiffres inversés (43 pour 34), 70 + 2 mal entendu (612), dizaines et unités confondues.

## Critères d'acceptation

- [x] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [x] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [x] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md` (nouveau fichier ; les 11 compétences du lot 1 restent dans `echantillons-lot1.md`), relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement (décision du 2026-10-15).
- [ ] Exercices jouables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [x] `cp-num-to-59` est liée à la fiche « Leçon CP — Les nombres jusqu'à 59 : dizaines et unités », `cp-num-to-100` et `cp-num-words-50` à la fiche « Leçon CP — Les nombres de 60 à 100 et en lettres » (`skillIds` des leçons).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001612 (Numération et comparaison des nombres) : partie CP.

## Remarques et décisions

- Implémentation (2026-10-17) : nouveau format `numberDictation` (le nombre n'est jamais écrit, il est lu au clic ; sans voix, une série ne le propose pas) ; `placeValue` accepte les nombres à deux chiffres ; propositions toujours inférieures à 100 (sauf 612 pour « soixante-douze » dans la dictée, erreur d'écoute voulue).
- Corrections communes au CE1 : comparaison écrite au singulier pour 1 (« 1 dizaine »), égalité plus rare dans `compare`, 612 / 815 proposés pour 72 / 95 dans `writeNumber`.
