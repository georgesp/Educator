---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaCE
url: https://github.com/georgesp/Educator/issues/177
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012023 — Leçon CM1 — La division euclidienne
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1880
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique ce que sont le quotient et le reste, et comment poser une division, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 3.
- **Compétences liées** (`skillIds`) : `cm1-div-remainder`, `cm1-div-column` — exercices : la fiche « Exercices CM1 — La division euclidienne ».
- **Objectif** : trouver combien de fois le diviseur tient dans le dividende et ce qui reste, puis poser la division.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « dividende », « diviseur », « quotient », « reste ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « La division euclidienne » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Un partage qui ne tombe pas juste** — 38 pneus pour des voitures de 5… 7 voitures, il reste 3 pneus. | `groupObjects` (groupBy 5) |
| 3 | **L'écrire** — 38 = 5 × 7 + 3 ; quotient 7, reste 3. | `equation` |
| 4 | **Le reste est petit** — S'il restait 6 pneus, on pourrait faire une voiture de plus. | `groupObjects` |
| 5 | **À toi** — Interaction : 29 casques en paquets de 4. | `groupObjects` (groupBy, interactif) |
| 6 | **Poser** — 847 ÷ 6 : combien de fois 6 dans 8 ? 1, reste 2 ; on abaisse le 4… | `columnOperation` (division) |
| 7 | **Le zéro au quotient** — 847 ÷ 4 : 4 dans 4, 1 ; 4 dans 0… 0 ! → 211, reste 3. | `columnOperation` |
| 8 | **Vérifier** — 6 × 141 + 1 = 847. | `equation` |
| 9 | **À toi** — Interaction : 965 ÷ 7 posée. | `columnOperation` (interactif) |
| 10 | **Je retiens** — « 38 = 5 × 7 + 3 : 7 est le quotient, 3 le reste. Le reste est toujours plus petit que le diviseur. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `columnOperation`, `equation`, `groupObjects`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 38 ÷ 5 : quotient et reste ?
2. Dans 847 ÷ 6, quel est le quotient ? (QCM : 140, 141, 14)
3. Le reste peut-il être 7 quand on divise par 5 ?
4. 57 élèves, voitures de 4 places : combien de voitures faut-il ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-division-euclidienne.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-div-remainder`, `cm1-div-column`.
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
