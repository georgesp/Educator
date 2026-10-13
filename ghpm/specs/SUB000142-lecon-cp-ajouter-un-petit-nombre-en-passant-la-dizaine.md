---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYfM
rev: 2026-10-13T18:57:42Z
url: https://github.com/georgesp/Educator/issues/110
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000142 — Leçon CP — Ajouter un petit nombre en passant la dizaine
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2110
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique comment ajouter un petit nombre quand on dépasse la dizaine, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 4.
- **Compétences liées** (`skillIds`) : `cp-add-small-crossing` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : ajouter un nombre inférieur à 10 en complétant d'abord jusqu'à la dizaine supérieure.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « dizaine », « compléter », « décomposer ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Ajouter un petit nombre en passant la dizaine » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Sans dépasser** — 32 + 4 = 36 : seules les unités changent. | `tensUnits` |
| 3 | **On dépasse** — 47 + 8 dépasse 50 : la voiture s'arrête au drapeau du 50. | `numberLine` (drapeau 50) |
| 4 | **Couper 8** — Pour aller de 47 à 50, il faut 3 : 8, c'est 3 et 5. | `tenFrame` + `calcChain` |
| 5 | **Le reste** — 50 + 5 = 55, donc 47 + 8 = 55. | `calcChain` |
| 6 | **À toi** — Interaction : 26 + 7, l'enfant choisit comment couper 7. | `calcChain` (interactif) |
| 7 | **Autre exemple** — 68 + 5 : 68 → 70 (+ 2), puis + 3 → 73. | `numberLine` |
| 8 | **À toi** — Interaction : 39 + 4 sur la piste. | `numberLine` (interactif) |
| 9 | **Vérifier** — On recompte avec les barres et les cubes pour vérifier. | `tensUnits` |
| 10 | **Je retiens** — « Je complète jusqu'à la dizaine, puis j'ajoute ce qu'il reste. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `numberLine`, `recapCard`, `tenFrame`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 38 + 5 = ?
2. 56 + 7 = ? (QCM : 62, 63, 64)
3. 29 + 4 = ?
4. 45 + 6 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [x] Leçon `content/lessons/math/cp-add-small-crossing.json`, validée par `lesson.schema.json`, `skillIds` = `cp-add-small-crossing`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [x] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [x] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [x] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [x] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
