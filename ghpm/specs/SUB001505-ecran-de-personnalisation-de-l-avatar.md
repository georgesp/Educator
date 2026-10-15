---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WoY
rev: 2026-10-10T19:39:28Z
url: https://github.com/georgesp/Educator/issues/38
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001505 — Écran de personnalisation de l'avatar
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#15"
order: 615
---

## User story

En tant qu'enfant, je veux essayer mes accessoires sur mon personnage afin de choisir ma tenue.

## Comportement attendu

- Accessible depuis le menu (clic sur l'avatar).
- Aperçu en grand, accessoires rangés par emplacement (tête, yeux, cou, compagnon), un seul accessoire par emplacement.
- Accessoires verrouillés visibles avec un cadenas et la condition (« Atteins le niveau 5 »).
- Changement de personnage de base toujours possible.

## Critères d'acceptation

- [x] Sauvegarde immédiate, sans bouton Valider.
- [x] Utilisable à la souris seule.

## Réalisation

- Écran **Mon avatar** (`/avatar`) : clic sur l'avatar du menu (pastille « Habiller »).
- Aperçu en grand ; onglets **Accessoires** / **Personnage** ; emplacements Tête, Yeux, Cou, Compagnon ; une tuile par accessoire montrant l'avatar qui le porte ; « Rien » pour vider l'emplacement, un second clic retire l'accessoire porté.
- Accessoire verrouillé : grisé avec un cadenas ; un clic affiche la condition (« Atteins le niveau 5 », « Gagne le badge « 10 séries sans faute » »).
- Pastilles de couleur sous la grille pour l'accessoire porté dans l'emplacement.
- Chaque clic est enregistré tout de suite (commande `update_avatar`, refusée par le cœur pour un accessoire verrouillé ou deux accessoires au même emplacement) ; l'écran revient à l'état enregistré en cas d'erreur.
- Changement de personnage de base toujours possible, sans perdre les accessoires.
