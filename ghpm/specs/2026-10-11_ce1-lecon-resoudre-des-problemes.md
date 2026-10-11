---
type: Issue
title: Leçon CE1 — Résoudre des problèmes
status: Todo
size: M
parent: "#1"
order: 1265
---
## User story

En tant qu'enfant de CE1, je veux une leçon courte et animée qui m'explique une méthode pour résoudre un problème, avant de m'entraîner.

## Leçon

- **Classe / période** : CE1, période 1.
- **Compétences liées** (`skillIds`) : `ce1-problems-additive`, `ce1-problems-two-steps`, `ce1-problems-multiplicative` — exercices : la fiche « Exercices CE1 — Résoudre des problèmes ».
- **Objectif** : comprendre l'histoire, la représenter par un schéma en barres, choisir l'opération, répondre par une phrase.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « problème », « question », « schéma », « de plus », « de moins », « en tout ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Résoudre des problèmes » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Une histoire** — « Tom a 35 billes, Léa en a 12 de plus. » La voix lit, les nombres s'allument. | `barModel` (vide) |
| 3 | **Comparer** — La barre de Léa dépasse celle de Tom de 12 : 35 + 12 = 47. | `barModel` (comparaison) |
| 4 | **Le tout et les parties** — Quand on cherche le tout, on ajoute ; quand on cherche une partie, on soustrait. | `barModel` (tout / parties) |
| 5 | **À toi** — Interaction : place les nombres de l'histoire sur le schéma. | `barModel` (interactif) |
| 6 | **Des paquets égaux** — 4 boîtes de 6 pneus : 4 × 6 = 24. | `groupObjects` (groupBy) + `barModel` |
| 7 | **Partager** — 24 pneus pour 4 voitures : 6 chacune. | `groupObjects` (share) |
| 8 | **Deux étapes** — Interaction : 3 rangées de 8 voitures, 5 partent : l'enfant enchaîne × puis −. | `arrayGrid` + `calcChain` (interactif) |
| 9 | **Répondre** — « Il reste 19 voitures. » La réponse est-elle possible ? | `equation` |
| 10 | **Je retiens** — « Je lis l'histoire, je fais un schéma, je choisis l'opération, je calcule, je réponds par une phrase et je vérifie. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `barModel`, `calcChain`, `equation`, `groupObjects`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Problème « de plus » (saisie)
2. Quelle opération pour 4 boîtes de 6 ? (QCM : +, −, ×)
3. Problème en deux étapes (saisie)
4. Quel schéma correspond à l'histoire ? (QCM)

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce1-resoudre-des-problemes.json`, validée par `lesson.schema.json`, `skillIds` = `ce1-problems-additive`, `ce1-problems-two-steps`, `ce1-problems-multiplicative`.
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
