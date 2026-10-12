---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZ_U
rev: 2026-10-11T18:50:37Z
url: https://github.com/georgesp/Educator/issues/175
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012021 — Leçon CM1 — Comparer, ranger et arrondir les nombres décimaux"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1530
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique comment comparer des nombres à virgule sans se faire piéger, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 3.
- **Compétences liées** (`skillIds`) : `cm1-dec-compare`, `cm1-dec-round` — exercices : la fiche « Exercices CM1 — Comparer, ranger et arrondir les nombres décimaux ».
- **Objectif** : comparer d'abord les parties entières puis les dixièmes, les centièmes ; arrondir à l'unité.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « arrondi », « encadrer », « dixième », « centième ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Comparer, ranger et arrondir les nombres décimaux » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Le piège** — 2,45 a plus de chiffres que 2,5… mais qui va le plus loin sur la piste ? | `numberLine` (décimaux) |
| 3 | **Rang par rang** — Parties entières égales, dixièmes : 5 > 4, donc 2,5 > 2,45. | `compareNumbers` + `placeValueTable` |
| 4 | **Ajouter un zéro** — 2,5 = 2,50 : on compare 50 et 45 centièmes. | `fractionBar` (stack) |
| 5 | **À toi** — Interaction : 3,1 … 3,09. | `compareNumbers` (interactif) |
| 6 | **Ranger** — Les temps de course des voitures : 12,4 s ; 12,04 s ; 12,14 s. | `sequencePattern` (ordre, interactif) |
| 7 | **Encadrer** — 12 < 12,3 < 13 : deux drapeaux. | `numberLine` |
| 8 | **Arrondir** — 7,62 est plus près de 8 que de 7 : arrondi 8. | `numberLine` (zoom) |
| 9 | **À toi** — Interaction : arrondis le temps de chaque voiture à la seconde. | `numberLine` (interactif) |
| 10 | **Je retiens** — « Je compare les parties entières, puis les dixièmes, puis les centièmes. 2,5 = 2,50 > 2,45. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `compareNumbers`, `fractionBar`, `numberLine`, `placeValueTable`, `recapCard`, `sequencePattern`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 2,5 … 2,45 (QCM : <, >, =)
2. Arrondi à l'unité de 7,62 ?
3. Encadre 12,3 entre deux entiers.
4. Range : 3,1 ; 3,09 ; 3,15.

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-decimaux-comparer-arrondir.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-dec-compare`, `cm1-dec-round`.
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
