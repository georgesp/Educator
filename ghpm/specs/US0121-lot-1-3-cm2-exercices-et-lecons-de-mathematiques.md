---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYzA
url: https://github.com/georgesp/Educator/issues/121
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: US0121 — Lot 6 — CM2 — exercices et leçons
status: Todo
size: XL
estimate:
startDate:
endDate:
parent:
order: 2100
---

Source : demande de Georges du 2026-10-11 — ajouter au lot 1 les exercices et les leçons de mathématiques de chaque classe.

- **Lot** : 6 — CM2 — exercices et leçons (mathématiques, orthographe et grammaire)
- **Prérequis** : fiche « Lot 5 — CM1 — exercices et leçons » (décimaux, division posée).
- **Lot suivant** : fiche « Lot packaging — Installeurs, signature Apple et tests Windows ».

## Objectif

Donner au CM2 ses propres exercices et leçons de mathématiques sur les cinq périodes, conformément au programme du cycle 3 de 2025 (en vigueur au CM2 à la rentrée 2026) : nombres jusqu'à 999 999 999, multiples et diviseurs, calcul mental sur les entiers et les décimaux, division posée d'un entier ou d'un décimal, fractions (dénominateurs jusqu'à 60), décimaux jusqu'aux millièmes, algèbre, proportionnalité et résolution de problèmes. Educator s'arrête au CM2.

Chaque thème a **deux fiches** : une fiche « Exercices » (compétences JSON, générateur, écran de série) et une fiche « Leçon »
(8 à 12 slides, 3 à 5 minutes, animations dans la charte Grand Prix, quiz).

## Thèmes

| Thème | Périodes | Compétences | Fiches |
|---|---|---|---|
| Les grands nombres jusqu'au milliard | 3 | `cm2-num-read-write`, `cm2-num-compare` | Exercices + Leçon |
| Multiples, diviseurs et diviseurs communs | 2, 3 | `cm2-divisors`, `cm2-common-divisors` | Exercices + Leçon |
| Calcul mental sur les entiers | 1, 2 | `cm2-add-sub-near-tens`, `cm2-mul-round`, `cm2-div-by-4-8` | Exercices + Leçon |
| Opérations posées, estimations et parenthèses | 1, 2, 3 | `cm2-column-integers`, `cm2-estimate`, `cm2-parentheses` | Exercices + Leçon |
| La division posée d'un entier ou d'un décimal | 1, 3 | `cm2-div-integer`, `cm2-div-decimal` | Exercices + Leçon |
| Les fractions : écrire, placer et comparer | 1, 2 | `cm2-frac-place-frame`, `cm2-frac-compare` | Exercices + Leçon |
| Calculer avec les fractions | 2, 3 | `cm2-frac-add-sub`, `cm2-frac-times-int` | Exercices + Leçon |
| Les nombres décimaux jusqu'aux millièmes | 1, 2, 4 | `cm2-dec-read-write`, `cm2-dec-compare-round`, `cm2-dec-usual-facts` | Exercices + Leçon |
| Calculer avec les décimaux | 1, 2, 3 | `cm2-dec-mental`, `cm2-dec-mul-div-powers`, `cm2-dec-mul-5-50`, `cm2-dec-mul-int` | Exercices + Leçon |
| Égalités à trous, programmes de calcul et suites | 2, 4 | `cm2-algebra-missing`, `cm2-sequences` | Exercices + Leçon |
| Proportionnalité | 3 | `cm2-proportion` | Exercices + Leçon |
| Résoudre des problèmes | 1, 5 | `cm2-problems-multi-step`, `cm2-problems-optimization` | Exercices + Leçon |

Les périodes suivent le programme officiel quand il les précise ; sinon ce sont des choix d'Educator, signalés dans le
champ `source` de chaque compétence.

## Évolutions du moteur et des écrans portées par les fiches du lot

- **Les grands nombres jusqu'au milliard** — Conversion nombre → français écrit jusqu'au milliard, orthographe traditionnelle (« millions » au pluriel, « mille » invariable) ; graphie rectifiée de 1990 acceptée en saisie.
- **Multiples, diviseurs et diviseurs communs** — Format `choice` à plusieurs bonnes réponses (lot 1, CE1) ; validation : toutes les bonnes et aucune fausse, retour détaillé.
- **Opérations posées, estimations et parenthèses** — Format `expression` (lot 5) étendu à deux paires de parenthèses.
- **La division posée d'un entier ou d'un décimal** — Division décimale sur le type décimal exact : le générateur ne choisit que des divisions qui tombent juste (au plus deux décimales) ; potence avec virgule au quotient au moment où l'on abaisse les dixièmes.
- **Les fractions : écrire, placer et comparer** — Dénominateurs jusqu'à 60 et fractions décimales en millièmes dans le type fraction.
- **Calculer avec les fractions** — Réponse fraction : toute fraction égale acceptée, sans simplification exigée (comme le précise le livret).
- **Les nombres décimaux jusqu'aux millièmes** — Type décimal étendu aux millièmes ; lecture Piper des millièmes.
- **Égalités à trous, programmes de calcul et suites** — Format `program` étendu au programme inverse ; `sequence` avec question à l'étape n.
- **Proportionnalité** — Gabarits de proportionnalité à deux étapes ; le générateur choisit des nombres qui évitent le passage par l'unité.
- **Résoudre des problèmes** — Gabarits de problèmes avec décimaux et fractions ; problèmes « algorithmes » sur grille (réutilise `arrayGrid`).

Ces évolutions reprennent les fiches mathématiques de l'ancien lot 4 (SUB001601 à SUB001606, SUB001612), qui décrivaient le même
besoin pour toutes les classes. L'ancien lot 4 est clos (Done) : ses fiches sont remplacées par les lots 4, 5 et 6, un par niveau (décision de Georges du 2026-10-12).

## Patte graphique

Les leçons utilisent uniquement la « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » : charte « S1 · Grand Prix »
(SUB000105), voiture sur la piste graduée, damier, rouge course pour ce qui est nouveau. Un exercice qui affiche une
aide visuelle (droite graduée, bande de fraction, opération posée) réutilise les mêmes composants.

## Orthographe et grammaire

Le lot porte aussi l'orthographe et la grammaire de la classe (reprise de l'ancien lot 4) : fiches « Exercices CM2 — Orthographe et grammaire » et « Leçons CM2 — Orthographe et grammaire ».

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] Le repli « Les exercices de ta classe arrivent bientôt » (SUB000108) ne s'affiche plus pour le CM2 : la classe a ses propres compétences sur les 5 périodes.
- [ ] Les compétences de la période 5 de la classe précédente apparaissent en « Révisions » en période 1 (SUB000113).
- [ ] Chaque compétence du CM2 a une leçon, accessible depuis l'écran Leçons et depuis la liste des exercices.
- [ ] Toutes les compétences sont vérifiées contre le programme en vigueur, `source` renseignée.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
