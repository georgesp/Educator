---
type: Issue
title: Leçon CE1 — Additions de nombres plus petits que 100
status: Todo
size: M
parent: "#1"
order: 1130
---
## User story

En tant qu'enfant de CE1, je veux une leçon courte et animée qui m'explique comment additionner des nombres à deux chiffres, en ligne et en colonnes, avant de m'entraîner.

## Leçon

- **Classe / période** : CE1, période 1.
- **Compétences liées** (`skillIds`) : `ce1-add-2-digits` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : additionner en décomposant, puis poser l'addition en colonnes avec retenue.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « somme », « retenue », « dizaine », « unité », « addition posée ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Additions de nombres plus petits que 100 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Décomposer** — 56 + 27 = 50 + 20 + 6 + 7 = 70 + 13 = 83. | `calcChain` |
| 3 | **Barres et cubes** — 13 cubes : on échange 10 cubes contre une barre. | `tensUnits` (échange) |
| 4 | **Poser** — On écrit 56 au-dessus de 27, unités sous unités, dizaines sous dizaines. | `columnOperation` (alignement) |
| 5 | **Les unités** — 6 + 7 = 13 : on écrit 3, la retenue 1 monte en rouge dans la colonne des dizaines. | `columnOperation` |
| 6 | **Les dizaines** — 1 + 5 + 2 = 8 : résultat 83. | `columnOperation` |
| 7 | **À toi** — Interaction : 34 + 48 posée, l'enfant écrit chiffre par chiffre. | `columnOperation` (interactif) |
| 8 | **Ordre de grandeur** — 56 + 27, c'est environ 60 + 30 = 90 : 83 est vraisemblable. | `calcChain` |
| 9 | **Trois nombres** — Interaction : 28 + 8 + 56 en colonnes, le 8 bien placé sous les unités. | `columnOperation` (interactif) |
| 10 | **Je retiens** — « En colonnes, j'aligne les unités. Si les unités dépassent 9, j'écris la retenue au-dessus des dizaines. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `columnOperation`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 45 + 37 = ?
2. 29 + 46 = ? (QCM : 65, 75, 615)
3. Quelle est la retenue dans 38 + 25 ?
4. 62 + 19 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce1-add-2-digits.json`, validée par `lesson.schema.json`, `skillIds` = `ce1-add-2-digits`.
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
