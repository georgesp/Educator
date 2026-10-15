---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhA4-8c
url: https://github.com/georgesp/Educator/issues/279
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB027603 — Écran « Synchronisation » de l'espace parent
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#276"
order: 5080
---

## User story

En tant que parent, je veux voir quels appareils sont associés, quand ils ont synchronisé pour la dernière fois, et pouvoir en retirer un.

## Travail

- Écran dans l'espace parent (derrière la barrière « adulte »), sur l'ordinateur **et** sur la tablette, avec les textes dans le fichier de ressources du front (jamais en dur dans les composants).
- **Ordinateur** : activer / désactiver la réception, afficher le QR code d'appairage, liste des tablettes associées (nom, dernière synchronisation), bouton « Retirer cette tablette » (confirmation, révocation immédiate du jeton).
- **Tablette** : état (associée à quel ordinateur, dernière synchronisation), bouton « Synchroniser maintenant », bouton « Dissocier ».
- Écran de **choix** au premier appairage quand des profils portent le même prénom des deux côtés (« Est-ce le même enfant ? »), selon l'ADR 0006.
- Commandes Tauri dans `commands.rs` / `services.rs` / `contracts.rs`, sans règle métier ; types TypeScript générés.

## Critères d'acceptation

- [ ] Les actions ci-dessus fonctionnent de bout en bout ; retirer une tablette l'empêche immédiatement de synchroniser.
- [ ] Les statistiques de l'espace parent reflètent les données fusionnées, sur les deux appareils.
- [ ] Ton bienveillant, aucun message culpabilisant ; écrans utilisables au doigt sur la tablette (cibles tactiles, voir SUB023809).
- [ ] Tests Vitest des composants avec une `UiApi` simulée ; parcours Playwright de l'écran côté ordinateur.
