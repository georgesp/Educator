---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYn0
rev: 2026-10-13T18:57:51Z
url: https://github.com/georgesp/Educator/issues/115
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000147 — Leçon CP — Compléments à la dizaine
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2160
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique comment trouver ce qu'il manque pour arriver à la dizaine suivante, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 4.
- **Compétences liées** (`skillIds`) : `cp-complement-next-ten` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : trouver le complément à la dizaine supérieure en s'appuyant sur les compléments à 10.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « complément », « dizaine », « il manque ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Compléments à la dizaine » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Les amis de 10** — Rappel : 4 et 6, 7 et 3, 8 et 2… remplissent le garage. | `tenFrame` |
| 3 | **74 → 80** — 74 a 4 unités : il en manque 6 pour faire une nouvelle dizaine. | `tensUnits` + `tenFrame` |
| 4 | **Sur la piste** — De 74 au drapeau 80 : 6 bonds. | `numberLine` (drapeau 80) |
| 5 | **L'écrire** — 74 + 6 = 80, ou 74 + … = 80. | `equation` (case à trou) |
| 6 | **À toi** — Interaction : 38 → 40, l'enfant remplit le garage. | `tenFrame` (interactif) |
| 7 | **À toi** — Interaction : 53 + … = 60. | `equation` (interactif) |
| 8 | **Rapide** — Trois nombres défilent : 21, 47, 86 ; on donne le complément. | `calcChain` |
| 9 | **À quoi ça sert** — C'est la première étape pour calculer 47 + 8. | `calcChain` |
| 10 | **Je retiens** — « Je regarde les unités : il manque leur ami de 10 pour arriver à la dizaine. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `equation`, `numberLine`, `recapCard`, `tenFrame`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 76 + … = 80
2. 41 + … = 50 (QCM : 9, 10, 1)
3. 18 + … = 20
4. 65 + … = 70

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [x] Leçon `content/lessons/math/cp-complement-next-ten.json`, validée par `lesson.schema.json`, `skillIds` = `cp-complement-next-ten`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [x] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [x] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [x] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [x] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
