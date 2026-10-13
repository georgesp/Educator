---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYJc
rev: 2026-10-13T18:57:34Z
url: https://github.com/georgesp/Educator/issues/98
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000130 — Leçon CE1 — Les premières fractions
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 870
---

## User story

En tant qu'enfant de CE1, je veux une leçon courte et animée qui m'explique ce qu'est une fraction, avant de m'entraîner.

## Leçon

- **Classe / période** : CE1, période 2.
- **Compétences liées** (`skillIds`) : `ce1-frac-unit`, `ce1-frac-read-write`, `ce1-frac-compare-add` — exercices : la fiche « Exercices CE1 — Les premières fractions ».
- **Objectif** : partager une unité en parts égales, nommer et écrire 1/2, 1/3, 1/4…, puis 3/4 ; comparer et ajouter des fractions simples.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « fraction », « demi », « tiers », « quart », « numérateur », « dénominateur ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Les premières fractions » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Partager juste** — Une bande de piste partagée en 2 parts égales : chaque part est un demi, 1/2. | `fractionBar` (2) |
| 3 | **Pas égales ?** — Si les parts ne sont pas égales, ce ne sont pas des demis. | `fractionBar` |
| 4 | **Tiers, quarts…** — En 3 : des tiers ; en 4 : des quarts ; puis cinquièmes, sixièmes, huitièmes, dixièmes. | `fractionBar` |
| 5 | **À toi** — Interaction : colorie un quart de la bande. | `fractionBar` (interactif) |
| 6 | **Plusieurs parts** — 3 quarts : 3/4. En bas le dénominateur (4), en haut le numérateur (3). | `fractionBar` + `equation` |
| 7 | **Plus de parts, plus petites** — 1/3 et 1/5 empilés : un cinquième est plus petit qu'un tiers. | `fractionBar` (stack) |
| 8 | **Ajouter** — 2/8 + 3/8 = 5/8 : on compte des huitièmes, le dénominateur ne change pas. | `fractionBar` |
| 9 | **À toi** — Interaction : 1/4 + 2/4 avec la bande. | `fractionBar` (interactif) |
| 10 | **Je retiens** — « Le dénominateur dit en combien de parts égales on partage ; le numérateur dit combien de parts on prend. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `equation`, `fractionBar`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Quelle fraction est coloriée ? (bande de 4, 1 part)
2. Écris en chiffres : trois quarts.
3. 1/3 … 1/5 (QCM : <, >, =)
4. 2/8 + 3/8 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [x] Leçon `content/lessons/math/ce1-premieres-fractions.json`, validée par `lesson.schema.json`, `skillIds` = `ce1-frac-unit`, `ce1-frac-read-write`, `ce1-frac-compare-add`.
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
