---
type: Issue
title: "Leçon CM1 — Les fractions : écrire, placer et dépasser 1"
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-2-cm1.md"
order: 1900
---
## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique comment une fraction peut dépasser 1, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 1.
- **Compétences liées** (`skillIds`) : `cm1-frac-read-place`, `cm1-frac-integer-part` — exercices : la fiche « Exercices CM1 — Les fractions : écrire, placer et dépasser 1 ».
- **Objectif** : placer des fractions sur une piste graduée, reconnaître une fraction > 1 et l'écrire comme entier + fraction.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « fraction », « numérateur », « dénominateur », « unité », « demi-droite graduée ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Les fractions : écrire, placer et dépasser 1 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Rappel** — La bande-unité en quarts : 1/4, 2/4, 3/4, 4/4 = 1. | `fractionBar` (4) |
| 3 | **Encore un quart** — 5/4 : une bande pleine et un quart d'une deuxième. | `fractionBar` (2 unités) |
| 4 | **Sur la piste** — La voiture avance de quart en quart : 1/4, 2/4… 7/4. | `numberLine` (fractions) |
| 5 | **Décomposer** — 7/4 = 4/4 + 3/4 = 1 + 3/4. | `fractionBar` + `equation` |
| 6 | **À toi** — Interaction : place 5/3 sur la piste graduée en tiers. | `numberLine` (interactif) |
| 7 | **Encadrer** — 11/3 : 9/3 = 3 et 12/3 = 4, donc 3 < 11/3 < 4. | `numberLine` (drapeaux) |
| 8 | **D'autres dénominateurs** — Jusqu'à 20 : 13/10, 9/8, 25/20. | `fractionBar` |
| 9 | **À toi** — Interaction : complète 9/5 = 1 + …/5. | `equation` (interactif) |
| 10 | **Je retiens** — « Quand le numérateur dépasse le dénominateur, la fraction est plus grande que 1 : 7/4 = 4/4 + 3/4 = 1 + 3/4. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `equation`, `fractionBar`, `numberLine`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 7/4 = 1 + …/4
2. Entre quels entiers est 11/3 ? (QCM : 2 et 3, 3 et 4, 11 et 12)
3. 5/4 est-il plus grand que 1 ?
4. Écris en chiffres : treize dixièmes.

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-fractions-superieures-a-1.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-frac-read-place`, `cm1-frac-integer-part`.
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
