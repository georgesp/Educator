---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACa3s
rev: 2026-10-17T13:46:45Z
url: https://github.com/georgesp/Educator/issues/222
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012210 — Leçon CP — Comparer, ranger et placer les nombres jusqu'à 100"
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 1950
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique comment savoir quel nombre est le plus grand, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 3.
- **Compétences liées** (`skillIds`) : `cp-num-compare`, `cp-num-line` — exercices : la fiche « Exercices CP — Comparer, ranger et placer les nombres jusqu'à 100 ».
- **Objectif** : comparer en regardant d'abord les dizaines, utiliser < et >, ranger et placer sur la piste.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « plus grand que », « plus petit que », « égal », « ranger ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Comparer, ranger et placer les nombres jusqu'à 100 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Qui en a le plus ?** — 49 voitures et 53 voitures en barres et cubes. | `tensUnits` |
| 3 | **D'abord les dizaines** — 4 barres contre 5 barres : 53 est plus grand. | `compareNumbers` |
| 4 | **Les signes** — 49 < 53 : la pointe montre le plus petit. | `compareNumbers` |
| 5 | **Mêmes dizaines** — 36 et 34 : même nombre de barres, on regarde les cubes. | `compareNumbers` |
| 6 | **À toi** — Interaction : choisis le signe pour 61 … 16. | `compareNumbers` (interactif) |
| 7 | **Ranger** — Les voitures numérotées vont sur la grille de départ dans l'ordre. | `sequencePattern` (ordre, interactif) |
| 8 | **Sur la piste** — Plus on avance, plus les nombres sont grands. | `numberLine` (0 à 100) |
| 9 | **À toi** — Interaction : place 47 sur la piste. | `numberLine` (interactif) |
| 10 | **Je retiens** — « Je regarde d'abord les dizaines : 49 a 4 dizaines, 53 en a 5, donc 49 < 53. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `compareNumbers`, `numberLine`, `recapCard`, `sequencePattern`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 49 … 53 (QCM : <, >, =)
2. Le plus grand : 38 ou 83 ?
3. Range : 35, 38, 33.
4. Quel nombre vient juste après 69 ?

Chaque question a une explication courte, affichée après la réponse et lisible au clic (rien n'est lu tout seul, décision du 2026-10-15).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [x] Leçon `content/lessons/math/cp-comparer-ranger-100.json`, validée par `lesson.schema.json`, `skillIds` = `cp-num-compare`, `cp-num-line`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [x] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [x] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [x] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
