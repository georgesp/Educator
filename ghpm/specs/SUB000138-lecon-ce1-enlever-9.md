---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYYY
rev: 2026-10-11T18:48:39Z
url: https://github.com/georgesp/Educator/issues/106
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000138 — Leçon CE1 — Enlever 9
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 800
---

## User story

En tant qu'enfant de CE1, je veux une leçon courte et animée qui m'explique l'astuce pour enlever 9 très vite, avant de m'entraîner.

## Leçon

- **Classe / période** : CE1, période 2.
- **Compétences liées** (`skillIds`) : `ce1-sub-9` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : soustraire 9 en retirant 10 puis en ajoutant 1.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « soustraire », « dizaine ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Enlever 9 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **9, presque 10** — Enlever 9, c'est enlever 10 mais rendre 1. | `tensUnits` |
| 3 | **L'astuce** — 52 − 9 : − 10 → 42, + 1 → 43. | `calcChain` |
| 4 | **Sur la piste** — Un grand bond en arrière de 10, un petit pas en avant. | `numberLine` |
| 5 | **Pourquoi + 1 ?** — On a enlevé un cube de trop : on le remet. | `tensUnits` |
| 6 | **À toi** — Interaction : 64 − 9 en deux étapes. | `calcChain` (interactif) |
| 7 | **Attention** — Erreur fréquente : 52 − 9 ≠ 41 ; la voiture montre pourquoi. | `numberLine` |
| 8 | **À toi** — Interaction : 30 − … = 21. | `equation` (interactif) |
| 9 | **Course éclair** — Trois calculs : 45 − 9, 71 − 9, 100 − 9. | `calcChain` |
| 10 | **Je retiens** — « Pour enlever 9, j'enlève 10 puis j'ajoute 1. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `equation`, `numberLine`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 45 − 9 = ?
2. 71 − 9 = ? (QCM : 60, 62, 61)
3. … − 9 = 30
4. Pour enlever 9 : j'enlève 10 puis …

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce1-sub-9.json`, validée par `lesson.schema.json`, `skillIds` = `ce1-sub-9`.
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
