---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Rrc
rev: 2026-10-11T07:56:31Z
url: https://github.com/georgesp/Educator/issues/4
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000103 — Écran de choix et de création du profil
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 40
---

## User story

En tant qu'enfant, je veux choisir mon profil ou en créer un nouveau dès l'ouverture de l'application afin de commencer à jouer avec mon personnage.

## Comportement attendu

- Premier écran de l'application.
- Grille de profils : grand personnage + nom en dessous ; un clic ouvre le menu d'accueil avec ce profil.
- Carte **« Nouveau profil »** toujours présente.
- Aucun profil existant : l'écran affiche directement la création.
- Création en une seule étape :
  - saisie du nom (clavier physique ou à l'écran) ;
  - choix de la classe par 5 gros boutons (CP, CE1, CE2, CM1, CM2) ;
  - personnage tiré au hasard, affiché en grand, avec un bouton **« Changer de personnage »** (nouveau tirage) ;
  - bouton **« C'est parti ! »**.
- La consigne de l'écran est lue par Piper (bouton haut-parleur, et lecture automatique au premier affichage).

## Critères d'acceptation

- [x] Création, sélection et retirage du personnage fonctionnent avec la souris seule.
- [x] Les erreurs de validation (nom vide, déjà pris) s'affichent en phrase simple, sans code d'erreur.
- [x] Le profil choisi est mémorisé pour la session (contexte front) et `LastUsedAt` est mis à jour.
- [x] Rendu correct de 1 à 8 profils sans défilement sur une fenêtre de 1280 × 800.
- [x] Test de parcours (Playwright) : création d'un profil puis sélection.

## Notes techniques

- Composants maison de la charte enfant (voir fiche « Charte graphique et composants enfant »).

## Implémentation (2026-10-11)

- Écran `ProfilesScreen` : grille des profils (personnage, prénom, classe) et carte « Nouveau profil ». Sans profil, l'écran de création s'affiche directement.
- Création : prénom, 5 boutons de classe, personnage tiré au hasard avec « Changer de personnage », puis « C'est parti ! ». Le personnage affiché est tiré par le cœur (`draw_character`), puis transmis à la création : c'est bien lui qui est enregistré.
- La consigne est lue au premier affichage, et le bouton Écouter la relit.
- Contrôlé en capture d'écran : 9 profils et la carte « Nouveau profil » tiennent sans défilement en 1280 × 800.
- Tests : `ProfilesScreen.test.tsx` (Vitest) ; `e2e/lot1.spec.ts` (création, puis sélection d'un profil existant).
