---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACa44
url: https://github.com/georgesp/Educator/issues/223
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012211 — Leçon CP — Compter de 1 en 1, de 2 en 2, de 10 en 10 et à rebours"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2560
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique comment compter de différentes façons, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 1.
- **Compétences liées** (`skillIds`) : `cp-count-1-2`, `cp-count-10-back`, `cp-ordinals` — exercices : la fiche « Exercices CP — Compter de 1 en 1, de 2 en 2, de 10 en 10 et à rebours ».
- **Objectif** : compter de 1 en 1, de 2 en 2, de 10 en 10 et à rebours depuis n'importe quel nombre.
- **Durée visée** : 3 à 5 minutes, 11 slides.
- **Vocabulaire du programme** : « compter », « à rebours », « suivant », « précédent ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Compter de 1 en 1, de 2 en 2, de 10 en 10 et à rebours » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **De 1 en 1** — La voiture avance case par case : 23, 24, 25… | `numberLine` (pas de 1) |
| 3 | **À rebours** — Le compte à rebours du départ : 10, 9, 8… partez ! | `numberLine` (arrière) |
| 4 | **De 2 en 2** — Bonds de 2 : 12, 14, 16… | `numberLine` (pas de 2) |
| 5 | **À toi** — Interaction : complète la suite de 2 en 2. | `sequencePattern` (interactif) |
| 6 | **De 10 en 10** — 7, 17, 27… : barre après barre, les unités ne bougent pas. | `tensUnits` + `sequencePattern` |
| 7 | **À toi** — Interaction : fais avancer la voiture de 10 en 10 depuis 4. | `numberLine` (interactif) |
| 8 | **Motifs** — Drapeau rouge, drapeau blanc… lequel est le 9e ? | `sequencePattern` (motifs) |
| 9 | **À toi** — Interaction : trouve le nombre caché dans la suite à rebours. | `sequencePattern` (interactif) |
| 10 | **Premier, deuxième…** — La course : la voiture rouge est 1re, la bleue 2e… Devant la 4e, il y a 3 voitures. Interaction : clique sur la 6e. | `groupObjects` (file, interactif) |
| 11 | **Je retiens** — « De 2 en 2, je saute un nombre. De 10 en 10, seul le chiffre des dizaines change. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `groupObjects`, `numberLine`, `recapCard`, `sequencePattern`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 12, 14, 16, … ?
2. 7, 17, 27, … ? (QCM : 28, 37, 47)
3. 31, 30, 29, … ?
4. Rouge, blanc, rouge, blanc : 9e drapeau ?
5. Clique sur la quatrième voiture (file de 8).

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-compter-suites.json`, validée par `lesson.schema.json`, `skillIds` = `cp-count-1-2`, `cp-count-10-back`, `cp-ordinals`.
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
