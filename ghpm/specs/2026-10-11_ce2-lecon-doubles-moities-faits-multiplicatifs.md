---
type: Issue
title: Leçon CE2 — Doubles, moitiés et faits multiplicatifs usuels
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-1-ce2.md"
order: 1460
---
## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique ce que sont un double et une moitié, et les nombres à connaître par cœur, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 2.
- **Compétences liées** (`skillIds`) : `ce2-double-half`, `ce2-mul-facts-25-60` — exercices : la fiche « Exercices CE2 — Doubles, moitiés et faits multiplicatifs usuels ».
- **Objectif** : relier double et moitié, calculer le double ou la moitié en décomposant, connaître les multiples de 25 et les décompositions de 60.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « double », « moitié », « multiple ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Doubles, moitiés et faits multiplicatifs usuels » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Le double** — Deux garages identiques : le double de 8 est 16. | `groupObjects` (deux rangées) |
| 3 | **La moitié** — 16 voitures partagées en 2 équipes égales : 8 chacune. | `groupObjects` (share) |
| 4 | **Décomposer** — Double de 17 = double de 10 + double de 7 = 34 ; moitié de 36 = 15 + 3 = 18. | `calcChain` + `tensUnits` |
| 5 | **À toi** — Interaction : partage 28 casques en deux. | `groupObjects` (share, interactif) |
| 6 | **Les pièces de 25** — 25, 50, 75, 100 : 4 × 25 = 100. | `numberLine` (pas de 25) |
| 7 | **Les façons de faire 60** — Grilles 2 × 30, 3 × 20, 4 × 15, 5 × 12, 6 × 10. | `arrayGrid` |
| 8 | **Grands doubles** — Double de 75 = 150 ; double de 600 = 1 200. | `calcChain` |
| 9 | **À toi** — Interaction : relie chaque nombre à son double. | `calcChain` (association) |
| 10 | **Je retiens** — « Le double, c'est 2 fois le nombre. La moitié, c'est partager en 2 parts égales. 4 × 25 = 100. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `calcChain`, `groupObjects`, `numberLine`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Double de 17 ?
2. Moitié de 36 ? (QCM : 13, 18, 16)
3. 3 × 25 = ?
4. 60 = 4 × …

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-doubles-moities-faits-multiplicatifs.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-double-half`, `ce2-mul-facts-25-60`.
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
