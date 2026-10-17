---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACY04
rev: 2026-10-17T13:46:51Z
url: https://github.com/georgesp/Educator/issues/122
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: US0122 — Lot 7 — CP — exercices et leçons de mathématiques
status: In progress
size: XL
estimate:
startDate:
endDate:
parent:
order: 1910
---

Source : demande de Georges du 2026-10-11 — ajouter au lot 1 les exercices et les leçons de mathématiques de chaque classe ;
décision du 2026-10-12 (un lot par niveau) ; revue du 2026-10-17 (conformité au programme de 2024, grandeurs et mesures,
voix au clic et iPad, comme le lot 4).

- **Lot** : 7 — CP — exercices et leçons (mathématiques, grandeurs et mesures)
- **Prérequis** : lot 1 (formats de numération, comparaison, `phrase`, `problem` créés pour le CE1, bibliothèque
  d'animations, format et lecteur de leçons), lot 2 (quiz, examens, défi chrono), lot 2bis (voix en direct), lot 3
  (difficulté adaptative, révisions, badges, code couleur). Évolutions partagées avec les lots 4 et 5 : voir
  « Ordre de réalisation ».
- **Lot suivant** : fiche « Lot packaging — Installeurs, signature Apple et tests Windows ».

## Objectif

Compléter le CP, qui n'a aujourd'hui que des compétences d'addition et de soustraction (lot 1), avec les autres notions
du programme du cycle 2 de 2024 :

- **mathématiques** : numération jusqu'à 100, comparaison, suites de nombres, ordinaux et suites répétitives, doubles et
  moitiés, addition posée, découverte de la multiplication et du partage, résolution de problèmes ;
- **grandeurs et mesures** (décision de Georges du 2026-10-16, étendue au CP par la revue du 2026-10-17) : longueurs,
  masses, monnaie, heures entières.

Les leçons des 11 compétences existantes du CP ont été rédigées au lot 1 et sont rattachées à ce lot (SUB000140 à
SUB000150, en relecture : il reste la durée mesurée avec la voix Piper et la relecture de Georges).

Chaque thème a une fiche « Exercices » (compétences JSON, générateur, écran de série) et une ou deux fiches « Leçon »
(8 à 12 slides, 3 à 5 minutes, animations dans la charte Grand Prix, quiz).

## Thèmes — mathématiques

| Thème | Périodes | Compétences | Fiches |
|---|---|---|---|
| Les nombres jusqu'à 100 : dizaines et unités | 2, 3, 5 | `cp-num-to-59`, `cp-num-to-100`, `cp-num-words-50` | Exercices + 2 leçons (jusqu'à 59 ; de 60 à 100 et en lettres) |
| Comparer, ranger et placer les nombres jusqu'à 100 | 3, 4 | `cp-num-compare`, `cp-num-line` | Exercices + Leçon |
| Compter de 1 en 1, de 2 en 2, de 10 en 10 et à rebours | 1, 2 | `cp-count-1-2`, `cp-count-10-back`, `cp-ordinals` (ordinaux et suites répétitives) | Exercices + Leçon |
| Doubles et moitiés | 3, 5 | `cp-doubles-halves`, `cp-half-even` | Exercices + Leçon |
| L'addition posée | 5 | `cp-add-column` | Exercices + Leçon |
| Découvrir la multiplication : « fois » | 4 | `cp-times-as-repeated-add` | Exercices + Leçon |
| Partager et faire des groupes | 5 | `cp-share-group` | Exercices + Leçon |
| Résoudre des problèmes | 2, 3, 5 | `cp-problems-one-step`, `cp-problems-one-step-100`, `cp-problems-two-steps`, `cp-problems-mixed` | Exercices + 2 leçons (une étape ; deux étapes et problèmes variés) |

## Thèmes — grandeurs et mesures

| Thème | Périodes | Compétences | Fiches |
|---|---|---|---|
| Longueurs et masses | 3, 4, 5 | `cp-length-compare-measure`, `cp-mass-compare`, `cp-length-m-cm` | Exercices + Leçon |
| La monnaie : les euros | 4, 5 | `cp-money-value`, `cp-money-make` | Exercices + Leçon |
| Lire l'heure : les heures entières | 3, 4 | `cp-clock-read`, `cp-clock-set` | Exercices + Leçon |

