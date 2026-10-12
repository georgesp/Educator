---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaeQ
url: https://github.com/georgesp/Educator/issues/202
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012114 — Leçon CM2 — Calculer avec les décimaux
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 2280
---

## User story

En tant qu'enfant de CM2, je veux une leçon courte et animée qui m'explique des astuces pour calculer vite avec des nombres à virgule, avant de m'entraîner.

## Leçon

- **Classe / période** : CM2, période 1.
- **Compétences liées** (`skillIds`) : `cm2-dec-mental`, `cm2-dec-mul-div-powers`, `cm2-dec-mul-5-50`, `cm2-dec-mul-int` — exercices : la fiche « Exercices CM2 — Calculer avec les décimaux ».
- **Objectif** : ajouter des décimaux de tête, déplacer les chiffres pour × et ÷ 10, 100, 1 000, multiplier par 5 et 50.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « dixième », « centième », « millième », « double », « moitié ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Calculer avec les décimaux » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Ajouter des dixièmes** — 3,7 + 5,6 : 8 unités + 13 dixièmes = 9,3. | `calcChain` + `placeValueTable` |
| 3 | **À toi** — Interaction : 4,8 + 2,5. | `calcChain` (interactif) |
| 4 | **÷ 100** — 4,5 ÷ 100 : les chiffres reculent de deux rangs → 0,045. | `placeValueTable` (shift) |
| 5 | **× 1 000** — 0,032 × 1 000 = 32. | `placeValueTable` (shift) |
| 6 | **Double et moitié** — Double de 2,4 = 4,8 ; moitié de 2,4 = 1,2. | `calcChain` |
| 7 | **× 5 et × 50** — 2,4 × 50 = 2,4 × 100 ÷ 2 = 120. | `calcChain` |
| 8 | **Posé** — 12,35 × 24 : on calcule 1 235 × 24, puis on place la virgule (deux décimales). | `columnOperation` (multiplication) |
| 9 | **À toi** — Interaction : 3,6 × 15 posée. | `columnOperation` (interactif) |
| 10 | **Je retiens** — « × 10, 100, 1 000 : les chiffres avancent de 1, 2, 3 rangs ; ÷ : ils reculent. × 5 = × 10 puis la moitié. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `columnOperation`, `placeValueTable`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 3,7 + 5,6 = ?
2. 4,5 ÷ 100 = ? (QCM : 0,45, 0,045, 450)
3. 2,4 × 50 = ?
4. Moitié de 2,4 ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm2-calcul-decimaux.json`, validée par `lesson.schema.json`, `skillIds` = `cm2-dec-mental`, `cm2-dec-mul-div-powers`, `cm2-dec-mul-5-50`, `cm2-dec-mul-int`.
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
