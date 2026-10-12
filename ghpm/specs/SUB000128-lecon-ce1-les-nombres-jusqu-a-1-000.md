---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYF0
url: https://github.com/georgesp/Educator/issues/96
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000128 — Leçon CE1 — Les nombres jusqu'à 1 000
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 1205
---

## User story

En tant qu'enfant de CE1, je veux une leçon courte et animée qui m'explique ce qu'est une centaine et comment lire les nombres jusqu'à 1 000, avant de m'entraîner.

## Leçon

- **Classe / période** : CE1, période 1.
- **Compétences liées** (`skillIds`) : `ce1-num-hundreds`, `ce1-num-read-write` — exercices : la fiche « Exercices CE1 — Les nombres jusqu'à 1 000 ».
- **Objectif** : comprendre la centaine (10 dizaines), lire, écrire et décomposer un nombre à trois chiffres.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « centaine », « dizaine », « unité », « chiffre », « nombre ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Les nombres jusqu'à 1 000 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Rappel** — Cubes et barres : 10 unités = 1 dizaine. | `tensUnits` |
| 3 | **La centaine** — 10 barres s'assemblent en une plaque : 1 centaine = 100. | `tensUnits` (plaque) |
| 4 | **Lire 347** — 3 plaques, 4 barres, 7 cubes : trois cent quarante-sept. | `tensUnits` + `placeValueTable` |
| 5 | **Le zéro** — 608 : pas de dizaine, le 0 garde la place. | `placeValueTable` |
| 6 | **À toi** — Interaction : fabrique 253 avec les plaques, barres et cubes. | `tensUnits` (interactif) |
| 7 | **Chiffre ou nombre ?** — 347 : chiffre des dizaines 4, nombre de dizaines 34. | `placeValueTable` |
| 8 | **À toi** — Interaction : écris en chiffres le nombre lu par la voix. | `placeValueTable` (interactif) |
| 9 | **Écrire en lettres** — Trait d'union entre dizaines et unités (quarante-sept), mais « vingt et un » ; « deux cents » mais « deux cent huit ». Depuis 1990, on peut aussi mettre des traits d'union partout : les deux écritures sont justes. | `equation` (deux écritures) |
| 10 | **Je retiens** — « 10 dizaines = 1 centaine. Dans 347, le 3 compte les centaines, le 4 les dizaines, le 7 les unités. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `equation`, `placeValueTable`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 3 centaines, 4 dizaines et 7 unités = ?
2. Écris en chiffres : six cent huit. (QCM : 68, 608, 6 008)
3. Combien de dizaines dans 347 ?
4. 300 + 40 + 7 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce1-nombres-jusqu-a-1000.json`, validée par `lesson.schema.json`, `skillIds` = `ce1-num-hundreds`, `ce1-num-read-write`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
