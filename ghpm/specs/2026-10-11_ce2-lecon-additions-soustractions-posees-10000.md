---
type: Issue
title: Leçon CE2 — Additions et soustractions posées jusqu'à 10 000
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-1-ce2.md"
order: 1400
---
## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique comment poser une addition et une soustraction avec des milliers, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 1.
- **Compétences liées** (`skillIds`) : `ce2-add-4-digits`, `ce2-sub-4-digits`, `ce2-money-add-sub` — exercices : la fiche « Exercices CE2 — Additions et soustractions posées jusqu'à 10 000 ».
- **Objectif** : poser et calculer additions et soustractions jusqu'à 10 000, avec retenues et cassages successifs.
- **Durée visée** : 3 à 5 minutes, 11 slides.
- **Vocabulaire du programme** : « termes », « somme », « différence », « retenue », « millier ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Additions et soustractions posées jusqu'à 10 000 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Aligner** — 2 456 + 3 789 : m, c, d, u l'un sous l'autre. | `columnOperation` (alignement) |
| 3 | **Retenues en chaîne** — 6 + 9 = 15, 1 + 5 + 8 = 14, 1 + 4 + 7 = 12, 1 + 2 + 3 = 6 → 6 245. | `columnOperation` (addition) |
| 4 | **Ordre de grandeur** — 2 500 + 3 800 ≈ 6 300 : le résultat est vraisemblable. | `calcChain` |
| 5 | **À toi** — Interaction : 1 235 + 487 + 2 060. | `columnOperation` (interactif) |
| 6 | **Soustraction** — 6 342 − 1 578 : cassages successifs, ou compensations successives ; les deux techniques sont rappelées. | `columnOperation` (soustraction, deux techniques) + `tensUnits` |
| 7 | **Les zéros** — 5 003 − 1 278 : pas de dizaine ni de centaine, on casse un millier. | `columnOperation` + `tensUnits` |
| 8 | **À toi** — Interaction : 7 400 − 2 635. | `columnOperation` (interactif) |
| 9 | **Avec des euros** — 12,35 € + 4,80 € : on aligne les virgules, euros sous euros, centimes sous centimes. | `columnOperation` (prix) |
| 10 | **Vérifier** — La somme de la différence et du petit terme redonne le grand. | `equation` (famille) |
| 11 | **Je retiens** — « J'aligne les chiffres par rang, je commence par les unités, et je contrôle avec un ordre de grandeur. » | `recapCard` |

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

1. 2 456 + 3 789 = ?
2. 5 003 − 1 278 = ? (QCM : 3 725, 3 735, 4 275)
3. Combien de retenues dans 2 456 + 3 789 ?
4. Comment s'appelle le résultat d'une soustraction ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-additions-soustractions-posees-10000.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-add-4-digits`, `ce2-sub-4-digits`, `ce2-money-add-sub`.
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

## Remarques et décisions

- Les deux techniques de soustraction posée (cassage et compensation) sont rappelées, comme au CE1 ; l'enfant garde la sienne.
