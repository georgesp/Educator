---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdqBU
rev: 2026-10-17T13:47:05Z
url: https://github.com/georgesp/Educator/issues/331
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012221 — Leçon CP — Les nombres de 60 à 100 et en lettres
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 1935
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique les nombres de soixante à cent et comment écrire les nombres en lettres, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 3 (nombres jusqu'à 100 au plus tard en période 3 ; écriture en lettres jusqu'à cinquante en fin de CP).
- **Compétences liées** (`skillIds`) : `cp-num-to-100`, `cp-num-words-50` — exercices : la fiche « Exercices CP — Les nombres jusqu'à 100 : dizaines et unités ». Les nombres jusqu'à 59 sont dans la fiche « Leçon CP — Les nombres jusqu'à 59 : dizaines et unités ».
- **Objectif** : lire et écrire les nombres de 60 à 100 malgré leur nom (soixante-dix, quatre-vingts), comme des dizaines et des unités ; écrire en lettres les nombres jusqu'à cinquante.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « dizaine », « unité », « chiffre », « nombre », « cent ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Les nombres de 60 à 100 et en lettres » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Après 59** — 5 barres et 9 cubes ; un cube de plus : 10 cubes font une barre, 6 barres = 60, soixante. | `tensUnits` (échange) |
| 3 | **Soixante-dix** — On dit « soixante-dix » (60 + 10) mais on voit 7 barres : on écrit 70. | `tensUnits` |
| 4 | **Soixante-douze** — 60 + 12 à l'oral, 7 dizaines et 2 unités à l'écrit : 72. | `tensUnits` + `placeValueTable` |
| 5 | **Quatre-vingts** — 8 barres : quatre-vingts, 80 ; quatre-vingt-quinze : 9 barres et 5 cubes, 95. | `tensUnits` |
| 6 | **À toi** — Interaction : fabrique avec les barres et les cubes le nombre entendu au clic (soixante-dix-huit). | `tensUnits` (interactif) |
| 7 | **Cent** — 10 barres de 10 : cent, 100, le bout de la piste. | `tensUnits` + `numberLine` (0 à 100) |
| 8 | **Écrire en lettres** — trente-deux, quarante-cinq : un trait d'union entre les mots ; vingt et un, trente et un : « et un ». | `placeValueTable` (mots) |
| 9 | **À toi** — Interaction : écris « quarante-trois » avec les étiquettes de lettres. | `letterTiles` (interactif) |
| 10 | **Je retiens** — « J'écris ce que je compte, pas ce que j'entends : soixante-douze, c'est 7 dizaines et 2 unités, 72. En lettres : un trait d'union, sauf avec « et un ». » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `letterTiles`, `numberLine`, `placeValueTable`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » (`letterTiles` : bibliothèque des leçons d'orthographe, déjà au schéma des leçons).
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Écris le nombre entendu au clic : soixante-douze.
2. Combien de dizaines dans 85 ?
3. Choisis la bonne écriture de 21 (QCM : vingt et un, vingt un, vingt-un ; la graphie de 1990 « vingt-et-un », juste elle aussi, n'est jamais proposée comme mauvaise réponse).
4. 7 dizaines et 4 unités = ? (QCM : 74, 47, 64)

Chaque question a une explication courte, affichée après la réponse et lisible au clic (rien n'est lu tout seul, décision du 2026-10-15).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [x] Leçon `content/lessons/math/cp-nombres-60-100.json`, validée par `lesson.schema.json`, `skillIds` = `cp-num-to-100`, `cp-num-words-50`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [x] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [x] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [x] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
