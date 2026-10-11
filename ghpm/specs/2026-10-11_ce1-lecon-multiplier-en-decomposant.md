---
type: Issue
title: Leçon CE1 — Multiplier en décomposant
status: Todo
size: M
parent: "#1"
order: 1255
---
## User story

En tant qu'enfant de CE1, je veux une leçon courte et animée qui m'explique comment calculer 13 × 7 de tête, avant de m'entraîner.

## Leçon

- **Classe / période** : CE1, période 5.
- **Compétences liées** (`skillIds`) : `ce1-mul-distributive` — exercices : la fiche « Exercices CE1 — Multiplier en décomposant ».
- **Objectif** : couper le nombre de 11 à 19 en 10 et ses unités, multiplier chaque partie et ajouter.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « décomposer », « fois », « produit ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Multiplier en décomposant » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Une grande grille** — 13 rangées de 7 voitures : trop pour compter une par une. | `arrayGrid` (13 × 7) |
| 3 | **Couper** — On coupe la grille : 10 rangées + 3 rangées. | `arrayGrid` (split 10 / 3) |
| 4 | **Deux petits calculs** — 10 × 7 = 70 et 3 × 7 = 21. | `arrayGrid` + `equation` |
| 5 | **Ajouter** — 70 + 21 = 91. | `calcChain` |
| 6 | **À toi** — Interaction : coupe la grille 16 × 4 et calcule. | `arrayGrid` (interactif) |
| 7 | **Autre exemple** — 12 × 5 = 50 + 10 = 60. | `calcChain` |
| 8 | **À toi** — Interaction : complète 14 × 3 = 10 × 3 + … × 3. | `equation` (interactif) |
| 9 | **Course éclair** — 11 × 6, 15 × 2, 17 × 3. | `calcChain` |
| 10 | **Je retiens** — « 13 × 7 = 10 × 7 + 3 × 7 = 70 + 21 = 91. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `calcChain`, `equation`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 13 × 7 = ?
2. 16 × 4 = ? (QCM : 46, 64, 74)
3. 14 × 3 = 10 × 3 + … × 3
4. 12 × 5 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce1-multiplier-en-decomposant.json`, validée par `lesson.schema.json`, `skillIds` = `ce1-mul-distributive`.
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
