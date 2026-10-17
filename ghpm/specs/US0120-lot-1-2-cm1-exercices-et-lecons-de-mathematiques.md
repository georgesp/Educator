---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYw0
rev: 2026-10-12T19:26:27Z
url: https://github.com/georgesp/Educator/issues/120
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: US0120 — Lot 5 — CM1 — exercices et leçons
status: Backlog
size: XL
estimate:
startDate:
endDate:
parent:
order: 1270
---

Source : demande de Georges du 2026-10-11 — ajouter au lot 1 les exercices et les leçons de mathématiques de chaque classe.

- **Lot** : 5 — CM1 — exercices et leçons (mathématiques, orthographe et grammaire)
- **Prérequis** : fiche « Lot 4 — CE2 — exercices et leçons » (multiplication, division, fractions dans le moteur).
- **Lot suivant** : fiche « Lot 6 — CM2 — exercices et leçons ».

## Objectif

Donner au CM1 ses propres exercices et leçons de mathématiques sur les cinq périodes, conformément au programme du cycle 3 de 2025 (en vigueur au CM1 depuis la rentrée 2025) : grands nombres jusqu'à 999 999, multiples et diviseurs, calcul mental, opérations posées et division euclidienne, fractions (y compris supérieures à 1), nombres décimaux jusqu'aux centièmes, premières notions d'algèbre, proportionnalité et résolution de problèmes. S'y ajoutent les grandeurs et mesures qui se calculent (durées, longueurs, masses, contenances, périmètres, aires), les probabilités et la lecture de données (tableaux, diagrammes).

Chaque thème a **deux fiches** : une fiche « Exercices » (compétences JSON, générateur, écran de série) et une fiche « Leçon »
(8 à 12 slides, 3 à 5 minutes, animations dans la charte Grand Prix, quiz).

## Thèmes

| Thème | Périodes | Compétences | Fiches |
|---|---|---|---|
| Les grands nombres jusqu'à 999 999 | 3 | `cm1-num-read-write`, `cm1-num-decompose` | Exercices + Leçon |
| Comparer, ranger et encadrer les grands nombres | 4 | `cm1-num-compare`, `cm1-num-frame` | Exercices + Leçon |
| Multiples, diviseurs et critères de divisibilité | 2, 3 | `cm1-divisibility`, `cm1-multiples-divisors` | Exercices + Leçon |
| Ajouter et soustraire de tête des nombres proches d'une dizaine | 1 | `cm1-add-sub-near-tens` | Exercices + Leçon |
| Multiplier par 10, 100, 1 000 et par des dizaines ou des centaines | 1, 2 | `cm1-mul-10-100-1000`, `cm1-mul-tens-hundreds` | Exercices + Leçon |
| Multiplier par 4, par 8 et par 5 | 2, 3 | `cm1-mul-by-4-8`, `cm1-mul-by-5` | Exercices + Leçon |
| Distributivité et calculs avec parenthèses | 4 | `cm1-distributive`, `cm1-parentheses` | Exercices + Leçon |
| Opérations posées sur les entiers | 1, 2 | `cm1-add-sub-column`, `cm1-mul-column`, `cm1-estimate` | Exercices + Leçon |
| La division euclidienne | 3, 4 | `cm1-div-remainder`, `cm1-div-column` | Exercices + Leçon |
| Les fractions : écrire, placer et dépasser 1 | 1, 2 | `cm1-frac-read-place`, `cm1-frac-integer-part` | Exercices + Leçon |
| Fractions : comparer, additionner et prendre une fraction d'une quantité | 2, 3 | `cm1-frac-compare-add`, `cm1-frac-of-quantity` | Exercices + Leçon |
| Les nombres décimaux : dixièmes et centièmes | 2, 3 | `cm1-dec-money`, `cm1-dec-fractions`, `cm1-dec-usual-fractions` | Exercices + Leçon |
| Comparer, ranger et arrondir les nombres décimaux | 3 | `cm1-dec-compare`, `cm1-dec-round` | Exercices + Leçon |
| Calculer avec les nombres décimaux | 4, 5 | `cm1-dec-add-sub`, `cm1-dec-mental`, `cm1-dec-mul-div-10`, `cm1-dec-mul-int` | Exercices + Leçon |
| Égalités à trous, programmes de calcul et suites | 2, 3 | `cm1-algebra-missing`, `cm1-calc-program` | Exercices + Leçon |
| La proportionnalité | 5 | `cm1-proportion` | Exercices + Leçon |
| Lire l'heure et calculer des durées | 1, 3 | `cm1-time-read`, `cm1-time-duration` | Exercices + Leçon |
| Longueurs, masses et contenances | 5 | `cm1-measure-units`, `cm1-measure-convert` | Exercices + Leçon |
| Périmètres et aires | 4, 5 | `cm1-perimeter`, `cm1-area-grid` | Exercices + Leçon |
| Probabilités | 2, 4 | `cm1-proba-events`, `cm1-proba-scale` | Exercices + Leçon |
| Lire et utiliser des données | 3, 5 | `cm1-data-read`, `cm1-data-problems` | Exercices + Leçon |
| Résoudre des problèmes | 1, 2, 4 | `cm1-problems-multi-step`, `cm1-problems-multiplicative`, `cm1-problems-counting` | Exercices + Leçon |

