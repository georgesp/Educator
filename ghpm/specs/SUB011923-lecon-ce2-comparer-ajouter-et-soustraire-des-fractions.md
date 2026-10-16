---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZXQ
rev: 2026-10-11T18:49:54Z
url: https://github.com/georgesp/Educator/issues/145
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011923 — Leçon CE2 — Comparer, ajouter et soustraire des fractions"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1220
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique comment comparer et ajouter des fractions, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 4.
- **Compétences liées** (`skillIds`) : `ce2-frac-compare`, `ce2-frac-add-sub` — exercices : la fiche « Exercices CE2 — Comparer, ajouter et soustraire des fractions ».
- **Objectif** : comparer des fractions avec les bandes, ajouter et soustraire des fractions de même dénominateur.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « fraction », « numérateur », « dénominateur », « comparer ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Comparer, ajouter et soustraire des fractions » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Même dénominateur** — 3/8 et 5/8 : des huitièmes, 5 parts c'est plus que 3. | `fractionBar` (stack) |
| 3 | **Même numérateur** — 2/3 et 2/5 : un tiers est plus grand qu'un cinquième, donc 2/3 > 2/5. | `fractionBar` (stack) |
| 4 | **Dénominateurs multiples** — 5/6 = 10/12, donc 7/12 < 5/6. | `fractionBar` (stack) |
| 5 | **À toi** — Interaction : choisis le signe pour 4/10 … 1/2. | `compareNumbers` + `fractionBar` (interactif) |
| 6 | **Ajouter** — Le gâteau : 1/10 + 3/10 + 2/10 = 6/10 mangés. | `fractionBar` + `equation` |
| 7 | **Pas les dénominateurs !** — 1/10 + 3/10 ≠ 4/20 : les parts restent des dixièmes. | `fractionBar` |
| 8 | **Soustraire** — 1 − 3/4 : il reste 1/4 de la bande. | `fractionBar` |
| 9 | **À toi** — Interaction : 1/2 + 1/4 avec des quarts. | `fractionBar` (interactif) |
| 10 | **Je retiens** — « Même dénominateur : je compare ou j'ajoute les numérateurs, le dénominateur ne change pas. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `compareNumbers`, `equation`, `fractionBar`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 3/8 … 5/8 (QCM : <, >, =)
2. 1/10 + 3/10 = ?
3. Le plus grand : 2/3 ou 2/5 ?
4. 1 − 3/4 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-fractions-comparer-ajouter.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-frac-compare`, `ce2-frac-add-sub`.
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
