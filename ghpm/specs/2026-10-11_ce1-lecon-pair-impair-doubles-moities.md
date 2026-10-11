---
type: Issue
title: Leçon CE1 — Pair, impair, doubles et moitiés
status: Todo
size: M
parent: "#1"
order: 1245
---
## User story

En tant qu'enfant de CE1, je veux une leçon courte et animée qui m'explique ce que sont les nombres pairs, les doubles et les moitiés, avant de m'entraîner.

## Leçon

- **Classe / période** : CE1, période 2.
- **Compétences liées** (`skillIds`) : `ce1-parity`, `ce1-double-half-facts`, `ce1-half-even` — exercices : la fiche « Exercices CE1 — Pair, impair, doubles et moitiés ».
- **Objectif** : reconnaître un nombre pair par son chiffre des unités, relier double et moitié, calculer la moitié en décomposant.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « pair », « impair », « double », « moitié ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Pair, impair, doubles et moitiés » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Par deux** — On range les voitures par paires : s'il n'en reste aucune seule, le nombre est pair. | `groupObjects` (groupBy 2) |
| 3 | **Le chiffre des unités** — Pairs : 0, 2, 4, 6, 8 à la fin ; 768 est pair, 773 est impair. | `numberLine` (pas de 2) |
| 4 | **À toi** — Interaction : range les nombres dans le garage « pair » ou « impair ». | `groupObjects` (tri, interactif) |
| 5 | **Le double** — Deux garages identiques : le double de 35 est 70. | `groupObjects` (deux rangées) + `tensUnits` |
| 6 | **La moitié** — Partager en deux parts égales : la moitié de 300 est 150. | `groupObjects` (share) |
| 7 | **Décomposer** — Moitié de 46 = moitié de 40 + moitié de 6 = 20 + 3 = 23. | `tensUnits` (share) + `calcChain` |
| 8 | **À toi** — Interaction : partage 84 casques en deux. | `tensUnits` (share, interactif) |
| 9 | **Les pièces de 25** — 25, 50, 75, 100 : quatre pièces de 25 font 100. | `numberLine` (pas de 25) |
| 10 | **Je retiens** — « Un nombre est pair s'il finit par 0, 2, 4, 6 ou 8. La moitié de 46 : moitié de 40 (20) + moitié de 6 (3) = 23. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `groupObjects`, `numberLine`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 573 est-il pair ?
2. Double de 35 ? (QCM : 60, 70, 75)
3. Moitié de 46 ?
4. 4 × 25 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce1-pair-impair-doubles-moities.json`, validée par `lesson.schema.json`, `skillIds` = `ce1-parity`, `ce1-double-half-facts`, `ce1-half-even`.
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
