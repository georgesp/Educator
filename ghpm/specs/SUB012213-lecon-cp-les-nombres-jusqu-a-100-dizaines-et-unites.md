---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACa70
rev: 2026-10-17T13:46:46Z
url: https://github.com/georgesp/Educator/issues/225
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012213 — Leçon CP — Les nombres jusqu'à 59 : dizaines et unités"
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 1930
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique ce que sont une dizaine et une unité, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 1 (le programme travaille la base dix dès la période 1 ; les nombres jusqu'à 59 sont acquis au plus tard en période 2).
- **Compétences liées** (`skillIds`) : `cp-num-to-59` — exercices : la fiche « Exercices CP — Les nombres jusqu'à 100 : dizaines et unités ». Les nombres de 60 à 100 et l'écriture en lettres ont leur propre leçon (« Leçon CP — Les nombres de 60 à 100 et en lettres »).
- **Objectif** : grouper par 10, lire et écrire un nombre jusqu'à 59 comme des dizaines et des unités.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « dizaine », « unité », « chiffre », « nombre ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Les nombres jusqu'à 59 : dizaines et unités » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Beaucoup de voitures** — 34 voitures en vrac : difficile à compter. | `groupObjects` |
| 3 | **Par paquets de 10** — On range les voitures par 10 : 3 paquets et 4 toutes seules. | `groupObjects` (groupBy 10) |
| 4 | **Barres et cubes** — 1 paquet de 10 = 1 barre ; une voiture seule = 1 cube. | `tensUnits` |
| 5 | **Écrire 34** — Le 3 pour les dizaines, le 4 pour les unités. | `tensUnits` + `placeValueTable` |
| 6 | **À toi** — Interaction : fais des paquets de 10 avec 26 casques. | `groupObjects` (groupBy, interactif) |
| 7 | **34 ou 43 ?** — 3 barres et 4 cubes, ou 4 barres et 3 cubes : pas le même nombre. | `tensUnits` |
| 8 | **Le dire et l'écrire** — « trente-quatre » : trente, ce sont 3 dizaines ; quatre, ce sont 4 unités ; 34 = 30 + 4. | `tensUnits` + `placeValueTable` |
| 9 | **À toi** — Interaction : fabrique avec les barres et les cubes le nombre entendu au clic sur le haut-parleur (cinquante-deux). | `tensUnits` (interactif) |
| 10 | **Je retiens** — « Dans 34, le 3 compte les dizaines (3 paquets de 10) et le 4 compte les unités. 34 et 43 ne sont pas pareils. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `groupObjects`, `placeValueTable`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Combien ? (3 barres et 4 cubes)
2. Écris le nombre entendu au clic : cinquante-trois.
3. 4 dizaines et 7 unités = ? (QCM : 47, 74, 11)
4. Combien de dizaines dans 52 ?

Chaque question a une explication courte, affichée après la réponse et lisible au clic (rien n'est lu tout seul, décision du 2026-10-15).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [x] Leçon `content/lessons/math/cp-dizaines-et-unites.json`, validée par `lesson.schema.json`, `skillIds` = `cp-num-to-59`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [x] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [x] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [x] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.

## Remarques et décisions

- Relecture (2026-10-17) : le tableau de numération n'a plus de colonne des centaines sous 100 ; la question « 34 ou 43 ? » montre les barres et les cubes (le signe < arrive en période 3).
