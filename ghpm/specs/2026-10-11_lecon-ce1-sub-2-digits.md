---
type: Issue
title: Leçon CE1 — Soustractions de nombres plus petits que 100
status: Todo
size: M
parent: "#1"
order: 1170
---
## User story

En tant qu'enfant de CE1, je veux une leçon courte et animée qui m'explique comment soustraire deux nombres à deux chiffres, en ligne et en colonnes, avec deux techniques au choix, avant de m'entraîner.

## Leçon

- **Classe / période** : CE1, période 3.
- **Compétences liées** (`skillIds`) : `ce1-sub-2-digits` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : soustraire en retirant les dizaines puis les unités, par l'écart, puis poser la soustraction en colonnes avec la technique du cassage ou celle de la compensation.
- **Durée visée** : 3 à 5 minutes, 11 slides.
- **Vocabulaire du programme** : « différence », « soustraction posée », « dizaine », « unité », « retenue ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Soustractions de nombres plus petits que 100 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **En ligne** — 74 − 28 : 74 − 20 = 54, puis 54 − 8 = 46. | `calcChain` |
| 3 | **L'écart** — 74 − 68 : de 68 à 74, il y a 6. | `numberLine` (écart) |
| 4 | **Poser** — 74 au-dessus de 28, unités sous unités ; 4 − 8 : impossible. | `columnOperation` (alignement) |
| 5 | **Technique 1 : le cassage** — On casse une dizaine de 74 : 6 dizaines et 14 unités ; 14 − 8 = 6, 6 − 2 = 4 → 46. | `columnOperation` (method cassage) + `tensUnits` |
| 6 | **Technique 2 : la compensation** — On ajoute 10 unités en haut (14) et 1 dizaine en bas (2 + 1 = 3) : 14 − 8 = 6, 7 − 3 = 4 → 46. L'écart ne change pas. | `columnOperation` (method compensation) + `numberLine` (écart conservé) |
| 7 | **Même résultat** — Les deux techniques côte à côte donnent 46 : j'utilise celle de ma classe. | `columnOperation` (deux colonnes) |
| 8 | **À toi** — Interaction : 63 − 27 posée, avec la technique choisie. | `columnOperation` (interactif) |
| 9 | **Vérifier** — Interaction : 46 + 28 = ? L'addition vérifie la soustraction. | `equation` (famille, interactif) |
| 10 | **Choisir** — Quand l'écart est petit, je compte en avançant ; sinon je pose. | `numberLine` |
| 11 | **Je retiens** — « En colonnes, je commence par les unités. Si je ne peux pas enlever, je casse une dizaine, ou j'ajoute 10 en haut et 1 dizaine en bas : les deux marchent. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `columnOperation`, `equation`, `numberLine`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 58 − 23 = ?
2. 74 − 28 = ? (QCM : 46, 54, 56)
3. Vérifie : 91 − 37 = 54 ? (vrai / faux)
4. 80 − 45 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce1-sub-2-digits.json`, validée par `lesson.schema.json`, `skillIds` = `ce1-sub-2-digits`.
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

## Remarques et décisions

- Les deux techniques de soustraction posée sont montrées (décision de Georges du 2026-10-11). La technique utilisée ensuite dans les exercices est celle choisie par l'enfant ou le parent (réglage du profil, modifiable dans les Options) ; le résultat est accepté quelle que soit la technique.
