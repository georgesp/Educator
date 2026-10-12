---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYvI
url: https://github.com/georgesp/Educator/issues/119
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: US0119 — Lot 4 — CE2 — exercices et leçons
status: Todo
size: XL
estimate:
startDate:
endDate:
parent:
order: 1300
---

Source : demande de Georges du 2026-10-11 — ajouter au lot 1 les exercices et les leçons de mathématiques de chaque classe.

- **Lot** : 4 — CE2 — exercices et leçons (mathématiques, orthographe et grammaire)
- **Prérequis** : fiche « Lot 1 — Socle mathématiques » (exercices et leçons du CE1, dont numération, multiplication, fractions et problèmes ; bibliothèque d'animations ; format et lecteur de leçons).
- **Lot suivant** : fiche « Lot 5 — CM1 — exercices et leçons ».

## Objectif

Donner au CE2 ses propres exercices et leçons de mathématiques sur les cinq périodes, conformément au programme du cycle 2 de 2024 : nombres jusqu'à 10 000, calcul mental, opérations posées, tables et multiplication posée, sens de la division, premières fractions et résolution de problèmes. Aujourd'hui, un enfant de CE2 reçoit les exercices du CE1.

Chaque thème a **deux fiches** : une fiche « Exercices » (compétences JSON, générateur, écran de série) et une fiche « Leçon »
(8 à 12 slides, 3 à 5 minutes, animations dans la charte Grand Prix, quiz).

## Thèmes

| Thème | Périodes | Compétences | Fiches |
|---|---|---|---|
| Nombres jusqu'à 10 000 | 1, 2 | `ce2-num-read-write`, `ce2-num-decompose` | Exercices + Leçon |
| Comparer, ranger et encadrer jusqu'à 10 000 | 2, 3 | `ce2-num-compare`, `ce2-num-frame` | Exercices + Leçon |
| Ajouter 8, 9, 18, 19, 28, 29, 38 ou 39 | 1 | `ce2-add-near-tens` | Exercices + Leçon |
| Soustraire 9, 19, 29 ou 39 | 2 | `ce2-sub-near-tens` | Exercices + Leçon |
| Additions et soustractions posées jusqu'à 10 000 | 1, 2, 5 | `ce2-add-4-digits`, `ce2-sub-4-digits`, `ce2-money-add-sub` | Exercices + Leçon |
| Tables de multiplication de 2, 3, 4, 5 et 10 | 1 | `ce2-mul-tables-2-5` | Exercices + Leçon |
| Tables de multiplication de 6, 7, 8 et 9 | 3, 4 | `ce2-mul-tables-6-9`, `ce2-mul-tables-all` | Exercices + Leçon |
| Doubles, moitiés et faits multiplicatifs usuels | 2, 3 | `ce2-double-half`, `ce2-mul-facts-25-60` | Exercices + Leçon |
| Multiplier par 10 et par 100 | 2 | `ce2-mul-10-100` | Exercices + Leçon |
| Multiplier par des dizaines, par 4 et par 8 | 3 | `ce2-mul-by-tens`, `ce2-mul-by-4-8` | Exercices + Leçon |
| Multiplier en décomposant | 3 | `ce2-mul-distributive` | Exercices + Leçon |
| Multiplication posée | 4, 5 | `ce2-mul-column-1-digit`, `ce2-mul-column-2-digits` | Exercices + Leçon |
| Comprendre la division | 4, 5 | `ce2-div-tables`, `ce2-div-inverse` | Exercices + Leçon |
| Les fractions : partager l'unité | 3 | `ce2-frac-read-write`, `ce2-frac-equal` | Exercices + Leçon |
| Comparer, ajouter et soustraire des fractions | 4, 5 | `ce2-frac-compare`, `ce2-frac-add-sub` | Exercices + Leçon |
| Résoudre des problèmes | 1, 3, 5 | `ce2-problems-additive`, `ce2-problems-multiplicative`, `ce2-problems-mixed` | Exercices + Leçon |

Les périodes suivent le programme officiel quand il les précise ; sinon ce sont des choix d'Educator, signalés dans le
champ `source` de chaque compétence.

## Évolutions du moteur et des écrans portées par les fiches du lot

- **Nombres jusqu'à 10 000** — Formats de numération (`writeNumber`, `readNumber`, `decompose`, `recompose`, `placeValue`) créés au lot 1 (fiche « Exercices CE1 — Les nombres jusqu'à 1 000 ») et étendus jusqu'à 10 000.
- **Nombres jusqu'à 10 000** — Conversion nombre → français écrit (lot 1, CE1) étendue jusqu'à 10 000, testée sur toute la plage (cas pièges : 71, 80, 81, 200, 201, 280, 1 000) ; réutilisée par la voix. Une fonction de comparaison accepte aussi la graphie rectifiée de 1990.
- **Nombres jusqu'à 10 000** — Saisie au pavé numérique avec espace des milliers affiché automatiquement (4 635).
- **Comparer, ranger et encadrer jusqu'à 10 000** — Formats `compare`, `order`, `frame` et `numberLinePoint` créés au lot 1 (fiche « Exercices CE1 — Comparer, ranger et placer les nombres jusqu'à 1 000 »), plages étendues jusqu'à 10 000 et graduations de 1 000 en 1 000.
- **Comparer, ranger et encadrer jusqu'à 10 000** — Le générateur choisit des nombres proches (mêmes chiffres de tête) pour que la comparaison porte sur les bons rangs.
- **Additions et soustractions posées jusqu'à 10 000** — Composant d'**opération posée** dans l'écran de série : l'enfant écrit le résultat chiffre par chiffre, de droite à gauche, avec les retenues (repris de SUB001603) ; correction ciblée sur la colonne fausse.
- **Additions et soustractions posées jusqu'à 10 000** — Prix : montants stockés en centimes (entiers) et affichés avec la virgule et « € » ; saisie avec la virgule. Le type décimal du lot 5 pourra les reprendre sans changer les exercices.
- **Additions et soustractions posées jusqu'à 10 000** — Champ `display: "column"` dans la compétence pour demander l'affichage posé.
- **Additions et soustractions posées jusqu'à 10 000** — Soustraction posée selon **les deux techniques** : cassage (chiffres barrés, dizaine cassée) ou compensation (10 unités ajoutées en haut, 1 dizaine en bas). La technique suit le réglage du profil (choisie à la première soustraction posée, modifiable dans les Options) ; le résultat est juste quelle que soit la technique, seules les marques d'aide changent.
- **Doubles, moitiés et faits multiplicatifs usuels** — Format `phrase` (lot 1, CE1) : consigne en mots générée (« Quel est le double de 17 ? »).
- **Doubles, moitiés et faits multiplicatifs usuels** — Liste des nombres usuels du programme dans le fichier de la compétence (`operands[].values`).
- **Multiplier en décomposant** — Après une erreur, l'aide affiche la décomposition (10 × 7 + 3 × 7) avec l'animation `arrayGrid` (lien avec la difficulté adaptative du lot 3).
- **Multiplication posée** — Composant d'opération posée étendu à la multiplication : lignes de produits partiels, zéro (ou point) de décalage, retenues, somme finale ; saisie chiffre par chiffre et correction ciblée (SUB001603).
- **Comprendre la division** — Opération `division` (exacte seulement au CE2 : a = b × q, jamais de division par 0) — reprise de SUB001604 ; symbole « ÷ » lu « divisé par ».
- **Comprendre la division** — Format `phrase` avec petits énoncés de partage et de groupement (gabarits, voir la fiche « Exercices CE2 — Résoudre des problèmes »).
- **Les fractions : partager l'unité** — Type **fraction**, saisie et lecture Piper créés au lot 1 (fiche « Exercices CE1 — Les premières fractions ») ; étendus aux dénominateurs jusqu'à 12 (septièmes, neuvièmes, onzièmes, douzièmes) et aux égalités de fractions.
- **Les fractions : partager l'unité** — Réponse fraction : **toute fraction égale est acceptée** (2/4 pour 1/2) ; le retour montre la forme attendue.
- **Comparer, ajouter et soustraire des fractions** — Format `compare` étendu aux fractions ; réponse sous forme de fraction : toute fraction égale est acceptée (6/10 ou 3/5), le retour affiche la forme attendue.
- **Résoudre des problèmes** — Format `problem` et gabarits créés au lot 1 (fiche « Exercices CE1 — Résoudre des problèmes ») ; nouveaux gabarits : comparaison multiplicative, produits cartésiens, problèmes en trois étapes ; réponse avec unité affichée.

