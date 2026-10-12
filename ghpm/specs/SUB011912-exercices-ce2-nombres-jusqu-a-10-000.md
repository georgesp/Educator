---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZJ4
rev: 2026-10-11T18:49:10Z
url: https://github.com/georgesp/Educator/issues/134
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011912 — Exercices CE2 — Nombres jusqu'à 10 000
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 930
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à lire, écrire et décomposer les nombres jusqu'à 10 000 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2 : nombres supérieurs à mille dès la période 1, jusqu'à 10 000 au plus tard en période 2.
- Collections organisées en groupes de dix, cent et mille ; relations entre unités de numération (ex. 32 centaines = 3 200).
- Écrire en chiffres et en lettres, décomposer (4 635 = 4 000 + 600 + 30 + 5), valeur d'un chiffre selon sa position.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-num-read-write` | 1 | Lire et écrire les nombres jusqu'à 10 000 | Nombre de 1 000 à 9 999 lu par Piper → écrire en chiffres ; nombre en chiffres → choisir l'écriture en lettres | `writeNumber`, `readNumber` |
| `ce2-num-decompose` | 2 | Décomposer les nombres jusqu'à 10 000 | Nombre de 1 000 à 10 000 : décomposition additive (4 000 + 600 + 30 + 5), recomposition, chiffre / nombre de centaines | `decompose`, `recompose`, `placeValue` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Écris en chiffres le nombre que tu entends : « quatre mille six cent trente-cinq » → 4 635.
- 4 000 + 600 + 5 = ? → 4 605 (attention au zéro).
- Dans 7 382, quel est le chiffre des centaines ? → 3.
- Combien de centaines dans 3 200 ? → 32.

## Évolutions du moteur et de l'écran de série

- Formats de numération (`writeNumber`, `readNumber`, `decompose`, `recompose`, `placeValue`) créés au lot 1 (fiche « Exercices CE1 — Les nombres jusqu'à 1 000 ») et étendus jusqu'à 10 000.
- Conversion nombre → français écrit (lot 1, CE1) étendue jusqu'à 10 000, testée sur toute la plage (cas pièges : 71, 80, 81, 200, 201, 280, 1 000) ; réutilisée par la voix. Une fonction de comparaison accepte aussi la graphie rectifiée de 1990.
- Saisie au pavé numérique avec espace des milliers affiché automatiquement (4 635).

## Distracteurs (QCM)

Chiffres inversés (4 653), zéro oublié ou ajouté (465, 46 035), confusion chiffre / nombre de centaines (3 au lieu de 32).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Nombres jusqu'à 10 000 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001612 (Numération et comparaison des nombres) : partie CE2.

## Remarques et décisions

- Orthographe des nombres en lettres (décision de Georges du 2026-10-11) : on affiche l'orthographe traditionnelle (traits d'union entre les dizaines et les unités sous cent, sauf avec « et » ; « quatre-vingts », « deux cents » mais « deux cent trois » ; « mille » invariable). La leçon présente aussi la rectification de 1990 (traits d'union partout : « deux-cent-trente »), qui est correcte : quand l'enfant écrira des nombres au clavier, les deux graphies seront acceptées.
