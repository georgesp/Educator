---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsEKY
url: https://github.com/georgesp/Educator/issues/252
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023813 — Icônes, écran de lancement et installation sur la tablette de test"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#238"
order: 3130
---

## User story

En tant que Georges, je veux installer Educator sur la tablette de mon enfant, avec une vraie icône, et la mettre à jour facilement.

## Dépend de

Fiches 01 et 02.

## Comportement attendu

- Icônes iOS (jeu complet généré depuis l'icône existante, `tauri icon`), écran de lancement sobre (couleur de la charte, mascotte), nom affiché « Educator ».
- iPad : installation sur appareil avec l'Apple ID gratuit (« Personal Team », profil valable 7 jours, voir `docs/tablette.md`) ; TestFlight seulement si un compte Apple Developer est pris.
- Procédure de mise à jour documentée dans `docs/tablette.md` (nouvelle version par-dessus, données conservées).
- `THIRD-PARTY-NOTICES.md` : rien à ajouter si seule la voix du système est utilisée ; à vérifier.

## Critères d'acceptation

- [ ] Installation réussie sur la tablette de test à partir d'un clone propre en suivant `docs/tablette.md`.
- [ ] Une mise à jour conserve les profils et les résultats.
- [ ] Hors périmètre rappelé : publication sur les stores.
