---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAvuo
url: https://github.com/georgesp/Educator/issues/76
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000115 — Modification du profil
status: Todo
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

- [ ] Prénom, classe et personnage modifiables, enregistrés en base.
- [ ] Après un changement de classe, la liste des compétences correspond à la nouvelle classe.
- [ ] Bouton Annuler qui ne change rien.
- [ ] Tests Vitest de l'écran et test des commandes.

## Notes techniques

- Les commandes existent déjà : `update_profile` (`updateProfile`) et `reroll_character`. Le travail est surtout côté front, plus la vérification des validations dans le cœur.
- La fiche SUB001416 prévoyait déjà « renommer, changer la classe, changer le personnage » dans les Options : cette partie passe au lot 1 avec cette fiche.
