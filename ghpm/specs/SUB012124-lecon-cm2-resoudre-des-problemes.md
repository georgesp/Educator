---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACarM
rev: 2026-10-11T18:51:28Z
url: https://github.com/georgesp/Educator/issues/212
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012124 — Leçon CM2 — Résoudre des problèmes
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 1880
---

## User story

En tant qu'enfant de CM2, je veux une leçon courte et animée qui m'explique comment s'organiser face à un problème long, avant de m'entraîner.

## Leçon

- **Classe / période** : CM2, période 1.
- **Compétences liées** (`skillIds`) : `cm2-problems-multi-step`, `cm2-problems-optimization` — exercices : la fiche « Exercices CM2 — Résoudre des problèmes ».
- **Objectif** : planifier les étapes, choisir les opérations, utiliser schémas et tableaux, rédiger une réponse et contrôler.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « étape », « donnée utile », « donnée inutile », « schéma », « unité ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Résoudre des problèmes » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Lire et trier** — Les données utiles s'allument, la donnée inutile se grise. | `barModel` (vide) |
| 3 | **Planifier** — Étape 1 : 12 tours ; étape 2 : 2/3 de tour ; étape 3 : total. | `calcChain` |
| 4 | **Schématiser** — Le schéma en barres de la course. | `barModel` |
| 5 | **À toi** — Interaction : remets les étapes dans l'ordre. | `calcChain` (ordre, interactif) |
| 6 | **Optimiser** — Pack ou pneus à l'unité : le tableau compare les prix. | `proportionTable` |
| 7 | **Algorithme** — Programme la voiture : avance 3, tourne, avance 2. | `arrayGrid` |
| 8 | **À toi** — Interaction : écris le programme pour atteindre le drapeau. | `arrayGrid` (interactif) |
| 9 | **Contrôler** — Unité, ordre de grandeur, la réponse répond-elle à la question ? | `calcChain` |
| 10 | **Je retiens** — « Je planifie mes étapes avant de calculer, et je contrôle ma réponse : unité, ordre de grandeur, sens. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `barModel`, `calcChain`, `proportionTable`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Problème en trois étapes (saisie)
2. Quelle donnée est inutile ? (QCM)
3. Le moins cher pour 10 pneus ? (QCM)
4. Quelle unité pour la réponse ? (QCM)

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm2-resoudre-des-problemes.json`, validée par `lesson.schema.json`, `skillIds` = `cm2-problems-multi-step`, `cm2-problems-optimization`.
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
