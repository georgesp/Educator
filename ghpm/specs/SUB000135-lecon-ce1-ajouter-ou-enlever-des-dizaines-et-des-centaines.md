---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYSY
rev: 2026-10-11T18:48:43Z
url: https://github.com/georgesp/Educator/issues/103
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000135 — Leçon CE1 — Ajouter ou enlever des dizaines et des centaines
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 830
---

## User story

En tant qu'enfant de CE1, je veux une leçon courte et animée qui m'explique comment ajouter ou enlever des dizaines et des centaines entières, avant de m'entraîner.

## Leçon

- **Classe / période** : CE1, période 4.
- **Compétences liées** (`skillIds`) : `ce1-add-sub-tens-hundreds` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : ajouter ou soustraire des dizaines ou des centaines entières, y compris en changeant de centaine (746 + 80).
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « centaine », « dizaine », « unité ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Ajouter ou enlever des dizaines et des centaines » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Une centaine** — 10 barres s'assemblent en une plaque : 1 centaine = 10 dizaines. | `tensUnits` (plaque) |
| 3 | **+ 60** — 234 + 60 : 3 dizaines + 6 dizaines = 9 dizaines → 294. | `tensUnits` + `placeValueTable` |
| 4 | **Changer de centaine** — 746 + 80 : 4 + 8 = 12 dizaines = 1 centaine et 2 dizaines → 826. | `tensUnits` (échange) |
| 5 | **+ 300** — 425 + 300 : seul le chiffre des centaines change → 725. | `placeValueTable` |
| 6 | **Enlever** — 650 − 70 : on casse une centaine en 10 dizaines → 580. | `tensUnits` (casser) |
| 7 | **À toi** — Interaction : 358 + 50. | `tensUnits` (interactif) |
| 8 | **Bonds de 100** — La voiture saute de 100 en 100 sur une piste de 0 à 1 000. | `numberLine` (pas de 100) |
| 9 | **À toi** — Interaction : 912 − 400. | `placeValueTable` (interactif) |
| 10 | **Je retiens** — « J'ajoute les dizaines aux dizaines, les centaines aux centaines. 10 dizaines font 1 centaine. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `numberLine`, `placeValueTable`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 234 + 60 = ?
2. 746 + 80 = ? (QCM : 726, 826, 836)
3. 650 − 70 = ?
4. 425 + … = 725

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce1-add-sub-tens-hundreds.json`, validée par `lesson.schema.json`, `skillIds` = `ce1-add-sub-tens-hundreds`.
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
