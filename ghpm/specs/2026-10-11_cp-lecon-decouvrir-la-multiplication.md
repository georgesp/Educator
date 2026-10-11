---
type: Issue
title: "Leçon CP — Découvrir la multiplication : « fois »"
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-4-cp.md"
order: 2620
---
## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique ce que veut dire « fois », avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 4.
- **Compétences liées** (`skillIds`) : `cp-times-as-repeated-add` — exercices : la fiche « Exercices CP — Découvrir la multiplication : « fois » ».
- **Objectif** : comprendre « 3 fois 7 » comme 7 + 7 + 7 avec des paquets identiques.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « fois », « paquet », « autant ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Découvrir la multiplication : « fois » » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Des paquets pareils** — 3 boîtes de 7 pneus. | `groupObjects` (groupBy 7) |
| 3 | **Compter vite** — 7 + 7 + 7 = 21 : on ajoute les paquets. | `groupObjects` + `calcChain` |
| 4 | **Dire « fois »** — 3 fois 7 = 21 : « fois » veut dire « autant de paquets ». | `equation` |
| 5 | **À toi** — Interaction : fais 4 paquets de 5 casques. | `groupObjects` (interactif) |
| 6 | **Des rangées** — La grille de départ : 2 rangées de 6 voitures = 2 fois 6. | `arrayGrid` |
| 7 | **3 fois 20** — 3 barres de 20 : 20 + 20 + 20 = 60. | `tensUnits` |
| 8 | **Pas une addition des deux nombres** — 3 fois 20 ≠ 23. | `groupObjects` |
| 9 | **À toi** — Interaction : combien de biscuits dans 2 paquets de 7 ? | `groupObjects` (interactif) |
| 10 | **Je retiens** — « 3 fois 7, c'est 3 paquets de 7 : 7 + 7 + 7 = 21. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `calcChain`, `equation`, `groupObjects`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 3 fois 20 = ?
2. 4 fois 5 = ? (QCM : 9, 20, 45)
3. 2 paquets de 7 biscuits : combien ?
4. 5 + 5 + 5, c'est … fois 5

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-decouvrir-la-multiplication.json`, validée par `lesson.schema.json`, `skillIds` = `cp-times-as-repeated-add`.
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
