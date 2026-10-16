---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZhY
rev: 2026-10-11T18:49:20Z
url: https://github.com/georgesp/Educator/issues/152
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011930 — Leçon CE2 — Soustraire 9, 19, 29 ou 39"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1000
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique l'astuce pour soustraire des nombres proches d'une dizaine, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 2.
- **Compétences liées** (`skillIds`) : `ce2-sub-near-tens` — exercices : la fiche « Exercices CE2 — Soustraire 9, 19, 29 ou 39 ».
- **Objectif** : soustraire 9, 19, 29 ou 39 en retirant la dizaine ronde puis en ajoutant 1.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « soustraire », « compenser », « différence ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Soustraire 9, 19, 29 ou 39 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Rappel** — Enlever 9 : − 10 puis + 1. | `calcChain` |
| 3 | **− 29** — 84 − 29 : − 30 → 54, + 1 → 55. | `calcChain` + `numberLine` |
| 4 | **Pourquoi + 1 ?** — On a retiré 30 voitures au lieu de 29 : on en remet une. | `groupObjects` (remove) |
| 5 | **− 39** — 312 − 39 : − 40 → 272, + 1 → 273. | `calcChain` |
| 6 | **À toi** — Interaction : 71 − 19 en deux bonds. | `numberLine` (interactif) |
| 7 | **Comparer les astuces** — Ajouter 29 : + 30 − 1. Enlever 29 : − 30 + 1. | `calcChain` (côte à côte) |
| 8 | **À toi** — Interaction : choisis la bonne chaîne pour 205 − 9. | `calcChain` (QCM) |
| 9 | **Course éclair** — Trois calculs mélangés. | `numberLine` |
| 10 | **Je retiens** — « Pour enlever 29, j'enlève 30 puis j'ajoute 1 : j'avais enlevé 1 de trop. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `groupObjects`, `numberLine`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 84 − 29 = ?
2. 312 − 39 = ? (QCM : 271, 273, 283)
3. 60 − 19 = ?
4. Pour enlever 39 : j'enlève 40 puis …

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-soustraire-9-19-29-39.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-sub-near-tens`.
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
