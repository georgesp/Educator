---
type: Issue
title: "Leçon CM1 — Fractions : comparer, additionner et prendre une fraction d'une quantité"
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-2-cm1.md"
order: 1920
---
## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique comment calculer avec des fractions, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 2.
- **Compétences liées** (`skillIds`) : `cm1-frac-compare-add`, `cm1-frac-of-quantity` — exercices : la fiche « Exercices CM1 — Fractions : comparer, additionner et prendre une fraction d'une quantité ».
- **Objectif** : comparer et ajouter des fractions avec les bandes ; calculer 3/4 de 20 en partageant puis en prenant 3 parts.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « fraction d'une quantité », « numérateur », « dénominateur ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Fractions : comparer, additionner et prendre une fraction d'une quantité » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Comparer** — 3/10 et 1/5 = 2/10 : 3/10 > 1/5. | `fractionBar` (stack) |
| 3 | **Additionner** — 2/5 + 4/5 = 6/5 = 1 + 1/5. | `fractionBar` |
| 4 | **À toi** — Interaction : 1/2 + 3/4 avec des quarts. | `fractionBar` (interactif) |
| 5 | **Une fraction de 20** — 20 casques partagés en 4 parts égales : 5 par part. | `groupObjects` (share) |
| 6 | **Prendre 3 parts** — 3 parts : 15 casques. 3/4 de 20 = 15. | `groupObjects` + `fractionBar` |
| 7 | **La règle** — Diviser par le dénominateur, multiplier par le numérateur. | `calcChain` |
| 8 | **À toi** — Interaction : 2/5 de 35 voitures. | `groupObjects` (share, interactif) |
| 9 | **En problème** — Un circuit de 600 m, la voiture a fait les 2/3 : combien de mètres ? | `barModel` |
| 10 | **Je retiens** — « 3/4 de 20 : je partage 20 en 4 (5), puis j'en prends 3 (15). » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `barModel`, `calcChain`, `fractionBar`, `groupObjects`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 3/4 de 20 = ?
2. 2/5 + 4/5 = ? (QCM : 6/10, 6/5, 8/5)
3. 3/10 … 1/5 (QCM : <, >, =)
4. 2/3 de 600 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-fractions-calculer.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-frac-compare-add`, `cm1-frac-of-quantity`.
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
