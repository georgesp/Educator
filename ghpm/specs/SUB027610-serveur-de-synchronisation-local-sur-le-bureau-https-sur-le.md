---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhA4_QU
url: https://github.com/georgesp/Educator/issues/286
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB027610 — Serveur de synchronisation local sur le bureau (HTTPS sur le Wi-Fi)
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#276"
order: 5050
---

## User story

En tant que parent, je veux que l'ordinateur reçoive les données de la tablette sur mon Wi-Fi, sans qu'aucun autre appareil du réseau puisse y accéder.

## Travail

- Nouvelle crate (par exemple `educator-sync`) pour le transport : serveur HTTPS léger, compilé sur le **bureau seulement** (`cfg(desktop)`), qui appelle le moteur de fusion via les traits de `educator-core`. Choix de la bibliothèque et du TLS dans l'ADR 0006 ; toute dépendance embarquée est consignée dans `THIRD-PARTY-NOTICES.md`.
- **TLS** avec un certificat auto-signé généré au premier lancement et conservé dans le dossier de données ; la tablette l'**épingle** par son empreinte (donnée dans le QR code), pas d'autorité de certification.
- **Authentification** : jeton propre à chaque tablette appairée (stocké sous forme d'empreinte côté ordinateur), vérifié à chaque requête ; appareil révoqué = refus immédiat.
- **Exposition minimale** : le serveur n'écoute que lorsque l'ADR l'autorise (écran « Synchronisation » ouvert, ou option activée), sur l'interface du réseau local, port choisi au hasard ou fixe documenté ; arrêt propre à la fermeture de l'écran ou de l'application.
- **Annonce sur le réseau** (mDNS / Bonjour) sous un nom neutre, sans prénom d'enfant, pour retrouver l'ordinateur sans saisir d'adresse.
- Limites de débit et de taille des requêtes ; aucune donnée d'enfant dans les logs (identifiants tronqués, comptes seulement).
- Fonctionne avec le pare-feu de macOS et de Windows : message explicite si l'autorisation réseau est refusée (tests Windows reportés comme le reste du projet).

## Critères d'acceptation

- [ ] Un client non appairé, un jeton invalide ou révoqué, ou une empreinte de certificat différente est refusé (tests d'intégration avec un client de test).
- [ ] Le serveur n'est pas joignable quand l'écran de synchronisation est fermé (si c'est la règle retenue).
- [ ] Une synchronisation complète se fait entre deux instances de test sur la machine (client simulé), sans réseau externe.
- [ ] Rien n'est écrit dans les logs qui permette d'identifier un enfant.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` et `cargo test --workspace` verts ; test d'architecture mis à jour pour la nouvelle crate.
