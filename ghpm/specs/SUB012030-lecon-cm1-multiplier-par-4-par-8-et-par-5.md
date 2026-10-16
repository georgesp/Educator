---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaL4
rev: 2026-10-11T18:50:17Z
url: https://github.com/georgesp/Educator/issues/184
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012030 — Leçon CM1 — Multiplier par 4, par 8 et par 5"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1390
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique trois astuces : × 4, × 8 et × 5, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 2.
- **Compétences liées** (`skillIds`) : `cm1-mul-by-4-8`, `cm1-mul-by-5` — exercices : la fiche « Exercices CM1 — Multiplier par 4, par 8 et par 5 ».
- **Objectif** : multiplier par 4 et 8 en doublant, par 5 en multipliant par 10 puis en prenant la moitié.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « double », « moitié », « produit ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Multiplier par 4, par 8 et par 5 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **× 4** — 45 × 4 : 90, puis 180. | `calcChain` |
| 3 | **× 8** — 45 × 8 : 90, 180, 360. | `calcChain` |
| 4 | **Pourquoi × 5** — 5, c'est la moitié de 10 : la grille de 10 colonnes se coupe en deux. | `arrayGrid` (split) |
| 5 | **× 5** — 5 × 64 : 640, moitié 320. | `calcChain` + `placeValueTable` |
| 6 | **Nombre impair** — 5 × 139 : 1 390, moitié 695. | `calcChain` |
| 7 | **À toi** — Interaction : 5 × 82. | `calcChain` (interactif) |
| 8 | **Choisir sa méthode** — 5 × 7 : la table suffit ; 5 × 100 : je sais ; 5 × 186 : l'astuce. | `calcChain` (QCM) |
| 9 | **À toi** — Interaction : 32 × 8 en trois doubles. | `calcChain` (interactif) |
| 10 | **Je retiens** — « × 4 : je double deux fois. × 8 : trois fois. × 5 : je multiplie par 10 puis je prends la moitié. » | `recapCard` |

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

1. 5 × 64 = ?
2. 45 × 8 = ? (QCM : 320, 360, 380)
3. 186 × 5 = ?
4. Pour × 5 : × 10 puis …

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-multiplier-par-4-8-5.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-mul-by-4-8`, `cm1-mul-by-5`.
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
