---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZe8
url: https://github.com/georgesp/Educator/issues/150
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011928 — Leçon CE2 — Nombres jusqu'à 10 000
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1320
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique comment sont construits les nombres jusqu'à 10 000, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 1.
- **Compétences liées** (`skillIds`) : `ce2-num-read-write`, `ce2-num-decompose` — exercices : la fiche « Exercices CE2 — Nombres jusqu'à 10 000 ».
- **Objectif** : comprendre le millier (10 centaines), lire, écrire et décomposer un nombre à 4 chiffres.
- **Durée visée** : 3 à 5 minutes, 12 slides.
- **Vocabulaire du programme** : « millier », « centaine », « dizaine », « unité », « chiffre », « nombre ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Nombres jusqu'à 10 000 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Rappel** — Cube, barre, plaque : unité, dizaine, centaine. | `tensUnits` |
| 3 | **Le millier** — 10 plaques s'empilent en un gros bloc : 1 millier = 10 centaines = 1 000. | `tensUnits` (bloc) |
| 4 | **Lire 4 635** — 4 blocs, 6 plaques, 3 barres, 5 cubes → quatre mille six cent trente-cinq. | `tensUnits` + `placeValueTable` |
| 5 | **Le tableau** — Chaque chiffre dans sa colonne : m, c, d, u ; l'espace sépare les milliers. | `placeValueTable` |
| 6 | **Le zéro** — 4 605 : pas de dizaine, le 0 garde la place. | `placeValueTable` |
| 7 | **À toi** — Interaction : glisse les blocs pour fabriquer 2 308. | `tensUnits` (interactif) |
| 8 | **Chiffre ou nombre ?** — Dans 3 200 : chiffre des centaines 2, nombre de centaines 32. | `placeValueTable` |
| 9 | **À toi** — Interaction : écris en chiffres le nombre lu par la voix. | `placeValueTable` (interactif) |
| 10 | **Écrire en lettres** — 4 635 : quatre mille six cent trente-cinq. Trait d'union entre dizaines et unités (trente-cinq), mais « vingt et un » ; « mille » ne prend jamais de s ; « deux cents » mais « deux cent trois ». | `placeValueTable` + `equation` |
| 11 | **Une autre écriture permise** — Depuis 1990, on a aussi le droit de mettre des traits d'union partout : quatre-mille-six-cent-trente-cinq. Les deux écritures sont justes. | `equation` (deux écritures côte à côte) |
| 12 | **Je retiens** — « 10 unités = 1 dizaine, 10 dizaines = 1 centaine, 10 centaines = 1 millier. Chaque chiffre a une valeur selon sa place. » | `recapCard` |

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

1. Écris en chiffres : sept mille quarante.
2. 4 000 + 600 + 5 = ? (QCM : 465, 4 605, 4 650)
3. Quel est le chiffre des centaines de 7 382 ?
4. Combien de centaines dans 3 200 ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-nombres-jusqu-a-10000.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-num-read-write`, `ce2-num-decompose`.
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
