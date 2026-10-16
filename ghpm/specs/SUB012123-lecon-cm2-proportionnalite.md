---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACap8
rev: 2026-10-11T18:51:25Z
url: https://github.com/georgesp/Educator/issues/211
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012123 — Leçon CM2 — Proportionnalité
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 1860
---

## User story

En tant qu'enfant de CM2, je veux une leçon courte et animée qui m'explique comment résoudre un problème de proportionnalité en deux étapes, avant de m'entraîner.

## Leçon

- **Classe / période** : CM2, période 3.
- **Compétences liées** (`skillIds`) : `cm2-proportion` — exercices : la fiche « Exercices CM2 — Proportionnalité ».
- **Objectif** : combiner « fois plus » et addition de colonnes pour résoudre des problèmes de proportionnalité.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « proportionnalité », « tableau », « linéarité ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Proportionnalité » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **La pile de feuilles** — 200 feuilles → 24 mm. Et 250 ? | `proportionTable` |
| 3 | **Une colonne facile** — 50 feuilles = 200 ÷ 4 → 24 ÷ 4 = 6 mm. | `proportionTable` (flèche ÷ 4) |
| 4 | **Ajouter** — 250 = 200 + 50 → 24 + 6 = 30 mm. | `proportionTable` (flèche +) |
| 5 | **À toi** — Interaction : 4 pneus 180 € ; 10 pneus ? | `proportionTable` (interactif) |
| 6 | **Plusieurs chemins** — 10 = 2 × 4 + 2, ou (4 ÷ 2) × 5 : on choisit le plus simple. | `proportionTable` |
| 7 | **Proportionnel ou pas ?** — Le prix d'un lot avec réduction n'est pas proportionnel. | `barModel` |
| 8 | **À toi** — Interaction : choisis les tableaux de proportionnalité. | `proportionTable` (QCM) |
| 9 | **Vérifier** — Plus de feuilles → plus épais, dans le même rapport. | `barModel` |
| 10 | **Je retiens** — « Je cherche une colonne facile (la moitié, le quart, 2 fois plus), puis j'ajoute ou je multiplie. » | `recapCard` |

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

1. 200 feuilles 24 mm : 250 feuilles ?
2. 4 pneus 180 € : 10 pneus ? (QCM : 400 €, 450 €, 186 €)
3. Ce tableau est-il de proportionnalité ? (2 → 5, 4 → 9)
4. 6 casques 90 € : 9 casques ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm2-proportionnalite.json`, validée par `lesson.schema.json`, `skillIds` = `cm2-proportion`.
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
