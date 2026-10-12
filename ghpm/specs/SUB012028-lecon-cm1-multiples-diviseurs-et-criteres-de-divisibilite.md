---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaJo
url: https://github.com/georgesp/Educator/issues/182
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012028 — Leçon CM1 — Multiples, diviseurs et critères de divisibilité"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1760
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique ce que sont les multiples et les diviseurs, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 2.
- **Compétences liées** (`skillIds`) : `cm1-divisibility`, `cm1-multiples-divisors` — exercices : la fiche « Exercices CM1 — Multiples, diviseurs et critères de divisibilité ».
- **Objectif** : relier multiple, diviseur et tables ; utiliser les critères de divisibilité par 2, 5 et 10.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « multiple », « diviseur », « divisible », « pair », « impair ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Multiples, diviseurs et critères de divisibilité » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Multiples** — Les multiples de 6 s'allument dans la table : 6, 12, 18… | `timesTable` |
| 3 | **Diviseurs** — 42 voitures en rangées égales : 6 × 7, 7 × 6, 2 × 21… 6 et 7 sont des diviseurs. | `arrayGrid` |
| 4 | **Multiple ↔ diviseur** — 42 = 6 × 7 : deux façons de dire la même chose. | `equation` (famille) |
| 5 | **Par 2** — Les nombres pairs finissent par 0, 2, 4, 6, 8 : bonds de 2 sur la piste. | `numberLine` (pas de 2) |
| 6 | **Par 5 et par 10** — Bonds de 5 : on tombe sur des nombres finissant par 0 ou 5 ; bonds de 10 : par 0. | `numberLine` + `sequencePattern` |
| 7 | **À toi** — Interaction : trie les nombres dans les garages « divisible par 5 » / « non ». | `groupObjects` (tri, interactif) |
| 8 | **Autres diviseurs** — Pour 3, 4, 6… pas de critère au CM1 : on utilise les tables ou une division. | `timesTable` |
| 9 | **À toi** — Interaction : trouve tous les diviseurs de 12 avec les grilles. | `arrayGrid` (interactif) |
| 10 | **Je retiens** — « 42 = 6 × 7 : 42 est un multiple de 6 et de 7 ; 6 et 7 sont des diviseurs de 42. Divisible par 2 : finit par 0, 2, 4, 6, 8 ; par 5 : 0 ou 5 ; par 10 : 0. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `equation`, `groupObjects`, `numberLine`, `recapCard`, `sequencePattern`, `timesTable`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 135 est-il divisible par 5 ?
2. Lequel est un multiple de 7 ? (QCM : 48, 49, 50)
3. 4 est-il un diviseur de 30 ?
4. Cite un nombre divisible par 2 et par 5.

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-multiples-diviseurs.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-divisibility`, `cm1-multiples-divisors`.
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
