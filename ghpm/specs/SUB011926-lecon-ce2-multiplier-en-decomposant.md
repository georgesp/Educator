---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZbw
url: https://github.com/georgesp/Educator/issues/148
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011926 — Leçon CE2 — Multiplier en décomposant
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1520
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique comment multiplier 13 × 7 de tête, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 3.
- **Compétences liées** (`skillIds`) : `ce2-mul-distributive` — exercices : la fiche « Exercices CE2 — Multiplier en décomposant ».
- **Objectif** : décomposer le plus grand facteur en dizaines et unités et ajouter les deux produits (distributivité).
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « décomposer », « produit », « facteur ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Multiplier en décomposant » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Une grande grille** — 13 rangées de 7 voitures : trop pour compter une par une. | `arrayGrid` (13 × 7) |
| 3 | **Couper** — On coupe la grille : 10 rangées + 3 rangées. | `arrayGrid` (split 10 / 3) |
| 4 | **Deux petits produits** — 10 × 7 = 70 et 3 × 7 = 21. | `arrayGrid` + `equation` |
| 5 | **Ajouter** — 70 + 21 = 91. | `calcChain` |
| 6 | **À toi** — Interaction : coupe la grille 24 × 6 où tu veux, puis calcule. | `arrayGrid` (interactif) |
| 7 | **Autre exemple** — 45 × 3 = 40 × 3 + 5 × 3 = 120 + 15 = 135. | `calcChain` |
| 8 | **À toi** — Interaction : complète 16 × 4 = 10 × 4 + … × 4. | `equation` (interactif) |
| 9 | **Course éclair** — 12 × 5, 21 × 4, 17 × 3. | `calcChain` |
| 10 | **Je retiens** — « 13 × 7 = 10 × 7 + 3 × 7 = 70 + 21 = 91 : je coupe le grand nombre en dizaines et unités. » | `recapCard` |

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
2. 24 × 6 = ? (QCM : 124, 144, 146)
3. 16 × 4 = 10 × 4 + … × 4
4. 45 × 3 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-multiplier-en-decomposant.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-mul-distributive`.
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
