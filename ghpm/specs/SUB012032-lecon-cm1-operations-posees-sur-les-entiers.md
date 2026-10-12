---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaNs
url: https://github.com/georgesp/Educator/issues/186
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012032 — Leçon CM1 — Opérations posées sur les entiers
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1860
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique comment poser des opérations avec de grands nombres sans se tromper, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 1.
- **Compétences liées** (`skillIds`) : `cm1-add-sub-column`, `cm1-mul-column` — exercices : la fiche « Exercices CM1 — Opérations posées sur les entiers ».
- **Objectif** : poser les trois opérations sur de grands nombres, contrôler par un ordre de grandeur.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « somme », « différence », « produit », « ordre de grandeur », « retenue ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Opérations posées sur les entiers » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Estimer d'abord** — 326 × 47 ≈ 300 × 50 = 15 000. | `calcChain` |
| 3 | **Addition** — 45 708 + 9 386 : retenues en chaîne. | `columnOperation` (addition) |
| 4 | **Soustraction et zéros** — 60 004 − 7 568 : cassages en cascade, ou compensations successives, selon la technique de l'enfant. | `columnOperation` (soustraction, deux techniques) + `tensUnits` |
| 5 | **À toi** — Interaction : 82 315 − 9 487. | `columnOperation` (interactif) |
| 6 | **Multiplication** — 326 × 47 = 326 × 7 + 326 × 40. | `columnOperation` (multiplication) + `arrayGrid` (split) |
| 7 | **Le décalage** — La 2e ligne commence au rang des dizaines. | `columnOperation` |
| 8 | **À toi** — Interaction : 214 × 36. | `columnOperation` (interactif) |
| 9 | **Vérifier** — 15 322 est proche de 15 000 : vraisemblable. | `calcChain` |
| 10 | **Je retiens** — « J'aligne, je commence par les unités, je n'oublie ni les retenues ni le décalage, et je vérifie l'ordre de grandeur. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `calcChain`, `columnOperation`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 45 708 + 9 386 = ?
2. 60 004 − 7 568 = ? (QCM : 52 436, 53 436, 52 536)
3. 326 × 47 = ?
4. Un ordre de grandeur de 498 × 21 ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-operations-posees-entiers.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-add-sub-column`, `cm1-mul-column`.
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
