---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXvQE
url: https://github.com/georgesp/Educator/issues/325
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012046 — Leçon CM1 — Périmètres et aires
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 5220
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique la différence entre le périmètre et l'aire, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 4.
- **Compétences liées** (`skillIds`) : `cm1-perimeter`, `cm1-area-grid` — exercices : la fiche « Exercices CM1 — Périmètres et aires ».
- **Objectif** : le périmètre est la longueur du tour ; l'aire est la surface, comptée en carreaux.
- **Durée visée** : 3 à 5 minutes, 11 slides.
- **Vocabulaire du programme** : « périmètre », « aire », « polygone », « côté », « centimètre carré ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Périmètres et aires » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Le tour du circuit** — La voiture fait le tour de la piste : le contour s'allume, c'est le périmètre. | `polygonFigure` (contour) |
| 3 | **Ajouter les côtés** — Triangle de 5 cm, 4 cm et 6 cm : 5 + 4 + 6 = 15 cm. | `polygonFigure` + `calcChain` |
| 4 | **Le rectangle** — 12 m sur 7 m : 12 + 7 + 12 + 7 = 38 m ; pas de formule à apprendre. | `polygonFigure` + `calcChain` |
| 5 | **À toi** — Interaction : calcule le périmètre du pentagone. | `polygonFigure` (interactif) |
| 6 | **L'aire** — On colorie l'intérieur : c'est la surface de la figure. | `gridFigure` |
| 7 | **Compter les carreaux** — Chaque carreau est une unité d'aire : 12 carreaux. | `gridFigure` |
| 8 | **Les demi-carreaux** — Deux moitiés font un carreau : 12 + 2 = 14 carreaux. | `gridFigure` |
| 9 | **À toi** — Interaction : compte les carreaux de la figure. | `gridFigure` (interactif) |
| 10 | **Aire ou périmètre ?** — Deux figures de 6 carreaux : l'une a un périmètre de 10, l'autre de 14. | `gridFigure` (comparaison) |
| 11 | **Je retiens** — « Le périmètre, c'est la longueur du tour : j'ajoute les côtés. L'aire, c'est la surface : je compte les carreaux. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `gridFigure`, `polygonFigure`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » ; `gridFigure` et `polygonFigure` sont ajoutées par la fiche « Animations des leçons CM1 — heures, mesures, probabilités et données ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Périmètre d'un rectangle de 12 m sur 7 m ? (QCM : 19 m, 38 m, 84 m)
2. Périmètre d'un triangle de côtés 5 cm, 4 cm et 6 cm ?
3. Combien de carreaux pour l'aire de la figure ?
4. Deux figures de même aire ont-elles toujours le même périmètre ? (QCM : oui, non)

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-perimetres-aires.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-perimeter`, `cm1-area-grid`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 3, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » et fiche « Animations des leçons CM1 — heures, mesures, probabilités et données » (`polygonFigure`, `gridFigure`).
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
