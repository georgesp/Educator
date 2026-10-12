---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZaA
url: https://github.com/georgesp/Educator/issues/147
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011925 — Leçon CE2 — Multiplication posée
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1540
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique comment poser une multiplication, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 4.
- **Compétences liées** (`skillIds`) : `ce2-mul-column-1-digit`, `ce2-mul-column-2-digits` — exercices : la fiche « Exercices CE2 — Multiplication posée ».
- **Objectif** : poser une multiplication par un nombre à un chiffre (retenues), puis à deux chiffres (deux lignes et décalage).
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « produit », « facteur », « retenue », « multiplication posée ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Multiplication posée » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Pourquoi poser** — 247 × 3 : 247 + 247 + 247, c'est long ; on pose. | `columnOperation` (multiplication) |
| 3 | **Unités** — 3 × 7 = 21 : j'écris 1, retenue 2 en rouge. | `columnOperation` |
| 4 | **Dizaines** — 3 × 4 = 12, + 2 = 14 : j'écris 4, retenue 1. | `columnOperation` |
| 5 | **Centaines** — 3 × 2 = 6, + 1 = 7 → 741. | `columnOperation` |
| 6 | **À toi** — Interaction : 156 × 4. | `columnOperation` (interactif) |
| 7 | **Deux chiffres** — 38 × 24 = 38 × 4 + 38 × 20 : deux lignes. | `columnOperation` + `arrayGrid` (split) |
| 8 | **Le décalage** — La ligne des dizaines commence sous les dizaines (on écrit un 0) ; on additionne : 912. | `columnOperation` |
| 9 | **À toi** — Interaction : 38 × 24, l'enfant remplit la 2e ligne puis compare à l'ordre de grandeur (40 × 25 = 1 000). | `columnOperation` (interactif) + `calcChain` |
| 10 | **Je retiens** — « Je multiplie chaque chiffre du haut en commençant par les unités et j'ajoute la retenue après avoir multiplié. Pour les dizaines, je décale d'un rang. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `calcChain`, `columnOperation`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 247 × 3 = ?
2. 156 × 4 = ? (QCM : 604, 624, 644)
3. 38 × 24 = ?
4. Pourquoi écrit-on un 0 au début de la 2e ligne ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-multiplication-posee.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-mul-column-1-digit`, `ce2-mul-column-2-digits`.
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
