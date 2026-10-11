---
type: Issue
title: Leçon CE2 — Multiplier par des dizaines, par 4 et par 8
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-1-ce2.md"
order: 1500
---
## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique deux astuces de calcul mental multiplicatif, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 3.
- **Compétences liées** (`skillIds`) : `ce2-mul-by-tens`, `ce2-mul-by-4-8` — exercices : la fiche « Exercices CE2 — Multiplier par des dizaines, par 4 et par 8 ».
- **Objectif** : calculer 7 × 30 comme 7 × 3 dizaines ; multiplier par 4 en doublant deux fois, par 8 en doublant trois fois.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « dizaine », « double », « produit ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Multiplier par des dizaines, par 4 et par 8 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Des dizaines** — 7 × 30 : 7 rangées de 3 barres de dix. | `arrayGrid` + `tensUnits` |
| 3 | **21 dizaines** — 7 × 3 = 21, donc 7 × 3 dizaines = 21 dizaines = 210. | `calcChain` + `placeValueTable` |
| 4 | **À toi** — Interaction : 6 × 40. | `calcChain` (interactif) |
| 5 | **× 4** — 35 × 4 : double → 70, double → 140. | `calcChain` |
| 6 | **Pourquoi** — La grille de 4 rangées se coupe en deux grilles de 2. | `arrayGrid` (split) |
| 7 | **× 8** — 25 × 8 : 50, 100, 200 : trois doubles. | `calcChain` |
| 8 | **À toi** — Interaction : 45 × 4 en deux doubles. | `calcChain` (interactif) |
| 9 | **Course éclair** — 8 × 20, 15 × 4, 12 × 8. | `calcChain` |
| 10 | **Je retiens** — « 7 × 30 = 7 × 3 dizaines = 21 dizaines = 210. × 4 : double du double. × 8 : encore une fois le double. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `calcChain`, `placeValueTable`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 7 × 30 = ?
2. 35 × 4 = ? (QCM : 120, 140, 39)
3. 25 × 8 = ?
4. Multiplier par 4, c'est prendre le … du double.

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-calcul-mental-multiplicatif.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-mul-by-tens`, `ce2-mul-by-4-8`.
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
