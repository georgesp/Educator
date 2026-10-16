---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaPA
rev: 2026-10-11T18:50:47Z
url: https://github.com/georgesp/Educator/issues/187
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012033 — Leçon CM1 — La proportionnalité
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1590
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique ce qu'est une situation de proportionnalité, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 5.
- **Compétences liées** (`skillIds`) : `cm1-proportion` — exercices : la fiche « Exercices CM1 — La proportionnalité ».
- **Objectif** : reconnaître une situation de proportionnalité, utiliser « 2 fois plus » et l'addition de colonnes.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « proportionnalité », « tableau », « fois plus ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « La proportionnalité » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Le stand de casques** — 1 casque 15 €, 2 casques 30 €, 3 casques 45 €. | `proportionTable` |
| 3 | **Fois plus** — 6 casques = 2 fois 3 casques : 2 × 45 = 90 €. | `proportionTable` (flèche × 2) |
| 4 | **Ajouter des colonnes** — 5 casques = 2 + 3 casques : 30 + 45 = 75 €. | `proportionTable` (flèche +) |
| 5 | **À toi** — Interaction : complète le tableau 2 → 8, 5 → 20, 7 → ?. | `proportionTable` (interactif) |
| 6 | **Pas proportionnel !** — L'âge et la taille : 2 fois plus vieux, pas 2 fois plus grand. | `barModel` |
| 7 | **Recette** — Pour 4 crêpes 100 g de farine ; pour 12 crêpes ? | `proportionTable` |
| 8 | **À toi** — Interaction : choisis les situations proportionnelles. | `proportionTable` (QCM) |
| 9 | **Les 5 T-shirts** — 5 T-shirts à 16 € : 80 €. | `proportionTable` |
| 10 | **Je retiens** — « Si j'achète 2 fois plus, je paie 2 fois plus. Le prix de 5, c'est le prix de 2 plus le prix de 3. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `barModel`, `proportionTable`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 3 casques 45 € : 6 casques ?
2. 2 → 8, 5 → 20 : 7 → ? (QCM : 22, 28, 35)
3. Est-ce proportionnel : l'âge et la taille ?
4. 4 crêpes : 100 g de farine. 12 crêpes ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-proportionnalite.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-proportion`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 3, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
