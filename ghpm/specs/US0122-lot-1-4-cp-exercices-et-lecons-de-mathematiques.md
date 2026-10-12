---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACY04
url: https://github.com/georgesp/Educator/issues/122
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: US0122 — Lot 1.4 — CP — exercices et leçons de mathématiques
status: Todo
size: XL
estimate:
startDate:
endDate:
parent:
order: 2500
---

Source : demande de Georges du 2026-10-11 — ajouter au lot 1 les exercices et les leçons de mathématiques de chaque classe.

- **Lot** : 1.4 — CP — exercices et leçons de mathématiques
- **Prérequis** : fiche « Lot 1 — Socle mathématiques » (formats de numération, comparaison, `phrase`, `problem` créés pour le CE1), le composant d'opération posée du lot 4 et le format `sequence` du lot 5 ; les thèmes qui n'en dépendent pas peuvent démarrer plus tôt.
- **Lot suivant** : fiche « Lot 2 — Orthographe, leçons et examens ».

## Objectif

Compléter le CP, qui n'a aujourd'hui que des compétences d'addition et de soustraction (lot 1), avec les autres notions du programme du cycle 2 de 2024 : numération jusqu'à 100, comparaison, suites de nombres, doubles et moitiés, addition posée, découverte de la multiplication et du partage, et résolution de problèmes. Les leçons des 11 compétences existantes du CP sont dans le lot 1.

Chaque thème a **deux fiches** : une fiche « Exercices » (compétences JSON, générateur, écran de série) et une fiche « Leçon »
(8 à 12 slides, 3 à 5 minutes, animations dans la charte Grand Prix, quiz).

## Thèmes

| Thème | Périodes | Compétences | Fiches |
|---|---|---|---|
| Les nombres jusqu'à 100 : dizaines et unités | 2, 3, 5 | `cp-num-to-59`, `cp-num-to-100`, `cp-num-words-50` | Exercices + Leçon |
| Comparer, ranger et placer les nombres jusqu'à 100 | 3, 4 | `cp-num-compare`, `cp-num-line` | Exercices + Leçon |
| Compter de 1 en 1, de 2 en 2, de 10 en 10 et à rebours | 1, 2 | `cp-count-1-2`, `cp-count-10-back`, `cp-ordinals` | Exercices + Leçon |
| Doubles et moitiés | 3, 5 | `cp-doubles-halves`, `cp-half-even` | Exercices + Leçon |
| L'addition posée | 5 | `cp-add-column` | Exercices + Leçon |
| Découvrir la multiplication : « fois » | 4 | `cp-times-as-repeated-add` | Exercices + Leçon |
| Partager et faire des groupes | 5 | `cp-share-group` | Exercices + Leçon |
| Résoudre des problèmes | 2, 5 | `cp-problems-one-step`, `cp-problems-two-steps` | Exercices + Leçon |

Les périodes suivent le programme officiel quand il les précise ; sinon ce sont des choix d'Educator, signalés dans le
champ `source` de chaque compétence.

## Évolutions du moteur et des écrans portées par les fiches du lot

- **Les nombres jusqu'à 100 : dizaines et unités** — Famille **numération** (lot 1, CE1) avec une **image** dans l'exercice : barres de dix et cubes dessinés (réutilise `tensUnits`).
- **Les nombres jusqu'à 100 : dizaines et unités** — Consigne lue avant l'affichage pour la dictée de nombres ; bouton Réécouter.
- **Comparer, ranger et placer les nombres jusqu'à 100** — Formats `compare`, `order` et `numberLinePoint` (lot 1, CE1), plages CP, tailles de boutons pour le CP.
- **Compter de 1 en 1, de 2 en 2, de 10 en 10 et à rebours** — Format `sequence` (lot 5) disponible au CP : suites courtes, cases vides au milieu ou à la fin.
- **Compter de 1 en 1, de 2 en 2, de 10 en 10 et à rebours** — Format `choice` sur image : une file de voitures dessinées, l'enfant clique sur celle demandée ; ordinaux lus par Piper (« sixième »).
- **Doubles et moitiés** — Format `phrase` (lot 1, CE1) avec des consignes adaptées au CP, lues par Piper.
- **L'addition posée** — Composant d'opération posée (lot 4) dans une version CP : grandes cases, une colonne à la fois, retenue pré-dessinée en pointillé.
- **Découvrir la multiplication : « fois »** — Format `phrase` : « 3 fois 20 » affiché en mots et en addition répétée, sans le signe × (introduit plus tard).
- **Découvrir la multiplication : « fois »** — Image de paquets d'objets dans l'exercice (réutilise `groupObjects`).
- **Partager et faire des groupes** — Format `phrase` avec gabarits de partage et de groupement, illustrés par `groupObjects` (pas de signe ÷ au CP).
- **Résoudre des problèmes** — Format `problem` (lot 1, CE1) adapté au CP : énoncé **lu** systématiquement, phrases courtes, image de la situation, réponse au pavé numérique.

Ces évolutions reprennent les fiches mathématiques de l'ancien lot 4 (SUB001601 à SUB001606, SUB001612), qui décrivaient le même
besoin pour toutes les classes. L'ancien lot 4 est clos (Done) : ses fiches sont remplacées par les lots 4, 5 et 6, un par niveau (décision de Georges du 2026-10-12).

## Patte graphique

Les leçons utilisent uniquement la « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » : charte « S1 · Grand Prix »
(SUB000105), voiture sur la piste graduée, damier, rouge course pour ce qui est nouveau. Un exercice qui affiche une
aide visuelle (droite graduée, bande de fraction, opération posée) réutilise les mêmes composants.

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] Les nouvelles compétences s'ajoutent aux 11 compétences du CP du lot 1, triées par période, sans les modifier.
- [ ] Les compétences de la période 5 du CP restent proposées en « Révisions » au CE1 (SUB000113).
- [ ] Chaque compétence du CP a une leçon, accessible depuis l'écran Leçons et depuis la liste des exercices.
- [ ] Toutes les compétences sont vérifiées contre le programme en vigueur, `source` renseignée.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
