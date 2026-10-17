---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdqFk
url: https://github.com/georgesp/Educator/issues/333
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012223 — Leçon CP — Longueurs et masses
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2074
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique comment comparer et mesurer des longueurs et comparer des masses, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 3.
- **Compétences liées** (`skillIds`) : `cp-length-compare-measure`, `cp-mass-compare`, `cp-length-m-cm` — exercices : la fiche « Exercices CP — Longueurs et masses ».
- **Objectif** : comparer deux longueurs, mesurer un segment à la règle en centimètres, savoir que 1 m = 100 cm, comparer deux masses avec une balance.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « long », « court », « longueur », « mètre », « centimètre », « lourd », « léger », « masse ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Longueurs et masses » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Plus long, plus court** — Deux rubans décalés : on aligne les bouts au départ pour comparer. | `ruler` (deux segments) |
| 3 | **Mesurer avec la règle** — On place le 0 au bout du crayon, on lit à l'autre bout : 9 cm. | `ruler` |
| 4 | **À toi** — Interaction : allonge le segment jusqu'à 6 cm. | `ruler` (interactif) |
| 5 | **Le mètre** — Une piste de 1 m : 10 barres de 10 cm, 100 cm. | `ruler` + `tensUnits` |
| 6 | **Quelle unité ?** — Une gomme se mesure en cm, la piste de course en m. | `calcChain` (association) |
| 7 | **Plus lourd, plus léger** — La balance penche du côté le plus lourd. | `scale` |
| 8 | **Gros mais léger** — Le gros ballon est plus léger que le petit casque : on pèse, on ne devine pas. | `scale` |
| 9 | **À toi** — Interaction : pose le casque et le pneu sur la balance. Lequel est le plus lourd ? | `scale` (interactif) |
| 10 | **Je retiens** — « Pour mesurer, je pose le 0 de la règle au bout. 1 m = 100 cm. Sur la balance, le plateau le plus lourd descend. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `recapCard`, `ruler`, `scale`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » ; `ruler` et `scale` : fiche « Extensions des animations et du quiz des leçons du CE2 » (SUB011946).
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Quelle est la longueur du crayon ? (règle)
2. Combien de centimètres dans 1 m ?
3. Une porte mesure : 2 cm, 2 m ou 20 m ? (QCM)
4. Quel objet est le plus lourd ? (balance)

Chaque question a une explication courte, affichée après la réponse et lisible au clic (rien n'est lu tout seul, décision du 2026-10-15).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Résumé par niveau du programme de mathématiques du cycle 2 (2024) — attendus de grandeurs et mesures du CP, à vérifier sur le PDF officiel](https://maitresselunicole.fr/wp-content/uploads/2025/03/resume-par-niveaux-programme-de-mathematiques-cycle-2-2024-1.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-longueurs-et-masses.json`, validée par `lesson.schema.json`, `skillIds` = `cp-length-compare-measure`, `cp-mass-compare`, `cp-length-m-cm`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Fiche « Extensions des animations et du quiz des leçons du CE2 » (SUB011946) : `ruler`, `scale`.
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
