---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYC4
url: https://github.com/georgesp/Educator/issues/94
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000126 — Leçon CE1 — Le signe × et les tables de multiplication
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 1235
---

## User story

En tant qu'enfant de CE1, je veux une leçon courte et animée qui m'explique ce que veut dire le signe ×, avant de m'entraîner.

## Leçon

- **Classe / période** : CE1, période 2.
- **Compétences liées** (`skillIds`) : `ce1-mul-meaning`, `ce1-mul-tables-2-5-10`, `ce1-mul-tables-3-4`, `ce1-mul-tables-6-9` — exercices : la fiche « Exercices CE1 — Le signe × et les tables de multiplication ».
- **Objectif** : passer de l'addition répétée au signe ×, voir que 3 × 5 = 5 × 3, retenir les premières tables.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « fois », « multiplication », « table », « produit (découverte) ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Le signe × et les tables de multiplication » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Des rangées** — La grille de départ : 3 rangées de 5 voitures. | `arrayGrid` (3 × 5) |
| 3 | **Addition répétée** — 5 + 5 + 5 = 15 : on l'écrit 3 × 5 = 15, « 3 fois 5 ». | `arrayGrid` + `equation` |
| 4 | **À toi** — Interaction : écris 4 + 4 + 4 + 4 avec le signe ×. | `equation` (interactif) |
| 5 | **Tourner la grille** — 5 rangées de 3 : toujours 15. 3 × 5 = 5 × 3. | `arrayGrid` (rotation) |
| 6 | **Table de 2** — Les doubles : 2 × 6 = 6 + 6 = 12. | `timesTable` (multiplication) |
| 7 | **Tables de 5 et de 10** — × 10 : les produits finissent par 0 ; × 5 : par 0 ou 5. | `timesTable` + `sequencePattern` |
| 8 | **À toi** — Interaction : construis 4 × 3 en plaçant des rangées. | `arrayGrid` (interactif) |
| 9 | **Facteur manquant** — … × 5 = 40 : combien de rangées de 5 pour 40 ? | `arrayGrid` + `equation` |
| 10 | **Je retiens** — « 3 × 5, c'est 3 fois 5 : 5 + 5 + 5 = 15. On peut changer l'ordre : 3 × 5 = 5 × 3. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `equation`, `recapCard`, `sequencePattern`, `timesTable`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 5 + 5 + 5 = … × 5
2. 2 × 7 = ? (QCM : 9, 14, 27)
3. … × 10 = 60
4. Vrai ou faux : 3 × 4 = 4 × 3

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce1-multiplication-et-tables.json`, validée par `lesson.schema.json`, `skillIds` = `ce1-mul-meaning`, `ce1-mul-tables-2-5-10`, `ce1-mul-tables-3-4`, `ce1-mul-tables-6-9`.
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
