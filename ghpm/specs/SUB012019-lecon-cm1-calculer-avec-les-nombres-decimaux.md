---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZ8Y
url: https://github.com/georgesp/Educator/issues/173
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012019 — Leçon CM1 — Calculer avec les nombres décimaux
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1980
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique comment calculer avec des nombres à virgule, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 4.
- **Compétences liées** (`skillIds`) : `cm1-dec-add-sub`, `cm1-dec-mul-div-10`, `cm1-dec-mul-int` — exercices : la fiche « Exercices CM1 — Calculer avec les nombres décimaux ».
- **Objectif** : poser additions et soustractions en alignant les virgules, multiplier et diviser par 10, multiplier un décimal par un entier.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « virgule », « dixième », « centième », « produit ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Calculer avec les nombres décimaux » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Aligner les virgules** — 12,5 + 3,75 : 12,50 au-dessus de 3,75, virgule sous virgule. | `columnOperation` (décimal) |
| 3 | **Les zéros** — 8,4 − 2,65 : on écrit 8,40 pour avoir des centièmes. | `columnOperation` (soustraction) |
| 4 | **À toi** — Interaction : 25,3 + 4,87. | `columnOperation` (interactif) |
| 5 | **De tête** — 4,6 + 3 = 7,6 : seule la partie entière change. | `placeValueTable` |
| 6 | **× 10 et ÷ 10** — 3,6 × 10 = 36 ; 3,6 ÷ 10 = 0,36 : les chiffres glissent. | `placeValueTable` (shift) |
| 7 | **× un entier** — 4,25 × 3 : 425 centièmes × 3 = 1 275 centièmes = 12,75. | `columnOperation` (multiplication) |
| 8 | **Estimer** — 4,25 × 3 ≈ 4 × 3 = 12 : la virgule est au bon endroit. | `calcChain` |
| 9 | **À toi** — Interaction : 2,8 × 6 posée. | `columnOperation` (interactif) |
| 10 | **Je retiens** — « J'aligne les virgules. × 10 : chaque chiffre avance d'un rang ; ÷ 10 : il recule d'un rang. » | `recapCard` |

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

1. 12,5 + 3,75 = ?
2. 3,6 × 10 = ? (QCM : 3,60, 36, 360)
3. 8,4 − 2,65 = ?
4. 4,25 × 3 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-calculer-avec-les-decimaux.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-dec-add-sub`, `cm1-dec-mul-div-10`, `cm1-dec-mul-int`.
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