Les périodes suivent le programme officiel quand il les précise ; sinon ce sont des choix d'Educator, signalés dans le
champ `source` de chaque compétence.

## Taille des nombres selon la période

Le programme limite les périodes 1 et 2 aux nombres entiers d'au plus quatre chiffres ; les nombres de cinq ou six
chiffres arrivent à partir de la période 3 ou du début de la période 4. Toute compétence du CM1 ouverte en période 1
ou 2 a donc deux plages : jusqu'à 9 999 en périodes 1 et 2, plus grande à partir de la période 3 (mécanisme de
difficulté par période de SUB000112 ; à défaut, deux compétences). Les exemples des leçons de période 1 et 2
respectent la même limite.

## Hors périmètre

- Espace et géométrie (figures, constructions aux instruments, symétrie, solides et patrons, repérage) et angles :
  Educator porte sur le calcul, et la construction aux instruments ne se fait pas à l'écran.
- Initiation à la pensée informatique : pas d'objectif propre au CM1 dans le programme.

## Évolutions du moteur et des écrans portées par les fiches du lot

- **Les grands nombres jusqu'à 999 999** — Formats de numération du CE2 étendus jusqu'au million ; conversion nombre → français écrit jusqu'au million (SUB001612), orthographe traditionnelle, graphie rectifiée de 1990 acceptée en saisie.
- **Comparer, ranger et encadrer les grands nombres** — Formats `compare`, `order`, `frame`, `numberLinePoint` du CE2, plages étendues.
- **Multiples, diviseurs et critères de divisibilité** — Formats `yesNo` et `choice` à plusieurs bonnes réponses créés au lot 1 (fiche « Exercices CE1 — Pair, impair, doubles et moitiés »).
- **Multiplier par 4, par 8 et par 5** — Règle d'exclusion de valeurs (`excludeValues`) dans la compétence pour écarter les faits connus, comme le préconise le livret.
- **Distributivité et calculs avec parenthèses** — Format `expression` : le générateur produit une expression avec parenthèses (arbre de calcul dans `educator-core`) et sa valeur ; affichage et lecture Piper (« ouvrez la parenthèse » non lu, structure annoncée « la somme de 12 et 8, multipliée par 5 »).
- **Opérations posées sur les entiers** — Composant d'opération posée (lot 4) : multiplication à trois lignes de produits partiels ; soustraction selon la technique du profil (cassage ou compensation).
- **La division euclidienne** — Opération `division` avec reste (SUB001604) : saisie du quotient et du reste dans deux champs ; propriété testée : dividende = diviseur × quotient + reste, reste < diviseur, jamais de division par 0.
- **La division euclidienne** — Composant d'opération posée étendu à la division (potence, soustractions successives).
- **Les fractions : écrire, placer et dépasser 1** — Fractions supérieures à 1 et dénominateurs jusqu'à 20 dans le type fraction (lot 1, CE1) ; `numberLinePoint` gradué en fractions.
- **Fractions : comparer, additionner et prendre une fraction d'une quantité** — Réponse fraction : toute fraction égale acceptée (simplification non attendue), forme attendue rappelée dans le retour.
- **Les nombres décimaux : dixièmes et centièmes** — Type **décimal** exact dans le cœur (entier + nombre de décimales, pas de flottant) — repris de SUB001605.
- **Les nombres décimaux : dixièmes et centièmes** — Saisie avec la virgule française ; le point est accepté et converti ; zéros inutiles tolérés (3,50 = 3,5).
- **Les nombres décimaux : dixièmes et centièmes** — Lecture Piper des décimaux (« trois virgule quarante-cinq », ou « 3 euros 45 » dans un contexte de prix).
- **Comparer, ranger et arrondir les nombres décimaux** — Formats `compare`, `order`, `frame` pour les décimaux ; le générateur produit des paires pièges (plus de chiffres ≠ plus grand).
- **Calculer avec les nombres décimaux** — Opérations sur le type décimal exact ; composant d'opération posée avec virgules alignées (et zéros ajoutés à droite affichés en gris).
- **Égalités à trous, programmes de calcul et suites** — Formats `program` (étapes affichées et lues) et `sequence` (suite de nombres ou de motifs SVG simples) générés par règles.
- **La proportionnalité** — Format `proportionTable` : tableau à compléter dans l'exercice (réutilise l'animation `proportionTable`) ; gabarits de problèmes.
- **Résoudre des problèmes** — Gabarits de problèmes (lot 1, CE1) étendus : décimaux (prix), restes, plusieurs questions dans un énoncé.
- **Taille des nombres** — Plage de nombres selon la période dans les compétences ouvertes en périodes 1 et 2 (≤ 9 999), test du contenu qui le vérifie.
- **Opérations posées sur les entiers** — Format `choice` d'ordre de grandeur (`cm1-estimate`) : distracteurs 10 fois plus petit et 10 fois plus grand.
- **Comparer, ranger et arrondir les nombres décimaux** — `numberLinePoint` gradué en dixièmes et centièmes ; format `between` (intercaler : toute réponse strictement entre les deux bornes, écrite au plus aux centièmes, est acceptée).
- **Égalités à trous, programmes de calcul et suites** — Affichage `balance` (deux plateaux, boîtes identiques et poids connus), comme dans la séquence 2 du livret.
- **Lire l'heure et calculer des durées** — Horloge à aiguilles SVG (lecture et aiguilles déplaçables), réponse horaire / durée en heures et minutes.
- **Longueurs, masses et contenances** — Unités dans la réponse, tableau de conversion en aide, lecture Piper des unités en toutes lettres.
- **Périmètres et aires** — Figures SVG générées : polygone coté, figure sur quadrillage (carreaux et demi-carreaux).
- **Probabilités** — Illustrations générées (sac de billes, dé, roue) et format `probabilityScale` (glisser sur l'échelle impossible → certain).
- **Lire et utiliser des données** — Tableaux, diagrammes en barres et courbes générés à partir des données de l'exercice.

Ces évolutions reprennent les fiches mathématiques de l'ancien lot 4 (SUB001601 à SUB001606, SUB001612), qui décrivaient le même
besoin pour toutes les classes. L'ancien lot 4 est clos (Done) : ses fiches sont remplacées par les lots 4, 5 et 6, un par niveau (décision de Georges du 2026-10-12).

## Patte graphique

Les leçons utilisent uniquement la « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » : charte « S1 · Grand Prix »
(SUB000105), voiture sur la piste graduée, damier, rouge course pour ce qui est nouveau. Un exercice qui affiche une
aide visuelle (droite graduée, bande de fraction, opération posée) réutilise les mêmes composants.

Les nouveaux thèmes (heure, mesures, figures, probabilités, données) demandent de nouvelles animations : elles sont
portées par la fiche « Animations des leçons CM1 — heures, mesures, probabilités et données » et rejoignent la même bibliothèque.

## Orthographe et grammaire

Le lot porte aussi l'orthographe et la grammaire de la classe (reprise de l'ancien lot 4) : fiches « Exercices CM1 — Orthographe et grammaire » et « Leçons CM1 — Orthographe et grammaire ».

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] Le repli « Les exercices de ta classe arrivent bientôt » (SUB000108) ne s'affiche plus pour le CM1 : la classe a ses propres compétences sur les 5 périodes.
- [ ] Les compétences de la période 5 de la classe précédente apparaissent en « Révisions » en période 1 (SUB000113).
- [ ] Chaque compétence du CM1 a une leçon, accessible depuis l'écran Leçons et depuis la liste des exercices.
- [ ] Toutes les compétences sont vérifiées contre le programme en vigueur, `source` renseignée.
- [ ] En périodes 1 et 2, aucun exercice ni exemple de leçon du CM1 n'utilise de nombre entier de plus de quatre chiffres (test du contenu).
- [ ] Les thèmes hors périmètre sont listés dans la section « Hors périmètre ».

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Programme de français du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025)](https://www.education.gouv.fr/sites/default/files/programme-de-fran-ais-pour-le-cycle-3-439824.pdf)
