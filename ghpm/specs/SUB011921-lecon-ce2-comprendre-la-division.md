---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZUU
rev: 2026-10-11T18:49:48Z
url: https://github.com/georgesp/Educator/issues/143
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011921 — Leçon CE2 — Comprendre la division
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1180
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique ce que veut dire diviser, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 4.
- **Compétences liées** (`skillIds`) : `ce2-div-tables`, `ce2-div-inverse` — exercices : la fiche « Exercices CE2 — Comprendre la division ».
- **Objectif** : relier la division au partage équitable et au groupement, écrire avec ÷ et retrouver le résultat avec les tables.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « partager », « diviser », « divisé par », « quotient (découverte) ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Comprendre la division » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Partager** — 24 casques pour 4 équipes : chacun reçoit son tour, 6 chacune. | `groupObjects` (share) |
| 3 | **Grouper** — 24 voitures en rangées de 4 : 6 rangées. | `groupObjects` (groupBy) + `arrayGrid` |
| 4 | **Le signe ÷** — 24 ÷ 4 = 6 : « 24 divisé par 4 égale 6 ». | `equation` |
| 5 | **Lien avec ×** — 4 × 6 = 24, donc 24 ÷ 4 = 6 et 24 ÷ 6 = 4 : la famille. | `equation` (famille ×/÷) |
| 6 | **À toi** — Interaction : partage 18 pneus entre 3 voitures. | `groupObjects` (share, interactif) |
| 7 | **Avec les tables** — 42 ÷ 6 : dans la table de 6, qui donne 42 ? 7. | `timesTable` |
| 8 | **Grands nombres** — 7 × 13 = 91, donc 91 ÷ 7 = 13. | `equation` (famille) |
| 9 | **À toi** — Interaction : 35 ÷ 5 avec la grille. | `arrayGrid` (interactif) |
| 10 | **Je retiens** — « 42 ÷ 6 = 7 car 6 × 7 = 42 : diviser, c'est chercher le facteur manquant. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `equation`, `groupObjects`, `recapCard`, `timesTable`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 42 ÷ 6 = ?
2. 24 élèves en équipes de 4 : combien d'équipes ? (QCM : 6, 20, 28)
3. 7 × 13 = 91, donc 91 ÷ 7 = ?
4. Vrai ou faux : 30 ÷ 5 = 6

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-comprendre-la-division.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-div-tables`, `ce2-div-inverse`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `equation` avec la division (« ÷ ») et la famille × / ÷.
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
