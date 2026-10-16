---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACa90
rev: 2026-10-11T18:51:50Z
url: https://github.com/georgesp/Educator/issues/227
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012215 — Leçon CP — Partager et faire des groupes
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2050
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique comment partager équitablement, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 5.
- **Compétences liées** (`skillIds`) : `cp-share-group` — exercices : la fiche « Exercices CP — Partager et faire des groupes ».
- **Objectif** : distribuer un par un pour partager équitablement, faire des groupes égaux et compter les groupes.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « partager », « équitablement », « autant », « groupe », « équipe ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Partager et faire des groupes » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Un partage juste** — 18 drapeaux pour 3 pilotes : chacun doit en avoir autant. | `groupObjects` (share) |
| 3 | **Un par un** — On distribue un drapeau à chacun, tour après tour : 6 chacun. | `groupObjects` (share, pas à pas) |
| 4 | **À toi** — Interaction : partage 12 casques entre 4 pilotes. | `groupObjects` (share, interactif) |
| 5 | **Faire des équipes** — 24 enfants en équipes de 4 : on forme des groupes de 4. | `groupObjects` (groupBy 4) |
| 6 | **Compter les groupes** — 6 équipes. | `groupObjects` |
| 7 | **Deux questions différentes** — « Combien chacun ? » ou « combien de groupes ? » | `barModel` |
| 8 | **À toi** — Interaction : 15 voitures en garages de 5 : combien de garages ? | `groupObjects` (groupBy, interactif) |
| 9 | **Vérifier** — 6 + 6 + 6 = 18 : chacun a bien autant. | `calcChain` |
| 10 | **Je retiens** — « Partager équitablement, c'est donner autant à chacun. Je distribue un par un jusqu'à la fin. » | `recapCard` |

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

1. 18 images pour 3 enfants : combien chacun ?
2. 24 élèves en équipes de 4 : combien d'équipes ? (QCM : 4, 6, 20)
3. 10 pommes pour 2 : combien chacun ?
4. Est-ce juste : 4 et 5 bonbons ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-partager-grouper.json`, validée par `lesson.schema.json`, `skillIds` = `cp-share-group`.
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
