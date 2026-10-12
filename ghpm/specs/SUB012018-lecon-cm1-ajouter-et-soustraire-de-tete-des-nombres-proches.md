---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZ7M
rev: 2026-10-11T18:50:11Z
url: https://github.com/georgesp/Educator/issues/172
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012018 — Leçon CM1 — Ajouter et soustraire de tête des nombres proches d'une dizaine
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1350
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique comment ajouter ou soustraire vite un nombre proche d'une dizaine, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 1.
- **Compétences liées** (`skillIds`) : `cm1-add-sub-near-tens` — exercices : la fiche « Exercices CM1 — Ajouter et soustraire de tête des nombres proches d'une dizaine ».
- **Objectif** : arrondir à la dizaine supérieure puis compenser, pour l'addition comme pour la soustraction.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « arrondir », « compenser ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Ajouter et soustraire de tête des nombres proches d'une dizaine » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Pourquoi arrondir** — Ajouter 40 est plus facile qu'ajouter 38. | `numberLine` (drapeaux) |
| 3 | **Ajouter** — 2 347 + 38 : + 40 → 2 387, − 2 → 2 385. | `calcChain` |
| 4 | **Soustraire** — 1 506 − 29 : − 30 → 1 476, + 1 → 1 477. | `calcChain` |
| 5 | **Le sens de la compensation** — Côte à côte : ajouter 38 (+ 40 − 2), soustraire 38 (− 40 + 2). | `calcChain` (comparaison) |
| 6 | **À toi** — Interaction : 615 + 19. | `calcChain` (interactif) |
| 7 | **Passer la centaine** — 1 495 + 18 : + 20 → 1 515, − 2 → 1 513. | `numberLine` |
| 8 | **À toi** — Interaction : 803 − 39. | `calcChain` (interactif) |
| 9 | **Course éclair** — Quatre calculs mélangés, sans chrono imposé. | `numberLine` |
| 10 | **Je retiens** — « J'arrondis à la dizaine (38 → 40), je calcule, puis je compense : + 40 − 2, ou − 40 + 2. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `numberLine`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 2 347 + 38 = ?
2. 1 506 − 29 = ? (QCM : 1 475, 1 477, 1 535)
3. 803 − 39 = ?
4. Pour soustraire 18 : − 20 puis …

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-ajouter-soustraire-de-tete.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-add-sub-near-tens`.
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
