---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYb0
rev: 2026-10-12T19:40:08Z
url: https://github.com/georgesp/Educator/issues/108
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000140 — Leçon CP — Additions de deux nombres plus petits que 100
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2090
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique comment additionner deux nombres à deux chiffres, de tête ou en colonnes, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 5.
- **Compétences liées** (`skillIds`) : `cp-add-2-digits` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : additionner en ajoutant les dizaines puis les unités, faire un échange de 10 unités contre 1 dizaine, découvrir l'addition posée.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « dizaine », « unité », « retenue », « addition posée ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Additions de deux nombres plus petits que 100 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Décomposer** — 47 = 40 + 7 et 28 = 20 + 8. | `tensUnits` |
| 3 | **Dizaines puis unités** — 40 + 20 = 60 ; 7 + 8 = 15 ; 60 + 15 = 75. | `calcChain` |
| 4 | **L'échange** — Les 15 cubes : 10 cubes deviennent une barre. | `tensUnits` (échange) |
| 5 | **Autre chemin** — 47 + 20 = 67, puis + 8 = 75. | `numberLine` + `calcChain` |
| 6 | **À toi** — Interaction : 35 + 24, sans échange. | `tensUnits` (interactif) |
| 7 | **À toi** — Interaction : 36 + 27, avec échange. | `tensUnits` (interactif) |
| 8 | **En colonnes** — 47 + 28 posée : unités sous les unités, 15 → on écrit 5 et la retenue 1 en rouge. | `columnOperation` (addition) |
| 9 | **Vérifier** — 47 + 28 ≈ 50 + 30 = 80 : 75 est proche, c'est vraisemblable. | `calcChain` |
| 10 | **Je retiens** — « J'ajoute les dizaines avec les dizaines, les unités avec les unités. 10 unités font 1 dizaine. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `columnOperation`, `numberLine`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 23 + 45 = ?
2. 38 + 27 = ? (QCM : 55, 65, 75)
3. 50 + 36 = ?
4. 19 + 19 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-add-2-digits.json`, validée par `lesson.schema.json`, `skillIds` = `cp-add-2-digits`.
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
