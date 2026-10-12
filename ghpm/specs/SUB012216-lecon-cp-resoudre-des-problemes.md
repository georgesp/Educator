---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACa-o
rev: 2026-10-11T18:51:53Z
url: https://github.com/georgesp/Educator/issues/228
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012216 — Leçon CP — Résoudre des problèmes
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2070
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique comment comprendre une histoire de nombres, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 2.
- **Compétences liées** (`skillIds`) : `cp-problems-one-step`, `cp-problems-two-steps` — exercices : la fiche « Exercices CP — Résoudre des problèmes ».
- **Objectif** : écouter l'histoire, la jouer avec des objets ou un schéma, choisir ajouter ou enlever, répondre.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « problème », « histoire », « question », « réponse ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Résoudre des problèmes » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Une histoire** — « Dans le bus, il y a 29 enfants… » : la voix raconte, le bus s'anime. | `groupObjects` |
| 3 | **Jouer l'histoire** — 12 enfants descendent : on les retire du bus. | `groupObjects` (remove) |
| 4 | **Encore une étape** — 7 enfants montent : on les ajoute. | `groupObjects` (join) |
| 5 | **La réponse** — Il y a 24 enfants dans le bus. | `calcChain` |
| 6 | **Le schéma** — Une barre pour le tout, des barres pour les parties. | `barModel` (tout / parties) |
| 7 | **À toi** — Interaction : joue l'histoire des 43 cerises. | `groupObjects` (interactif) |
| 8 | **Ajouter ou enlever ?** — « Elle en mange » → on enlève ; « il en gagne » → on ajoute. | `barModel` |
| 9 | **À toi** — Interaction : choisis le bon schéma pour l'histoire. | `barModel` (QCM) |
| 10 | **Je retiens** — « J'écoute l'histoire, je la dessine ou je la joue, je cherche si on ajoute ou on enlève, puis je réponds. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `barModel`, `calcChain`, `groupObjects`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Problème du bus (saisie)
2. Léa a 43 cerises, elle en mange 18 : combien en reste-t-il ?
3. « Il en gagne » : on ajoute ou on enlève ? (QCM)
4. Quel schéma correspond ? (QCM)

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-resoudre-des-problemes.json`, validée par `lesson.schema.json`, `skillIds` = `cp-problems-one-step`, `cp-problems-two-steps`.
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
