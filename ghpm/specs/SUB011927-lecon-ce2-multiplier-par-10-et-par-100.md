---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZdo
url: https://github.com/georgesp/Educator/issues/149
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011927 — Leçon CE2 — Multiplier par 10 et par 100
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1480
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique pourquoi multiplier par 10 ou par 100 décale les chiffres, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 2.
- **Compétences liées** (`skillIds`) : `ce2-mul-10-100` — exercices : la fiche « Exercices CE2 — Multiplier par 10 et par 100 ».
- **Objectif** : comprendre que × 10 transforme chaque unité en dizaine (chaque chiffre avance d'un rang), × 100 de deux rangs.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « dizaine », « centaine », « rang », « produit ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Multiplier par 10 et par 100 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **10 fois 3** — 10 fois 3 cubes : 3 dizaines, 30. | `tensUnits` |
| 3 | **10 fois 24** — 24 = 2 d + 4 u ; × 10 → 2 c + 4 d = 240. | `tensUnits` + `placeValueTable` |
| 4 | **Les chiffres avancent** — Dans le tableau, 7, 2, 4 glissent d'une colonne vers la gauche : 7 240. | `placeValueTable` (shift) |
| 5 | **Le zéro** — Le 0 garde la place des unités vides. | `placeValueTable` |
| 6 | **× 100** — 36 × 100 : les chiffres avancent de deux rangs, 3 600. | `placeValueTable` (shift 2) |
| 7 | **À toi** — Interaction : fais glisser les chiffres de 58 pour × 10. | `placeValueTable` (interactif) |
| 8 | **Facteur manquant** — … × 10 = 450 : on fait reculer les chiffres. | `placeValueTable` |
| 9 | **À toi** — Interaction : 10 × 307. | `placeValueTable` (interactif) |
| 10 | **Je retiens** — « Quand je multiplie par 10, chaque chiffre prend une valeur 10 fois plus grande : il avance d'un rang. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `placeValueTable`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 10 × 724 = ?
2. 36 × 100 = ? (QCM : 360, 3 600, 36 000)
3. … × 10 = 450
4. Vrai ou faux : 10 × 307 = 3 070

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-multiplier-par-10-et-100.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-mul-10-100`.
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
