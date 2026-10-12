---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaAs
url: https://github.com/georgesp/Educator/issues/176
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012022 — Leçon CM1 — Distributivité et calculs avec parenthèses
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1840
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique comment calculer en décomposant et à quoi servent les parenthèses, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 4.
- **Compétences liées** (`skillIds`) : `cm1-distributive`, `cm1-parentheses` — exercices : la fiche « Exercices CM1 — Distributivité et calculs avec parenthèses ».
- **Objectif** : utiliser la distributivité (23 × 6 = 20 × 6 + 3 × 6) et effectuer d'abord le calcul entre parenthèses.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « parenthèses », « décomposer », « produit », « somme ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Distributivité et calculs avec parenthèses » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Couper la grille** — 23 × 6 : la grille se coupe en 20 × 6 et 3 × 6. | `arrayGrid` (split) |
| 3 | **Ajouter les morceaux** — 120 + 18 = 138. | `calcChain` |
| 4 | **À toi** — Interaction : coupe 47 × 4 et calcule. | `arrayGrid` (interactif) |
| 5 | **Les parenthèses** — (12 + 8) × 5 : on calcule d'abord 12 + 8 = 20, puis × 5 = 100. | `equation` (étapes) |
| 6 | **Sans parenthèses ?** — 12 + 8 × 5 n'est pas le même calcul : la grille change. | `arrayGrid` + `equation` |
| 7 | **Soustraction** — 50 − (18 + 7) = 50 − 25 = 25. | `equation` |
| 8 | **À toi** — Interaction : relie chaque calcul à son histoire. | `barModel` (association) |
| 9 | **Estimer** — 23 × 6 ≈ 20 × 6 = 120 : 138 est proche. | `calcChain` |
| 10 | **Je retiens** — « Je calcule d'abord ce qui est entre parenthèses. Pour multiplier un grand nombre, je le coupe en morceaux faciles. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `barModel`, `calcChain`, `equation`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 23 × 6 = ?
2. (12 + 8) × 5 = ? (QCM : 52, 100, 60)
3. 50 − (18 + 7) = ?
4. Que calcule-t-on en premier dans 4 × (6 + 3) ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-distributivite-parentheses.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-distributive`, `cm1-parentheses`.
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
