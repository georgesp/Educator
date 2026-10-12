---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaoc
url: https://github.com/georgesp/Educator/issues/210
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012122 — Leçon CM2 — Opérations posées, estimations et parenthèses"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 2180
---

## User story

En tant qu'enfant de CM2, je veux une leçon courte et animée qui m'explique comment calculer juste et vérifier ses calculs, avant de m'entraîner.

## Leçon

- **Classe / période** : CM2, période 1.
- **Compétences liées** (`skillIds`) : `cm2-column-integers`, `cm2-estimate`, `cm2-parentheses` — exercices : la fiche « Exercices CM2 — Opérations posées, estimations et parenthèses ».
- **Objectif** : estimer avant de calculer, poser les opérations, calculer des expressions à deux paires de parenthèses.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « ordre de grandeur », « estimer », « parenthèses », « expression ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Opérations posées, estimations et parenthèses » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Estimer** — 498 × 21 ≈ 500 × 20 = 10 000. | `calcChain` |
| 3 | **À toi** — Interaction : choisis l'ordre de grandeur de 3 120 + 6 970. | `calcChain` (QCM) |
| 4 | **Multiplier par 307** — 4 058 × 307 : la ligne des dizaines vaut 0, on peut la sauter mais on décale de deux rangs. | `columnOperation` (multiplication) |
| 5 | **À toi** — Interaction : 2 615 × 48. | `columnOperation` (interactif) |
| 6 | **Deux parenthèses** — (45 − 15) × (3 + 7) = 30 × 10 = 300. | `equation` (étapes) |
| 7 | **Traduire une histoire** — « 3 lots de 4 casques et 2 lots de 5 » → 3 × 4 + 2 × 5. | `barModel` + `equation` |
| 8 | **À toi** — Interaction : relie chaque expression à son histoire. | `barModel` (association) |
| 9 | **Vérifier** — Le résultat est-il proche de l'estimation ? | `calcChain` |
| 10 | **Je retiens** — « J'estime, je calcule, je compare : si le résultat est loin de l'estimation, je recommence. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `barModel`, `calcChain`, `columnOperation`, `equation`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 498 × 21 ≈ ? (QCM : 1 000, 10 000, 100 000)
2. (45 − 15) × (3 + 7) = ?
3. 2 615 × 48 = ?
4. Que calcule-t-on en premier dans 100 − (20 + 5) × 2 ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm2-operations-posees-et-parentheses.json`, validée par `lesson.schema.json`, `skillIds` = `cm2-column-integers`, `cm2-estimate`, `cm2-parentheses`.
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
