---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYkk
rev: 2026-10-12T19:40:18Z
url: https://github.com/georgesp/Educator/issues/113
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000145 — Leçon CP — Additions jusqu'à 10
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2140
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique ce que veut dire ajouter et comment trouver le résultat d'une addition jusqu'à 10, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 1.
- **Compétences liées** (`skillIds`) : `cp-add-sum-10` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : comprendre l'addition comme une réunion, utiliser les signes + et =, compter à partir du plus grand nombre.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « ajouter », « plus », « égal », « addition ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Additions jusqu'à 10 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Réunir** — 3 voitures rouges et 2 voitures bleues se rejoignent sur la ligne de départ : on compte, il y en a 5. | `groupObjects` (join) |
| 3 | **Les signes** — On l'écrit 3 + 2 = 5 : « + » veut dire « on met ensemble », « = » veut dire « ça fait autant que ». | `equation` |
| 4 | **À toi** — Interaction : l'enfant fait glisser 4 voitures puis 3 sur la grille et trouve 7. | `groupObjects` (join, interactif) |
| 5 | **Partir du plus grand** — 2 + 6 : la voiture part de 6 et fait 2 bonds sur la piste : 7, 8. | `numberLine` (0 à 10, bonds de 1) |
| 6 | **Dans l'ordre que je veux** — 2 + 6 et 6 + 2 donnent le même résultat : les voitures échangent leur place. | `groupObjects` + `equation` |
| 7 | **Le garage de 10** — Le garage a 10 places : 6 voitures + 4 voitures le remplissent. Les « amis de 10 ». | `tenFrame` |
| 8 | **À toi** — Interaction : 7 voitures sont garées, l'enfant ajoute celles qui manquent pour remplir le garage. | `tenFrame` (interactif) |
| 9 | **Le nombre caché** — 4 + … = 9 : on compte les bonds de 4 jusqu'à 9. | `equation` (case à trou) + `numberLine` |
| 10 | **Je retiens** — « Ajouter, c'est réunir. Je pars du plus grand nombre et j'avance. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `equation`, `groupObjects`, `numberLine`, `recapCard`, `tenFrame`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 5 + 3 = ? (saisie)
2. 2 + 7 = ? (QCM : 8, 9, 10)
3. 4 + … = 10 (saisie)
4. Vrai ou faux : 3 + 4 = 4 + 3

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-add-sum-10.json`, validée par `lesson.schema.json`, `skillIds` = `cp-add-sum-10`.
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
