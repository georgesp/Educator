---
type: Issue
title: "Leçon CM2 — Les fractions : écrire, placer et comparer"
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-3-cm2.md"
order: 2220
---
## User story

En tant qu'enfant de CM2, je veux une leçon courte et animée qui m'explique comment situer et comparer des fractions, avant de m'entraîner.

## Leçon

- **Classe / période** : CM2, période 1.
- **Compétences liées** (`skillIds`) : `cm2-frac-place-frame`, `cm2-frac-compare` — exercices : la fiche « Exercices CM2 — Les fractions : écrire, placer et comparer ».
- **Objectif** : passer d'une fraction à « entier + fraction », encadrer, placer, et comparer à 1 ou à 1/2.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « fraction », « encadrer », « dénominateur commun ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Les fractions : écrire, placer et comparer » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Entier + fraction** — 23/6 = 18/6 + 5/6 = 3 + 5/6. | `fractionBar` (plusieurs unités) |
| 3 | **Dans l'autre sens** — 2 + 3/8 = 16/8 + 3/8 = 19/8. | `fractionBar` + `equation` |
| 4 | **À toi** — Interaction : complète 17/5 = 3 + …/5. | `equation` (interactif) |
| 5 | **Placer** — La piste graduée en sixièmes : la voiture s'arrête à 23/6. | `numberLine` (fractions) |
| 6 | **Même dénominateur** — 5/12 et 2/3 = 8/12 : 5/12 < 2/3. | `fractionBar` (stack) |
| 7 | **Comparer à 1/2** — 5/12 < 6/12 = 1/2 ; 7/10 > 1/2. | `fractionBar` (stack) |
| 8 | **À toi** — Interaction : range 3/4, 5/8, 1/2. | `fractionBar` (ordre, interactif) |
| 9 | **Dénominateurs plus grands** — Jusqu'à 60 : des minutes ! 15/60 d'heure = 1/4 d'heure. | `fractionBar` |
| 10 | **Je retiens** — « Pour comparer, je mets au même dénominateur quand l'un est multiple de l'autre, ou je compare à 1/2 ou à 1. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `equation`, `fractionBar`, `numberLine`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 23/6 = 3 + …/6
2. 5/12 … 1/2 (QCM : <, >, =)
3. 2 + 3/8 = …/8
4. Range : 3/4, 5/8, 1/2

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm2-fractions.json`, validée par `lesson.schema.json`, `skillIds` = `cm2-frac-place-frame`, `cm2-frac-compare`.
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
