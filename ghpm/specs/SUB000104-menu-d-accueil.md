---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Rtk
rev: 2026-10-11T15:22:28Z
url: https://github.com/georgesp/Educator/issues/5
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000104 — Menu d'accueil
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#1"
order: 150
---

## User story

En tant qu'enfant, je veux un menu simple façon jeu vidéo afin de savoir tout de suite où aller.

## Comportement attendu

- Mise en page en deux colonnes (maquette « S1 · Grand Prix — Menu », voir SUB000105) :
  - **à gauche**, le menu aligné verticalement : 4 grandes entrées empilées **Leçons**, **Exercices**, **Examen**, **Options** ;
  - **à droite**, le panneau du profil : personnage en grand, nom, bouton **« Changer de profil »** (retour à l'écran de choix), **classe**, **points** et **niveau** avec sa progression.
- Le niveau n'existe qu'au lot 3 (SUB001501) : au lot 1, le panneau affiche classe et points, et la place du niveau reste prévue.
- Fenêtre étroite : le panneau du profil passe sous le menu.
- Lot 1 : seule l'entrée **Exercices** est active. Leçons, Examen et Options sont visibles mais grisées avec un petit cadenas et l'indication « Bientôt ».
- Chaque entrée peut être lue à voix haute au survol ou via le bouton haut-parleur.

## Critères d'acceptation

- [x] Les points affichés sont à jour au retour d'une série d'exercices.
- [x] Les entrées inactives ne sont pas cliquables et l'expliquent à la voix (« Cette partie arrive bientôt »).
- [x] Navigation possible au clavier (flèches + Entrée) en plus de la souris.

## Implémentation (2026-10-11)

- `MenuScreen` reprend la maquette S1 : entrées numérotées 01 à 04 en parallélogramme à gauche, panneau du profil à droite. Ce panneau montre le personnage, « Changer de pilote », la classe et les points. La place du niveau est réservée, avec la mention « Les niveaux arrivent bientôt ».
- Les entrées verrouillées (cadenas) restent focalisables. Un clic dit « Cette partie arrive bientôt » sans changer d'écran.
- Les flèches haut et bas passent d'une entrée à l'autre ; Entrée ou Espace active l'entrée.
- Une entrée est lue à voix haute après 0,6 s de survol. Le bouton haut-parleur relit la consigne.
- Tests : `MenuScreen.test.tsx`. Le parcours `e2e/lot1.spec.ts` vérifie les points à jour au retour d'une série.
