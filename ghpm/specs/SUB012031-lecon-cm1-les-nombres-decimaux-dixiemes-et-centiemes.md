---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaMo
rev: 2026-10-11T18:50:34Z
url: https://github.com/georgesp/Educator/issues/185
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012031 — Leçon CM1 — Les nombres décimaux : dixièmes et centièmes"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1510
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique ce qu'est un nombre à virgule, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 2.
- **Compétences liées** (`skillIds`) : `cm1-dec-money`, `cm1-dec-fractions`, `cm1-dec-usual-fractions` — exercices : la fiche « Exercices CM1 — Les nombres décimaux : dixièmes et centièmes ».
- **Objectif** : comprendre dixièmes et centièmes comme fractions décimales, écrire un décimal avec une virgule et le décomposer.
- **Durée visée** : 3 à 5 minutes, 11 slides.
- **Vocabulaire du programme** : « dixième », « centième », « virgule », « partie entière », « partie décimale », « fraction décimale ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Les nombres décimaux : dixièmes et centièmes » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Les prix** — Un casque à 3,45 € : 3 euros et 45 centimes. | `groupObjects` (pièces) |
| 3 | **Le dixième** — L'unité partagée en 10 : un dixième, 1/10 = 0,1. | `fractionBar` (10) |
| 4 | **Le centième** — Un dixième partagé en 10 : un centième, 1/100 = 0,01. | `fractionBar` (zoom) |
| 5 | **Le tableau** — Les colonnes dixièmes et centièmes après la virgule. | `placeValueTable` (décimal) |
| 6 | **Décomposer** — 3,45 = 3 + 4/10 + 5/100 = 345/100. | `placeValueTable` + `equation` |
| 7 | **À toi** — Interaction : place les chiffres de 207/100 dans le tableau → 2,07. | `placeValueTable` (interactif) |
| 8 | **Sur la piste** — Entre 3 et 4, dix petits bonds de 0,1. | `numberLine` (décimaux) |
| 9 | **À toi** — Interaction : écris le prix du pneu en euros avec une virgule. | `groupObjects` + `equation` (interactif) |
| 10 | **À connaître par cœur** — 1/2 = 0,5 ; 1/4 = 0,25 ; 3/4 = 0,75 : la moitié, le quart et les trois quarts d'un euro. | `fractionBar` + `placeValueTable` |
| 11 | **Je retiens** — « La virgule sépare la partie entière de la partie décimale : 3,45 = 3 unités, 4 dixièmes et 5 centièmes. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `equation`, `fractionBar`, `groupObjects`, `numberLine`, `placeValueTable`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 3 + 4/10 + 5/100 = ?
2. 207/100 = ? (QCM : 2,7, 2,07, 20,7)
3. Chiffre des dixièmes de 12,38 ?
4. Partie entière de 45,6 ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-nombres-decimaux.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-dec-money`, `cm1-dec-fractions`, `cm1-dec-usual-fractions`.
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
