---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYTk
rev: 2026-10-13T16:39:23Z
url: https://github.com/georgesp/Educator/issues/104
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000136 — Leçon CE1 — Tables d'addition
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 780
---

## User story

En tant qu'enfant de CE1, je veux une leçon courte et animée qui m'explique comment retrouver vite les résultats des tables d'addition, avant de m'entraîner.

## Leçon

- **Classe / période** : CE1, période 1.
- **Compétences liées** (`skillIds`) : `ce1-add-tables` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : automatiser les tables dans les deux sens avec les doubles, les amis de 10, le passage par 10 et l'astuce du 9.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « somme », « double », « table d'addition ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Tables d'addition » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Rappel des doubles** — Doubles de 1 à 10 : les rangées de voitures se dédoublent. | `groupObjects` (deux rangées) |
| 3 | **Presque-doubles** — 6 + 7 = 6 + 6 + 1 = 13. | `calcChain` |
| 4 | **Amis de 10** — Les paires qui font 10 s'allument dans la table. | `timesTable` (addition) |
| 5 | **Passer par 10** — 8 + 6 = 8 + 2 + 4 = 14. | `tenFrame` + `calcChain` |
| 6 | **L'astuce du 9** — 9 + 7 = 10 + 7 − 1 = 16. | `calcChain` |
| 7 | **À toi** — Interaction : choisis l'astuce la plus rapide pour 7 + 8. | `calcChain` (QCM) |
| 8 | **Dans les deux sens** — Interaction : 7 + … = 15 puis 13 = 6 + … ; la famille 7, 8, 15 apparaît. | `equation` (famille, interactif) |
| 9 | **Course de fluence** — Défi d'une minute, sans chrono imposé : la voiture avance à chaque réponse. | `numberLine` |
| 10 | **Je retiens** — « Doubles, amis de 10, passage par 10 : je retrouve n'importe quel résultat de la table. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `equation`, `groupObjects`, `numberLine`, `recapCard`, `tenFrame`, `timesTable`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 7 + 8 = ?
2. 9 + 6 = ? (QCM : 14, 15, 16)
3. 8 + … = 13
4. 13 = 6 + …

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [x] Leçon `content/lessons/math/ce1-add-tables.json`, validée par `lesson.schema.json`, `skillIds` = `ce1-add-tables`.
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
