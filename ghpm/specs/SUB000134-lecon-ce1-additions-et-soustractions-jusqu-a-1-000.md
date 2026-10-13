---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYQ4
rev: 2026-10-11T18:48:45Z
url: https://github.com/georgesp/Educator/issues/102
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000134 — Leçon CE1 — Additions et soustractions jusqu'à 1 000
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 840
---

## User story

En tant qu'enfant de CE1, je veux une leçon courte et animée qui m'explique comment poser une addition et une soustraction avec des nombres à trois chiffres, avant de m'entraîner.

## Leçon

- **Classe / période** : CE1, période 5.
- **Compétences liées** (`skillIds`) : `ce1-add-3-digits`, `ce1-sub-3-digits` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : poser et calculer des additions et soustractions de nombres à trois chiffres, avec retenues.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « centaine », « retenue », « somme », « différence ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Additions et soustractions jusqu'à 1 000 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Bien aligner** — 245 + 437 : chaque chiffre dans sa colonne (c, d, u). | `columnOperation` (alignement) |
| 3 | **Addition** — 5 + 7 = 12 → 2, retenue 1 ; 1 + 4 + 3 = 8 ; 2 + 4 = 6 → 682. | `columnOperation` (addition) |
| 4 | **Deux retenues** — 378 + 456 : retenues aux dizaines et aux centaines. | `columnOperation` |
| 5 | **À toi** — Interaction : 263 + 519. | `columnOperation` (interactif) |
| 6 | **Soustraction** — 652 − 318 : 2 − 8 impossible ; cassage (on casse une dizaine) ou compensation (+ 10 en haut, + 1 dizaine en bas), selon la technique choisie. | `columnOperation` (soustraction, method choisie) + `tensUnits` |
| 7 | **Le zéro au milieu** — 503 − 128 : avec le cassage, on casse une centaine ; avec la compensation, on compense deux fois. | `columnOperation` (deux techniques) |
| 8 | **À toi** — Interaction : 741 − 365. | `columnOperation` (interactif) |
| 9 | **Vérifier** — Ordre de grandeur (650 − 300 ≈ 350) et preuve par l'addition. | `calcChain` + `equation` |
| 10 | **Je retiens** — « J'aligne unités, dizaines, centaines. Je commence par les unités et je n'oublie pas les retenues. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `columnOperation`, `equation`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 245 + 437 = ?
2. 652 − 318 = ? (QCM : 334, 344, 346)
3. Combien de retenues dans 378 + 456 ?
4. 500 − 125 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [x] Leçon `content/lessons/math/ce1-add-sub-3-digits.json`, validée par `lesson.schema.json`, `skillIds` = `ce1-add-3-digits`, `ce1-sub-3-digits`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [x] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [x] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [x] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [x] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.

## Remarques et décisions

- Les deux compétences de période 5 (`ce1-add-3-digits` et `ce1-sub-3-digits`) partagent une leçon : la technique posée est la même idée. À scinder en deux leçons si la durée dépasse 5 minutes.
