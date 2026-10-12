---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYpc
rev: 2026-10-12T19:40:24Z
url: https://github.com/georgesp/Educator/issues/116
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000148 — Leçon CP — Enlever un petit nombre à une dizaine
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2170
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique comment enlever un petit nombre à 50, 60 ou 80, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 5.
- **Compétences liées** (`skillIds`) : `cp-sub-from-tens` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : soustraire un nombre inférieur à 10 à un nombre entier de dizaines (50 − 6) avec les compléments à 10.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « dizaine », « casser une dizaine », « complément ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Enlever un petit nombre à une dizaine » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Pas de cubes** — 50 = 5 barres, 0 cube : pour enlever 6, il faut casser une barre. | `tensUnits` (casser) |
| 3 | **Ce qui reste** — Il reste 4 barres et 4 cubes : 44. | `tensUnits` |
| 4 | **L'astuce** — 10 − 6 = 4 (amis de 10), donc 50 − 6 = 44. | `calcChain` |
| 5 | **Sur la piste** — On recule de 50 à 44 : on passe sous le drapeau des 50. | `numberLine` (arrière) |
| 6 | **À toi** — Interaction : 30 − 7 avec les barres. | `tensUnits` (interactif) |
| 7 | **Vérifier** — 44 + 6 = 50 : on retrouve la dizaine. | `equation` (famille) |
| 8 | **À toi** — Interaction : 80 − … = 75. | `equation` (interactif) |
| 9 | **Course éclair** — 40 − 3, 90 − 8, 20 − 9. | `calcChain` |
| 10 | **Je retiens** — « 50 − 6 : 10 − 6 = 4, donc 50 − 6 = 44. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `equation`, `numberLine`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 40 − 3 = ?
2. 90 − 8 = ? (QCM : 81, 82, 88)
3. 60 − … = 52
4. 20 − 9 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-sub-from-tens.json`, validée par `lesson.schema.json`, `skillIds` = `cp-sub-from-tens`.
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
