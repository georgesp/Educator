---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYPU
rev: 2026-10-12T19:53:43Z
url: https://github.com/georgesp/Educator/issues/101
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB000133 — Leçon CE1 — Ajouter 9, 19 ou 29"
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 662
---

## User story

En tant qu'enfant de CE1, je veux une leçon courte et animée qui m'explique l'astuce pour ajouter 9, 19 ou 29, avant de m'entraîner.

## Leçon

- **Classe / période** : CE1, période 2.
- **Compétences liées** (`skillIds`) : `ce1-add-9-19-29` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : ajouter 9, 19 ou 29 en ajoutant 10, 20 ou 30 puis en retirant 1.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « dizaine », « ajouter », « enlever ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Ajouter 9, 19 ou 29 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Rappel** — Ajouter 9 : + 10 puis − 1 (vu au CP). | `calcChain` |
| 3 | **19, presque 20** — Deux barres avec un cube en moins : 19 = 20 − 1. | `tensUnits` |
| 4 | **+ 19** — 46 + 19 : + 20 → 66, − 1 → 65. | `calcChain` + `numberLine` |
| 5 | **29, presque 30** — 29 = 30 − 1 ; 34 + 29 : + 30 → 64, − 1 → 63. | `calcChain` |
| 6 | **À toi** — Interaction : 57 + 19 en deux bonds. | `numberLine` (interactif) |
| 7 | **Attention** — On enlève 1 à la fin, on n'en ajoute pas. | `calcChain` |
| 8 | **À toi** — Interaction : relie chaque calcul à sa chaîne (+ 9, + 19, + 29). | `calcChain` (association) |
| 9 | **Course éclair** — 35 + 9, 35 + 19, 35 + 29 : que remarques-tu ? | `numberLine` |
| 10 | **Je retiens** — « 9 = 10 − 1, 19 = 20 − 1, 29 = 30 − 1 : j'ajoute la dizaine ronde puis j'enlève 1. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `numberLine`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 28 + 9 = ?
2. 46 + 19 = ? (QCM : 64, 65, 66)
3. 51 + 29 = ?
4. Pour ajouter 29, j'ajoute … puis j'enlève 1.

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [x] Leçon `content/lessons/math/ce1-add-9-19-29.json`, validée par `lesson.schema.json`, `skillIds` = `ce1-add-9-19-29`.
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
