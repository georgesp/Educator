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

- Icônes iOS et Android (jeu complet généré depuis l'icône existante, `tauri icon`), écran de lancement sobre (couleur de la charte, mascotte), nom affiché « Educator ».
- iPad : installation sur appareil avec le compte retenu (profil de développement ou TestFlight) ; Android : APK ou AAB de test installable en sideload.
- Procédure de mise à jour documentée dans `docs/tablette.md` (nouvelle version par-dessus, données conservées).
- `THIRD-PARTY-NOTICES.md` : rien à ajouter si seule la voix du système est utilisée ; à vérifier.

## Critères d'acceptation

- [ ] Installation réussie sur la tablette de test à partir d'un clone propre en suivant `docs/tablette.md`.
- [ ] Une mise à jour conserve les profils et les résultats.
- [ ] Hors périmètre rappelé : publication sur les stores.
