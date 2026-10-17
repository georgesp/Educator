---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZZI
rev: 2026-10-11T18:49:51Z
url: https://github.com/georgesp/Educator/issues/146
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011924 — Leçon CE2 — Les fractions : partager l'unité"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1200
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique ce qu'est une fraction, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 3.
- **Compétences liées** (`skillIds`) : `ce2-frac-read-write`, `ce2-frac-equal`, `ce2-frac-number-line` — exercices : la fiche « Exercices CE2 — Les fractions : partager l'unité ».
- **Objectif** : comprendre une fraction comme un nombre de parts égales d'une unité, la lire, l'écrire et reconnaître des fractions égales.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « fraction », « numérateur », « dénominateur », « demi », « tiers », « quart », « dixième ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Les fractions : partager l'unité » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Parts égales** — Une bande de piste partagée en 4 parts égales : chaque part est un quart. | `fractionBar` (4) |
| 3 | **Pas égales ?** — Si les parts ne sont pas égales, ce ne sont pas des quarts. | `fractionBar` |
| 4 | **Écrire** — 3 parts sur 4 : 3/4 ; le 4 en bas (dénominateur), le 3 en haut (numérateur). | `fractionBar` + `equation` |
| 5 | **Lire** — 1/2 un demi, 1/3 un tiers, 1/4 un quart, puis cinquièmes, sixièmes… dixièmes. | `fractionBar` |
| 6 | **À toi** — Interaction : colorie 5/8 de la bande. | `fractionBar` (interactif) |
| 7 | **Fractions égales** — Bandes empilées : 1/2 et 5/10 couvrent la même longueur. | `fractionBar` (stack) |
| 8 | **Sur la piste** — La piste de 0 à 1 graduée en dixièmes : la voiture s'arrête à 3/10. | `numberLine` (fractions) |
| 9 | **À toi** — Interaction : trouve …/8 égal à 3/4 avec les bandes. | `fractionBar` (stack, interactif) |
| 10 | **Je retiens** — « 3/4 : l'unité est partagée en 4 parts égales et j'en prends 3. Le dénominateur dit en combien de parts, le numérateur combien j'en prends. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `equation`, `fractionBar`, `numberLine`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Quelle fraction est coloriée ? (bande de 8, 3 parts)
2. Écris en chiffres : cinq sixièmes.
3. 1/2 = …/10
4. Dans 3/4, comment s'appelle le 4 ? (QCM : numérateur, dénominateur)
5. Sur la piste graduée en dixièmes, où s'arrête la voiture ? → 3/10

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-fractions-partager-l-unite.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-frac-read-write`, `ce2-frac-equal`, `ce2-frac-number-line`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `numberLine` graduée en fractions ; quiz à choix en mots.
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
