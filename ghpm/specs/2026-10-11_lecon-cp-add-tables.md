---
type: Issue
title: Leçon CP — Tables d'addition
status: Todo
size: M
parent: "#1"
order: 1040
---
## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique des astuces pour retenir les tables d'addition dans les deux sens, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 2.
- **Compétences liées** (`skillIds`) : `cp-add-tables` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : mémoriser les tables à l'aide des doubles, des presque-doubles, des amis de 10 et du passage par 10.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « double », « dizaine », « table d'addition ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Tables d'addition » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Les doubles** — 4 + 4 : deux rangées identiques de voitures, 8. | `groupObjects` (deux rangées) |
| 3 | **Presque-doubles** — 4 + 5, c'est 4 + 4 + 1 = 9. | `calcChain` |
| 4 | **Les amis de 10** — 1 + 9, 2 + 8, 3 + 7, 4 + 6, 5 + 5 remplissent le garage. | `tenFrame` |
| 5 | **10 + un nombre** — 10 + 6 = 16 : une barre de dix et 6 cubes. | `tensUnits` |
| 6 | **Passer par 10** — 8 + 5 : je remplis le garage avec 2, il reste 3 → 13. | `tenFrame` (deux garages) + `calcChain` |
| 7 | **À toi** — Interaction : 9 + 4 avec les deux garages. | `tenFrame` (interactif) |
| 8 | **Dans les deux sens** — Interaction : 6 + … = 10 puis 7 + … = 12, l'enfant remplit la case vide. | `equation` (case à trou, interactif) |
| 9 | **Ma table** — Le tableau des tables s'allume : les doubles sur la diagonale, les amis de 10 en rouge. | `timesTable` (addition) |
| 10 | **Je retiens** — « Pour retenir mes tables : les doubles, les amis de 10, et je passe par 10. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `equation`, `groupObjects`, `recapCard`, `tenFrame`, `tensUnits`, `timesTable`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 6 + 6 = ?
2. 7 + 8 = ? (QCM : 14, 15, 16)
3. 5 + … = 12
4. 9 + 3 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-add-tables.json`, validée par `lesson.schema.json`, `skillIds` = `cp-add-tables`.
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
