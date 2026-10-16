---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhA4-28
rev: 2026-10-16T07:23:16Z
url: https://github.com/georgesp/Educator/issues/278
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB027602 — Client de synchronisation sur la tablette
status: In Review
size: L
estimate:
startDate:
endDate:
parent: "#276"
order: 5070
---

## User story

En tant que parent, je veux que la tablette envoie et reçoive les données dès qu'elle retrouve l'ordinateur sur le Wi-Fi, ou quand j'appuie sur un bouton.

## Travail

- Client HTTPS dans la crate de transport (compilé aussi pour iOS et Android), avec épinglage du certificat enregistré à l'appairage.
- **Retrouver l'ordinateur** : découverte mDNS / Bonjour, avec repli sur la dernière adresse connue ; sur iOS, déclarer le service Bonjour et la permission « Réseau local » ; sur Android, verrou multicast et configuration de sécurité réseau pour l'empreinte épinglée.
- Déclenchement : bouton « Synchroniser » dans l'espace parent de la tablette, et tentative silencieuse à l'ouverture de l'application si un appareil est appairé. **Jamais pendant une série d'exercices** ni pendant un défi chrono.
- Gestion du cycle de vie mobile (voir SUB023811) : si l'application passe en arrière-plan en pleine synchronisation, elle s'arrête proprement et reprend plus tard (idempotence).
- Échecs normaux (ordinateur éteint, autre réseau Wi-Fi, certificat différent, version incompatible) : messages simples et bienveillants dans l'espace parent, aucun message d'erreur pour l'enfant.

## Critères d'acceptation

- [ ] Synchronisation complète entre l'ordinateur et la tablette de test, dans les deux sens.
- [x] Ordinateur éteint ou sur un autre réseau : la tablette fonctionne normalement hors ligne, la synchronisation est simplement reportée.
- [x] Wi-Fi coupé en plein échange : aucun doublon, aucune perte à la synchronisation suivante.
- [x] Une empreinte de certificat différente (autre machine, tentative d'usurpation) bloque la synchronisation avec un message clair.
- [x] Aucune synchronisation ne démarre pendant une série ou un défi.

## Réalisation (2026-10-15)

- `educator-sync::service::ClientSync` : bonjour (profils de l'ordinateur), envoi, réception, envoi de ce que la
  réception a changé ; curseurs enregistrés après chaque lot ; arrêt propre au passage en arrière-plan
  (`interrupt_sync`) ; tentative silencieuse à « Qui joue ? » (`web/src/app/automaticSync.ts`).
- Écart : l'iPad retrouve l'ordinateur par les adresses du QR code et la dernière adresse qui a répondu ; la recherche
  Bonjour native est une fiche de suivi (le multicast « brut » demande une autorisation d'Apple, ADR 0007).
- Reste : la synchronisation complète sur l'iPad réel (recette SUB027609).
- 2026-10-16 : développement terminé et testé (tests automatiques verts) ; passée en revue. Reste la vérification sur le Mac et l'iPad réels, jouée avec la recette SUB027609.
