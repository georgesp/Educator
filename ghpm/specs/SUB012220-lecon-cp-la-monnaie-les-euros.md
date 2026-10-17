---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdp_8
url: https://github.com/georgesp/Educator/issues/330
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012220 — Leçon CP — La monnaie : les euros"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2076
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique les pièces et les billets en euros, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 4.
- **Compétences liées** (`skillIds`) : `cp-money-value`, `cp-money-make` — exercices : la fiche « Exercices CP — La monnaie : les euros ».
- **Objectif** : reconnaître les pièces et les billets en euros, trouver la valeur d'un porte-monnaie, faire une somme, rendre la monnaie en complétant.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « euro », « pièce », « billet », « prix », « payer », « rendre la monnaie ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « La monnaie : les euros » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Pièces et billets** — Pièces de 1 € et de 2 €, billets de 5, 10, 20 et 50 €. | `coins` |
| 3 | **Compter un porte-monnaie** — Je commence par les billets : 10 + 10 = 20, puis les pièces : 21, 22, 23 €. | `coins` + `calcChain` |
| 4 | **Comme les dizaines** — Un billet de 10 €, c'est 10 pièces de 1 € : 2 billets et 3 pièces, 23 €. | `tensUnits` |
| 5 | **À toi** — Interaction : fais 35 € avec les billets et les pièces. | `coins` (interactif) |
| 6 | **Qui a le plus ?** — 25 € contre 22 € : on compte chaque porte-monnaie, puis on compare. | `coins` + `compareNumbers` |
| 7 | **Payer** — Un livre coûte 7 € : je donne un billet de 10 €, c'est plus que le prix. | `coins` |
| 8 | **Rendre la monnaie** — On complète de 7 jusqu'à 10 : 8, 9, 10 ; on rend 3 €. | `numberLine` (bonds) |
| 9 | **À toi** — Interaction : un jeu coûte 15 €, on paie avec un billet de 20 € ; rends la monnaie. | `coins` (interactif) |
| 10 | **Je retiens** — « Je compte d'abord les billets, puis les pièces. Pour rendre la monnaie, je complète du prix jusqu'à ce qu'on m'a donné. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `coins`, `compareNumbers`, `numberLine`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » ; `coins` : fiche « Extensions des animations et du quiz des leçons du CE2 » (SUB011946).
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 2 billets de 10 € et 3 pièces de 1 € : combien d'euros ? (saisie)
2. Qui a le plus d'argent ? (QCM sur deux porte-monnaie)
3. Fais 12 € avec les pièces et les billets.
4. Un livre coûte 7 €, tu paies avec 10 €. Combien te rend-on ? (QCM : 3 €, 7 €, 17 €)

Chaque question a une explication courte, affichée après la réponse et lisible au clic (rien n'est lu tout seul, décision du 2026-10-15).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Résumé par niveau du programme de mathématiques du cycle 2 (2024) — attendus de grandeurs et mesures du CP, à vérifier sur le PDF officiel](https://maitresselunicole.fr/wp-content/uploads/2025/03/resume-par-niveaux-programme-de-mathematiques-cycle-2-2024-1.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-la-monnaie.json`, validée par `lesson.schema.json`, `skillIds` = `cp-money-value`, `cp-money-make`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Fiche « Extensions des animations et du quiz des leçons du CE2 » (SUB011946) : `coins`.
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
