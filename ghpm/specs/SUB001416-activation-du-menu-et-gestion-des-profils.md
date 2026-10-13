---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Whk
rev: 2026-10-11T16:02:55Z
url: https://github.com/georgesp/Educator/issues/32
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001416 — Activation du menu et gestion des profils
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#14"
order: 490
---

## User story

En tant qu'enfant ou parent, je veux accéder à toutes les parties de l'application et gérer les profils.

## Comportement attendu

- Leçons, Exercices et Examen actifs dans le menu.
- Exercices → choix de la matière (Mathématiques / Orthographe).
- **Options** (première version) : supprimer un profil (confirmation explicite en deux étapes), couper les sons. Renommer un profil, changer sa classe et son personnage sont faits au lot 1 par SUB000115 (bouton « Modifier », écran `EditProfileScreen`) : les Options pourront y mener.

## Critères d'acceptation

- [x] Suppression d'un profil impossible par un clic accidentel.
- [x] Changement de classe pris en compte immédiatement dans les compétences proposées.

## Implémentation (2026-10-14)

- Leçons, Exercices, Examen et Options actifs ; Exercices et Leçons → choix Mathématiques / Orthographe.
- Options (`OptionsScreen`) : couper les sons, masquer la mascotte, modifier le profil (`EditProfileScreen`),
  supprimer un profil (bouton, puis confirmation où il faut retaper le nom).
- Les réglages « couper les sons » et « masquer la mascotte » sont gardés sur l'ordinateur (stockage du webview,
  comme le bouton son du lot 1) ; ils passeront dans la base avec les Options du lot 3 (SUB001512).
