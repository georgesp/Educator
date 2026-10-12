---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACahg
rev: 2026-10-11T18:51:16Z
url: https://github.com/georgesp/Educator/issues/204
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012116 — Leçon CM2 — Les nombres décimaux jusqu'aux millièmes
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 1800
---

## User story

En tant qu'enfant de CM2, je veux une leçon courte et animée qui m'explique ce qu'est un millième, avant de m'entraîner.

## Leçon

- **Classe / période** : CM2, période 1.
- **Compétences liées** (`skillIds`) : `cm2-dec-read-write`, `cm2-dec-compare-round`, `cm2-dec-usual-facts` — exercices : la fiche « Exercices CM2 — Les nombres décimaux jusqu'aux millièmes ».
- **Objectif** : ajouter le rang des millièmes, écrire et décomposer, comparer et arrondir.
- **Durée visée** : 3 à 5 minutes, 11 slides.
- **Vocabulaire du programme** : « millième », « centième », « dixième », « arrondi ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Les nombres décimaux jusqu'aux millièmes » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Rappel** — Dixièmes et centièmes dans le tableau. | `placeValueTable` (décimal) |
| 3 | **Le millième** — Zoom sur la piste : un centième se coupe en 10. | `numberLine` (zoom) |
| 4 | **Chronomètre** — Les temps des voitures au millième : 58,247 s. | `placeValueTable` |
| 5 | **Décomposer** — 4,073 = 4 + 7/100 + 3/1 000. | `placeValueTable` + `equation` |
| 6 | **À toi** — Interaction : écris le temps dicté dans le tableau. | `placeValueTable` (interactif) |
| 7 | **Comparer** — 0,7 = 0,700 > 0,689. | `compareNumbers` |
| 8 | **Arrondir** — 3,456 au dixième : entre 3,4 et 3,5, plus près de 3,5. | `numberLine` (zoom) |
| 9 | **À toi** — Interaction : classe les temps du podium. | `sequencePattern` (ordre, interactif) |
| 10 | **À connaître par cœur** — 1/2 = 0,5 ; 1/4 = 0,25 ; 3/4 = 0,75 ; 1/5 = 0,2 ; la moitié de 7 est 3,5. | `fractionBar` + `placeValueTable` |
| 11 | **Je retiens** — « Un millième, c'est un centième partagé en 10 : 1/1 000 = 0,001. Je compare rang par rang après la virgule. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `compareNumbers`, `equation`, `fractionBar`, `numberLine`, `placeValueTable`, `recapCard`, `sequencePattern`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 4 + 7/100 + 3/1 000 = ?
2. 0,7 … 0,689 (QCM : <, >, =)
3. Arrondi au dixième de 3,456 ?
4. Chiffre des millièmes de 58,247 ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm2-decimaux-milliemes.json`, validée par `lesson.schema.json`, `skillIds` = `cm2-dec-read-write`, `cm2-dec-compare-round`, `cm2-dec-usual-facts`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 3, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
