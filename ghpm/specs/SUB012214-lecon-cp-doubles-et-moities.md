---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACa80
rev: 2026-10-17T13:46:48Z
url: https://github.com/georgesp/Educator/issues/226
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012214 — Leçon CP — Doubles et moitiés
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 1990
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique ce que sont un double et une moitié, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 3.
- **Compétences liées** (`skillIds`) : `cp-doubles-halves`, `cp-half-even` — exercices : la fiche « Exercices CP — Doubles et moitiés ».
- **Objectif** : relier double et moitié avec des collections identiques et des partages en deux.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « double », « moitié », « partager ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Doubles et moitiés » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Le miroir** — 5 voitures et leur reflet : le double de 5 est 10. | `groupObjects` (deux rangées) |
| 3 | **Doubles à connaître** — Les doubles de 1 à 10 s'allument dans la table. | `timesTable` (addition, diagonale) |
| 4 | **À toi** — Interaction : fabrique le double de 6. | `groupObjects` (interactif) |
| 5 | **La moitié** — 14 casques pour 2 pilotes, autant chacun : 7. | `groupObjects` (share) |
| 6 | **Double ↔ moitié** — 7 + 7 = 14, donc la moitié de 14 est 7. | `equation` (famille) |
| 7 | **Doubles des dizaines** — Double de 30 = 60 : 3 barres + 3 barres. | `tensUnits` |
| 8 | **Moitié de 46** — 4 barres et 6 cubes en deux : 2 barres et 3 cubes = 23. | `tensUnits` (share) |
| 9 | **À toi** — Interaction : partage 18 drapeaux en deux. | `groupObjects` (share, interactif) |
| 10 | **Je retiens** — « Le double de 7, c'est 7 + 7 = 14. La moitié de 14, c'est 7 : je partage en deux parts égales. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `equation`, `groupObjects`, `recapCard`, `tensUnits`, `timesTable`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Double de 7 ?
2. Moitié de 16 ? (QCM : 6, 8, 32)
3. Double de 30 ?
4. Moitié de 46 ?

Chaque question a une explication courte, affichée après la réponse et lisible au clic (rien n'est lu tout seul, décision du 2026-10-15).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [x] Leçon `content/lessons/math/cp-doubles-et-moities.json`, validée par `lesson.schema.json`, `skillIds` = `cp-doubles-halves`, `cp-half-even`.
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
