---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaik
url: https://github.com/georgesp/Educator/issues/205
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012117 — Leçon CM2 — La division posée d'un entier ou d'un décimal
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 2200
---

## User story

En tant qu'enfant de CM2, je veux une leçon courte et animée qui m'explique comment continuer une division après la virgule, avant de m'entraîner.

## Leçon

- **Classe / période** : CM2, période 1.
- **Compétences liées** (`skillIds`) : `cm2-div-integer`, `cm2-div-decimal` — exercices : la fiche « Exercices CM2 — La division posée d'un entier ou d'un décimal ».
- **Objectif** : poser une division euclidienne, puis la poursuivre après la virgule pour obtenir un quotient décimal.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « dividende », « diviseur », « quotient », « reste », « quotient décimal ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « La division posée d'un entier ou d'un décimal » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Rappel** — 5 728 ÷ 7 : la potence, q = 818, r = 2. | `columnOperation` (division) |
| 3 | **Vérifier** — 7 × 818 + 2 = 5 728. | `equation` |
| 4 | **À toi** — Interaction : 3 465 ÷ 6. | `columnOperation` (interactif) |
| 5 | **Partager sans reste** — 45 € pour 4 pilotes : 11 € chacun, il reste 1 € à partager. | `groupObjects` (share) |
| 6 | **Après la virgule** — 1 € = 10 dixièmes ; 10 ÷ 4 = 2, reste 2 ; 20 centièmes ÷ 4 = 5 → 11,25. | `columnOperation` (division décimale) |
| 7 | **Un décimal à diviser** — 7,2 ÷ 3 : 7 ÷ 3 = 2 reste 1, la virgule, 12 dixièmes ÷ 3 = 4 → 2,4. | `columnOperation` |
| 8 | **À toi** — Interaction : 9,6 ÷ 4. | `columnOperation` (interactif) |
| 9 | **Estimer** — 45 ÷ 4 ≈ 44 ÷ 4 = 11 : 11,25 est vraisemblable. | `calcChain` |
| 10 | **Je retiens** — « Quand il reste quelque chose, j'écris une virgule au quotient, j'ajoute un 0 au dividende et je continue. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `columnOperation`, `equation`, `groupObjects`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 5 728 ÷ 7 : quotient et reste ?
2. 45 ÷ 4 = ? (QCM : 11,1, 11,25, 1 125)
3. 7,2 ÷ 3 = ?
4. Que fait-on quand il reste 1 et qu'on veut continuer ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm2-division-posee.json`, validée par `lesson.schema.json`, `skillIds` = `cm2-div-integer`, `cm2-div-decimal`.
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
