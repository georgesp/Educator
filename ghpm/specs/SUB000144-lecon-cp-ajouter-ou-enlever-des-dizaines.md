---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYiw
url: https://github.com/georgesp/Educator/issues/112
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000144 — Leçon CP — Ajouter ou enlever des dizaines
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 1050
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique pourquoi ajouter ou enlever des dizaines est facile, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 3.
- **Compétences liées** (`skillIds`) : `cp-add-sub-tens` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : comprendre qu'ajouter ou enlever des dizaines ne change que le chiffre des dizaines.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « dizaine », « unité », « chiffre des dizaines ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Ajouter ou enlever des dizaines » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Une dizaine** — 10 cubes s'assemblent en une barre : 1 dizaine = 10 unités. | `tensUnits` |
| 3 | **Lire 37** — 37, c'est 3 barres et 7 cubes : 3 dizaines et 7 unités. | `tensUnits` |
| 4 | **+ 10** — 37 + 10 : une barre de plus, 47. Le 7 des unités ne bouge pas. | `tensUnits` + `equation` |
| 5 | **− 10** — 37 − 10 : une barre de moins, 27. | `tensUnits` |
| 6 | **Bonds de 10** — La voiture part de 6 et fait des bonds de 10 : 16, 26, 36… | `numberLine` (pas de 10) |
| 7 | **+ 30** — 46 + 30 : trois barres de plus, 76. | `tensUnits` |
| 8 | **À toi** — Interaction : ajouter des barres à 25 pour arriver à 65. | `tensUnits` (interactif) |
| 9 | **À toi** — Interaction : enlever 4 barres à 76. | `tensUnits` (interactif) |
| 10 | **Je retiens** — « Quand j'ajoute ou j'enlève des dizaines, seul le chiffre des dizaines change. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `equation`, `numberLine`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 53 + 10 = ?
2. 48 − 10 = ?
3. 76 − 30 = ? (QCM : 36, 46, 73)
4. 24 + … = 54

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-add-sub-tens.json`, validée par `lesson.schema.json`, `skillIds` = `cp-add-sub-tens`.
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
