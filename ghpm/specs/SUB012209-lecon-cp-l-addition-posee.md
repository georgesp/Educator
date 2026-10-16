---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACa2o
rev: 2026-10-11T18:51:44Z
url: https://github.com/georgesp/Educator/issues/221
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012209 — Leçon CP — L'addition posée
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2010
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique comment poser une addition, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 5.
- **Compétences liées** (`skillIds`) : `cp-add-column` — exercices : la fiche « Exercices CP — L'addition posée ».
- **Objectif** : aligner unités et dizaines, additionner les unités, écrire la retenue, additionner les dizaines.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « addition posée », « retenue », « dizaine », « unité ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « L'addition posée » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Ranger les chiffres** — 45 et 37 l'un sous l'autre, chaque chiffre dans sa colonne. | `columnOperation` (alignement) |
| 3 | **Les unités** — 5 + 7 = 12 : 1 dizaine et 2 unités. | `tensUnits` (échange) |
| 4 | **La retenue** — On écrit 2 en bas, le 1 de la dizaine monte en rouge. | `columnOperation` |
| 5 | **Les dizaines** — 1 + 4 + 3 = 8 → 82. | `columnOperation` |
| 6 | **À toi** — Interaction : 36 + 25. | `columnOperation` (interactif) |
| 7 | **Un petit nombre** — 28 + 8 : le 8 va sous les unités ! | `columnOperation` (alignement) |
| 8 | **Trois nombres** — 28 + 8 + 56. | `columnOperation` |
| 9 | **À toi** — Interaction : 47 + 39. | `columnOperation` (interactif) |
| 10 | **Je retiens** — « Unités sous les unités, dizaines sous les dizaines. Si les unités font 10 ou plus, j'écris la retenue en haut des dizaines. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `columnOperation`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 45 + 37 = ?
2. Où écrit-on le 8 de 28 + 8 ? (QCM : sous le 2, sous le 8)
3. 36 + 25 = ?
4. 28 + 8 + 56 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-addition-posee.json`, validée par `lesson.schema.json`, `skillIds` = `cp-add-column`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
