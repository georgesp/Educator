---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYdM
rev: 2026-10-13T18:57:40Z
url: https://github.com/georgesp/Educator/issues/109
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000141 — Leçon CP — Ajouter 9
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2100
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique l'astuce pour ajouter 9 très vite, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 4.
- **Compétences liées** (`skillIds`) : `cp-add-9` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : ajouter 9 en ajoutant 10 puis en retirant 1.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « dizaine », « ajouter », « enlever ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Ajouter 9 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **9, c'est presque 10** — Une barre de dix avec un cube en moins : 9 = 10 − 1. | `tensUnits` |
| 3 | **L'astuce** — 37 + 9 : + 10 → 47, puis − 1 → 46. | `calcChain` |
| 4 | **Sur la piste** — Un grand bond de 10, puis un petit pas en arrière. | `numberLine` |
| 5 | **Pourquoi ça marche** — On ajoute une barre entière et on rend un cube. | `tensUnits` |
| 6 | **À toi** — Interaction : 24 + 9, l'enfant fait les deux étapes. | `calcChain` (interactif) |
| 7 | **Attention** — 58 + 9 : + 10 → 68, − 1 → 67 (et non 69 !). | `calcChain` |
| 8 | **Choisis le bon chemin** — Interaction : quelle chaîne donne 42 + 9 ? | `calcChain` (QCM) |
| 9 | **Course éclair** — Trois calculs à la suite : la voiture avance à chaque bonne réponse. | `numberLine` |
| 10 | **Je retiens** — « Pour ajouter 9, j'ajoute 10 puis j'enlève 1. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `numberLine`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 15 + 9 = ?
2. 42 + 9 = ? (QCM : 50, 51, 52)
3. 66 + 9 = ?
4. 9 + 27 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [x] Leçon `content/lessons/math/cp-add-9.json`, validée par `lesson.schema.json`, `skillIds` = `cp-add-9`.
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
