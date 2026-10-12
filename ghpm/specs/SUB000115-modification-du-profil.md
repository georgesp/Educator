---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAvuo
rev: 2026-10-11T15:22:51Z
url: https://github.com/georgesp/Educator/issues/76
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000115 — Modification du profil
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#1"
order: 124
---

Source : retours de Georges après les premiers essais du lot 1 (2026-10-11).

## User story

En tant qu'enfant ou parent, je veux modifier un profil après l'avoir créé afin de corriger un prénom, changer de classe ou de personnage.

## Comportement attendu

- Depuis l'écran de choix du profil et depuis le panneau de l'avatar du menu, un bouton **Modifier** ouvre la fiche du profil.
- On peut modifier :
  - le prénom (mêmes règles qu'à la création) ;
  - la classe (CP → CM2) ;
  - le personnage (choix dans la liste des personnages ou nouveau tirage au hasard).
- Les points et l'historique du profil sont conservés.
- Le changement de classe est pris en compte immédiatement dans les compétences proposées.
- La **suppression** d'un profil reste dans la fiche « Activation du menu et gestion des profils » (SUB001416, lot 2), avec sa confirmation en deux étapes.

## Critères d'acceptation

- [x] Prénom, classe et personnage modifiables, enregistrés en base.
- [x] Après un changement de classe, la liste des compétences correspond à la nouvelle classe.
- [x] Bouton Annuler qui ne change rien.
- [x] Tests Vitest de l'écran et test des commandes.

## Notes techniques

- Les commandes existent déjà : `update_profile` (`updateProfile`) et `reroll_character`. Le travail est surtout côté front, plus la vérification des validations dans le cœur.
- La fiche SUB001416 prévoyait déjà « renommer, changer la classe, changer le personnage » dans les Options : cette partie passe au lot 1 avec cette fiche.

## Implémentation (2026-10-11)

- Nouvel écran `EditProfileScreen` (route `/profil/:id`). On l'ouvre avec un lien « Modifier » sous chaque
  profil de l'écran de choix, et dans le panneau du profil du menu (`?retour=menu` pour y revenir).
- On y modifie :
  - le prénom, avec les mêmes règles qu'à la création ; le message d'erreur du cœur est affiché tel quel ;
  - la classe ;
  - le personnage, choisi dans la liste ou tiré « Au hasard » (`draw_character`).
- Rien n'est enregistré avant « Enregistrer » : le tirage au hasard n'utilise pas `reroll_character`, qui
  enregistre tout de suite. « Annuler » ne change rien.
- Un seul appel `update_profile`. Les points et l'historique sont conservés. Après un changement de classe, la
  liste des compétences suit la nouvelle classe : la requête dépend de la classe du profil.
- La suppression reste dans SUB001416, dont la fiche a été mise à jour.
- Tests :
  - `EditProfileScreen.test.tsx` : enregistrement, Annuler, tirage, prénom refusé, compétences de la nouvelle
    classe ;
  - test des validations de mise à jour dans le cœur (`profiles.rs`) ;
  - test de la commande (`services.rs`) : points conservés, compétences CE1, personnage inconnu refusé (400).
