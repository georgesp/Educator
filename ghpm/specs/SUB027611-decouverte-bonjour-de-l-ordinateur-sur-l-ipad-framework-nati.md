---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBLzc8
url: https://github.com/georgesp/Educator/issues/288
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB027611 — Découverte Bonjour de l'ordinateur sur l'iPad (framework natif)
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#276"
order: 5110
---

## User story

En tant que parent, je veux que l'iPad retrouve l'ordinateur même s'il a changé d'adresse sur le Wi-Fi (box
redémarrée), sans scanner à nouveau le QR code.

## Contexte

Suivi du lot tablette 2 (ADR 0007, section 7). L'ordinateur s'annonce déjà en Bonjour (`_educator-sync._tcp`, nom
« Educator-XXXX »). Sur iPadOS, une bibliothèque Rust qui écoute le multicast demande l'autorisation
`com.apple.developer.networking.multicast`, accordée par Apple sur demande et impossible avec le compte gratuit : la
recherche doit passer par `NWBrowser` (Network framework), en Swift.

## Travail

- Petit plugin Tauri (ou extension du plugin existant) : `NWBrowser` sur `_educator-sync._tcp`, renvoie les adresses
  et le port des ordinateurs trouvés.
- `ClientSync` essaie, après les adresses connues, celles trouvées par Bonjour ; l'empreinte du certificat reste
  vérifiée (une machine qui s'annonce sous le même nom ne passe pas).

## Critères d'acceptation

- [ ] Ordinateur dont l'adresse a changé : la synchronisation suivante réussit sans nouveau scan.
- [ ] Une autre machine annoncée sous le même nom est refusée (empreinte).
- [ ] Permission « Réseau local » refusée : message clair dans l'espace parent.