Les périodes suivent le programme officiel quand il les précise ; sinon ce sont des choix d'Educator, signalés dans le
champ `source` de chaque compétence. Le programme ne donne pas de période pour les grandeurs et mesures du CP.

## Évolutions du moteur et des écrans portées par les fiches du lot

- **Les nombres jusqu'à 100 : dizaines et unités** — Famille **numération** (lot 1, CE1) avec une **image** dans l'exercice : barres de dix et cubes dessinés (réutilise `tensUnits`) ; le champ `picture` des compétences (aujourd'hui `arrayGrid` seulement) s'ouvre à `tensUnits` et `groupObjects`.
- **Les nombres jusqu'à 100 : dizaines et unités** — Dictée de nombres : le nombre est lu au clic sur « Écouter le nombre », jamais tout seul (décision du 2026-10-15) ; réécoute illimitée.
- **Comparer, ranger et placer les nombres jusqu'à 100** — Formats `compare`, `order` et `numberLinePoint` (lot 1, CE1), plages CP, tailles de boutons pour le CP.
- **Compter de 1 en 1, de 2 en 2, de 10 en 10 et à rebours** — Format `sequence` (lot 5) disponible au CP : suites courtes de nombres, cases vides au milieu ou à la fin ; suites répétitives de symboles, de formes ou de lettres.
- **Compter de 1 en 1, de 2 en 2, de 10 en 10 et à rebours** — Format `choice` sur image : une file de voitures dessinées, l'enfant clique sur celle demandée ; ordinaux lus au clic (« sixième »).
- **Doubles et moitiés** — Format `phrase` (lot 1, CE1) avec des consignes adaptées au CP, lues au clic.
- **L'addition posée** — Composant d'opération posée (lot 4) dans une version CP : grandes cases, une colonne à la fois, retenue pré-dessinée en pointillé.
- **Découvrir la multiplication : « fois »** — Format `phrase` : « 3 fois 20 » affiché en mots et en addition répétée, sans le signe × (introduit plus tard).
- **Découvrir la multiplication : « fois »** — Image de paquets d'objets dans l'exercice (réutilise `groupObjects`).
- **Partager et faire des groupes** — Format `phrase` avec gabarits de partage et de groupement, illustrés par `groupObjects` (pas de signe ÷ au CP).
- **Résoudre des problèmes** — Format `problem` (lot 1, CE1) adapté au CP : énoncé lu au clic sur « Écouter l'histoire » (bouton toujours visible), phrases courtes, image de la situation, réponse au pavé numérique ; gabarits « sans cassage de dizaine » pour la période 2, inconnue sur l'état initial, sortes `product` et `sharing` (≤ 30) et mélange des sortes ; énoncés qui n'induisent pas l'opération.
- **Longueurs et masses** — Type « mesure », formats `measureRead`, `estimate` et `compare` étendu (fiche « Exercices CE2 — Longueurs ») dans une version CP : mètre et centimètre seulement, règle graduée sans millimètres, balance à plateaux sans masses marquées.
- **La monnaie : les euros** — Composant `coins` et format `coinsMake` (fiche « Exercices CE2 — Payer et rendre la monnaie ») limités aux euros entiers : pièces de 1 € et 2 €, billets de 5 € à 50 €.
- **Lire l'heure : les heures entières** — Composant `clock`, formats `clockSet` et `clockMatch` (fiche « Exercices CE2 — Lire l'heure et calculer des durées ») limités aux heures entières de 1 h à 12 h.

Ces évolutions reprennent les fiches mathématiques de l'ancien lot 4 (SUB001601 à SUB001606, SUB001612), qui décrivaient le même
besoin pour toutes les classes. L'ancien lot 4 est clos (Done) : ses fiches sont remplacées par les lots 4, 5 et 6, un par niveau (décision de Georges du 2026-10-12).

## Ordre de réalisation

