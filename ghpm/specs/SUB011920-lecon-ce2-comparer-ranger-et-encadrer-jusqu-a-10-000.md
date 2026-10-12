---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZSo
rev: 2026-10-11T18:49:14Z
url: https://github.com/georgesp/Educator/issues/142
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011920 — Leçon CE2 — Comparer, ranger et encadrer jusqu'à 10 000"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 960
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique comment comparer et ranger de grands nombres, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 2.
- **Compétences liées** (`skillIds`) : `ce2-num-compare`, `ce2-num-frame` — exercices : la fiche « Exercices CE2 — Comparer, ranger et encadrer jusqu'à 10 000 ».
- **Objectif** : comparer deux nombres en regardant d'abord le nombre de chiffres puis les chiffres de gauche à droite ; encadrer et placer sur une droite graduée.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « plus grand que », « plus petit que », « encadrer », « ranger », « croissant », « décroissant ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Comparer, ranger et encadrer jusqu'à 10 000 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Les signes** — < et > : la pointe montre le plus petit nombre. | `compareNumbers` |
| 3 | **Nombre de chiffres** — 987 < 1 024 : 3 chiffres contre 4. | `compareNumbers` |
| 4 | **Rang par rang** — 6 234 et 6 243 : milliers égaux, centaines égales, dizaines 3 < 4. | `compareNumbers` (rang en rouge) |
| 5 | **À toi** — Interaction : choisis le signe pour 5 229 … 5 292. | `compareNumbers` (interactif) |
| 6 | **Ranger** — Les voitures portent des nombres : on les range sur la grille du plus petit au plus grand. | `sequencePattern` (ordre, interactif) |
| 7 | **Encadrer** — 4 000 < 4 635 < 5 000 : entre deux drapeaux de la piste. | `numberLine` (pas de 1 000) |
| 8 | **La graduation** — De 100 en 100 : la voiture s'arrête sur 3 400. | `numberLine` (pas de 100) |
| 9 | **À toi** — Interaction : place 6 300 sur la piste. | `numberLine` (interactif) |
| 10 | **Je retiens** — « Pour comparer, je compte les chiffres, puis je compare rang par rang en partant de la gauche. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `compareNumbers`, `numberLine`, `recapCard`, `sequencePattern`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 6 234 … 6 243 (QCM : <, >, =)
2. Quel est le plus grand : 5 999 ou 6 001 ?
3. Encadre 7 382 entre deux milliers.
4. Range dans l'ordre croissant : 3 210, 3 120, 3 201.

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-comparer-ranger-10000.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-num-compare`, `ce2-num-frame`.
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
