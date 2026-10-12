---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACagQ
rev: 2026-10-11T18:51:01Z
url: https://github.com/georgesp/Educator/issues/203
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012115 — Leçon CM2 — Calcul mental sur les entiers
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 1700
---

## User story

En tant qu'enfant de CM2, je veux une leçon courte et animée qui m'explique les astuces de calcul mental du CM2, avant de m'entraîner.

## Leçon

- **Classe / période** : CM2, période 1.
- **Compétences liées** (`skillIds`) : `cm2-add-sub-near-tens`, `cm2-mul-round`, `cm2-div-by-4-8` — exercices : la fiche « Exercices CM2 — Calcul mental sur les entiers ».
- **Objectif** : compenser autour de 100, compter les zéros dans les produits de nombres ronds, diviser par 4 ou 8 par moitiés successives.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « compenser », « moitié », « ordre de grandeur ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Calcul mental sur les entiers » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Proche de 100** — 4 562 + 98 : + 100 → 4 662, − 2 → 4 660. | `calcChain` |
| 3 | **− 99** — 7 015 − 99 : − 100 → 6 915, + 1 → 6 916. | `calcChain` |
| 4 | **À toi** — Interaction : 3 248 + 59. | `calcChain` (interactif) |
| 5 | **Nombres ronds** — 30 × 400 = 3 × 4 × 10 × 100 = 12 × 1 000 = 12 000. | `placeValueTable` + `calcChain` |
| 6 | **À toi** — Interaction : 6 × 7 000 et 50 × 80. | `calcChain` (interactif) |
| 7 | **÷ 4** — 360 ÷ 4 : moitié 180, moitié 90. | `calcChain` |
| 8 | **÷ 8** — 360 ÷ 8 : 180, 90, 45. | `calcChain` |
| 9 | **Distributivité** — 102 × 7 = 100 × 7 + 2 × 7 = 714. | `arrayGrid` (split) |
| 10 | **Je retiens** — « + 98 : + 100 − 2. 30 × 400 = 3 × 4 × 1 000. ÷ 4 : la moitié de la moitié. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `calcChain`, `placeValueTable`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 4 562 + 98 = ?
2. 30 × 400 = ? (QCM : 1 200, 12 000, 120 000)
3. 360 ÷ 8 = ?
4. 102 × 7 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm2-calcul-mental-entiers.json`, validée par `lesson.schema.json`, `skillIds` = `cm2-add-sub-near-tens`, `cm2-mul-round`, `cm2-div-by-4-8`.
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