Plusieurs fiches du lot réutilisent des évolutions décrites dans des lots qui ne sont pas encore faits :

| Évolution | Décrite dans | Utilisée par |
|---|---|---|
| Opération posée dans l'exercice (`columnOperation`) | SUB011901 (lot 4, CE2) | Exercices CP — L'addition posée |
| Format `sequence` | SUB012007 (lot 5, CM1) | Exercices CP — Compter de 1 en 1, de 2 en 2, de 10 en 10 et à rebours |
| Type « mesure », `measureRead`, `estimate`, famille « Grandeurs et mesures », voix des unités | SUB011940 (lot 4) | Exercices CP — Longueurs et masses, La monnaie, Lire l'heure |
| Animations `ruler`, `scale`, `coins`, `clock` | SUB011946 (lot 4) | Leçons CP de grandeurs et mesures |
| Format `coinsMake` | SUB011942 (lot 4) | Exercices CP — La monnaie : les euros |
| Formats `clockSet`, `clockMatch` | SUB011937 (lot 4) | Exercices CP — Lire l'heure : les heures entières |

Règle : le premier lot réalisé porte l'évolution (code, schéma, tests) ; l'autre la réutilise et n'ajoute que ses plages
et ses réglages. Les thèmes qui n'en dépendent pas (nombres, comparaison, doubles et moitiés, « fois », partages,
problèmes) peuvent démarrer tout de suite.

## Patte graphique

Les leçons utilisent uniquement la « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » : charte « S1 · Grand Prix »
(SUB000105), voiture sur la piste graduée, damier, rouge course pour ce qui est nouveau. Un exercice qui affiche une
aide visuelle (droite graduée, opération posée, règle, balance, horloge, pièces) réutilise les mêmes composants.

## Hors périmètre du lot 7

- **Espace et géométrie** (repérage dans l'espace, solides, figures planes) et **organisation et gestion de données**
  (tableaux, diagrammes en barres) : au programme du CP, mais hors du champ d'Educator (calcul, mesures et français),
  comme au CE2 (lot 4).
- **Masses** : le CP compare des masses sans unité ; le gramme et le kilogramme ne sont pas au programme du CP.
- **Monnaie** : euros entiers seulement (choix d'Educator) ; les centimes viennent dans les classes suivantes.
- **Heure** : heures entières seulement, comme le programme ; demi-heures, quarts et minutes viennent ensuite.

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] Les nouvelles compétences s'ajoutent aux 11 compétences du CP du lot 1, triées par période, sans les modifier.
- [ ] Les compétences de la période 5 du CP restent proposées en « Révisions » au CE1 (SUB000113).
- [ ] Chaque compétence du CP a une leçon, accessible depuis l'écran Leçons et depuis la liste des exercices.
- [ ] Toutes les compétences sont vérifiées contre le programme en vigueur, `source` renseignée ; les attendus de grandeurs et mesures sont vérifiés sur le PDF officiel.
- [ ] L'examen et l'évaluation de niveau du CP intègrent les nouvelles compétences (questions jouables sans écoute seulement, comme aujourd'hui).
- [ ] Défi chrono du CP : seules les compétences de faits numériques et de calcul mental y entrent (pas l'addition posée, les problèmes, les mesures ni l'horloge à placer) ; les attendus de fluence de fin de CP sont notés dans la `source` des compétences concernées : 8 égalités à trou par minute (tables d'addition), 8 doubles ou moitiés par minute, 9 calculs en 3 minutes (calcul mental).
- [ ] Chaque nouvelle compétence est rattachée à une catégorie des badges (`numbers`, `problems`…) et à sa famille du code couleur (SUB001514), dont « Grandeurs et mesures ».
- [ ] Tout est jouable au doigt sur iPad (ADR 0006) et rien n'est lu tout seul (décision du 2026-10-15).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Résumé par niveau du programme de mathématiques du cycle 2 (2024) — attendus de grandeurs et mesures du CP, à vérifier sur le PDF officiel](https://maitresselunicole.fr/wp-content/uploads/2025/03/resume-par-niveaux-programme-de-mathematiques-cycle-2-2024-1.pdf)
