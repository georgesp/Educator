---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZgI
url: https://github.com/georgesp/Educator/issues/151
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011929 — Leçon CE2 — Résoudre des problèmes
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1620
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique une méthode pour résoudre un problème, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 1.
- **Compétences liées** (`skillIds`) : `ce2-problems-additive`, `ce2-problems-multiplicative`, `ce2-problems-mixed` — exercices : la fiche « Exercices CE2 — Résoudre des problèmes ».
- **Objectif** : suivre quatre étapes (comprendre, schématiser, calculer, répondre) et utiliser le schéma en barres.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « problème », « donnée », « question », « schéma », « réponse ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Résoudre des problèmes » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Une histoire** — « 125 casques au stand, 38 arrivent. Combien en tout ? » : la voix lit, les mots importants s'allument. | `barModel` (vide) |
| 3 | **Le tout et les parties** — Deux barres pour les parties, une grande barre pour le tout : on cherche le tout → addition. | `barModel` (tout / parties) |
| 4 | **Chercher une partie** — On connaît le tout et une partie : on cherche l'autre partie → soustraction. | `barModel` |
| 5 | **Comparer** — Une barre plus longue de 25 : « 25 de plus que ». | `barModel` (comparaison) |
| 6 | **Fois plus** — 3 fois plus : la petite barre est reportée 3 fois. | `barModel` (fois plus) |
| 7 | **À toi** — Interaction : place les nombres de l'histoire sur le schéma. | `barModel` (interactif) |
| 8 | **Deux étapes** — Interaction : 125 billes, + 38, − 20 : l'enfant enchaîne les deux calculs. | `barModel` + `calcChain` (interactif) |
| 9 | **Répondre** — Une phrase réponse et une vérification : est-ce que c'est possible ? | `equation` |
| 10 | **Je retiens** — « Je lis et je raconte l'histoire, je fais un schéma, je calcule, puis je réponds par une phrase et je vérifie. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `barModel`, `calcChain`, `equation`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Problème parties-tout lu par la voix (saisie)
2. Quelle opération ? « 3 fois plus que 45 » (QCM : +, −, ×)
3. Problème en deux étapes (saisie)
4. Quel schéma correspond à l'histoire ? (QCM de 3 schémas)

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-resoudre-des-problemes.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-problems-additive`, `ce2-problems-multiplicative`, `ce2-problems-mixed`.
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
