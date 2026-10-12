---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYsk
rev: 2026-10-12T19:40:28Z
url: https://github.com/georgesp/Educator/issues/118
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000150 — Leçon CP — Soustractions jusqu'à 20
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2190
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique comment soustraire jusqu'à 20, même quand il faut casser une dizaine, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 3.
- **Compétences liées** (`skillIds`) : `cp-sub-within-20` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : soustraire en enlevant les unités, en cassant une dizaine, en reculant jusqu'à 10 ou en cherchant l'écart.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « soustraction », « dizaine », « écart », « il reste ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Soustractions jusqu'à 20 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Assez d'unités** — 15 − 3 : on enlève 3 cubes, la barre reste → 12. | `tensUnits` |
| 3 | **Casser une dizaine** — 13 − 5 : seulement 3 cubes ; on casse la barre en 10 cubes, on en enlève 5 → 8. | `tensUnits` (casser) |
| 4 | **Reculer jusqu'à 10** — 13 − 5 = 13 − 3 − 2 : on s'arrête au drapeau du 10, puis on recule encore de 2. | `numberLine` + `calcChain` |
| 5 | **À toi** — Interaction : 14 − 6 en passant par 10. | `numberLine` (interactif) |
| 6 | **L'écart** — 17 − 15 : de 15 à 17, 2 bonds. | `numberLine` (écart) |
| 7 | **La famille** — 8 + 7 = 15, donc 15 − 7 = 8. | `equation` (famille) |
| 8 | **Petit problème** — 12 casques sur l'étagère, 5 tombent. Combien en reste-t-il ? | `barModel` (tout / parties) |
| 9 | **À toi** — Interaction : 16 − … = 9. | `equation` (case à trou, interactif) |
| 10 | **Je retiens** — « Pour enlever, je peux reculer jusqu'à 10, casser une dizaine ou chercher l'écart. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `barModel`, `calcChain`, `equation`, `numberLine`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 18 − 5 = ?
2. 12 − 4 = ? (QCM : 7, 8, 9)
3. 20 − … = 13
4. 11 − 9 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-sub-within-20.json`, validée par `lesson.schema.json`, `skillIds` = `cp-sub-within-20`.
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
