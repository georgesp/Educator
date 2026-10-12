---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaKo
url: https://github.com/georgesp/Educator/issues/183
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012029 — Leçon CM1 — Multiplier par 10, 100, 1 000 et par des dizaines ou des centaines"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1800
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique pourquoi multiplier par 10, 100 ou 1 000 décale les chiffres, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 1.
- **Compétences liées** (`skillIds`) : `cm1-mul-10-100-1000`, `cm1-mul-tens-hundreds` — exercices : la fiche « Exercices CM1 — Multiplier par 10, 100, 1 000 et par des dizaines ou des centaines ».
- **Objectif** : faire glisser les chiffres dans le tableau de numération pour × 10, × 100, × 1 000 ; calculer 7 × 300 comme 21 centaines.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « rang », « dizaine », « centaine », « millier ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Multiplier par 10, 100, 1 000 et par des dizaines ou des centaines » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **× 10** — Chaque chiffre de 45 avance d'un rang : 450. | `placeValueTable` (shift) |
| 3 | **× 100 et × 1 000** — 45 × 100 = 4 500 ; 45 × 1 000 = 45 000. | `placeValueTable` (shift 2, 3) |
| 4 | **Pas « ajouter un zéro »** — On ne colle pas des zéros : les chiffres changent de valeur. | `placeValueTable` |
| 5 | **À toi** — Interaction : fais glisser les chiffres de 307 pour × 100. | `placeValueTable` (interactif) |
| 6 | **Des centaines** — 7 × 300 : 7 rangées de 3 plaques de cent. | `arrayGrid` + `tensUnits` |
| 7 | **21 centaines** — 7 × 3 = 21 → 21 centaines = 2 100. | `calcChain` |
| 8 | **Le piège du 5 × 4** — 5 × 400 = 20 centaines = 2 000. | `calcChain` |
| 9 | **À toi** — Interaction : 60 × 8 et 9 × 700. | `calcChain` (interactif) |
| 10 | **Je retiens** — « × 10, × 100, × 1 000 : chaque chiffre avance de 1, 2 ou 3 rangs. 7 × 300 = 7 × 3 centaines = 21 centaines = 2 100. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `calcChain`, `placeValueTable`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 38 × 1 000 = ?
2. 7 × 300 = ? (QCM : 210, 2 100, 21 000)
3. … × 100 = 4 500
4. 5 × 400 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-multiplier-par-10-100-1000.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-mul-10-100-1000`, `cm1-mul-tens-hundreds`.
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