Ces évolutions reprennent les fiches mathématiques de l'ancien lot 4 (SUB001601 à SUB001606, SUB001612), qui décrivaient le même
besoin pour toutes les classes. L'ancien lot 4 est clos (Done) : ses fiches sont remplacées par les lots 4, 5 et 6, un par niveau (décision de Georges du 2026-10-12).

## Patte graphique

Les leçons utilisent uniquement la « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » : charte « S1 · Grand Prix »
(SUB000105), voiture sur la piste graduée, damier, rouge course pour ce qui est nouveau. Un exercice qui affiche une
aide visuelle (droite graduée, bande de fraction, opération posée) réutilise les mêmes composants.

## Orthographe et grammaire

Le lot porte aussi l'orthographe et la grammaire de la classe (reprise de l'ancien lot 4) : fiches « Exercices CE2 — Orthographe et grammaire » et « Leçons CE2 — Orthographe et grammaire ».

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] Le repli « Les exercices de ta classe arrivent bientôt » (SUB000108) ne s'affiche plus pour le CE2 : la classe a ses propres compétences sur les 5 périodes.
- [ ] Les compétences de la période 5 de la classe précédente apparaissent en « Révisions » en période 1 (SUB000113).
- [ ] Chaque compétence du CE2 a une leçon, accessible depuis l'écran Leçons et depuis la liste des exercices.
- [ ] Toutes les compétences sont vérifiées contre le programme en vigueur, `source` renseignée.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
