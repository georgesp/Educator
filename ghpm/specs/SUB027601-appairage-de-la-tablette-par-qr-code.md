---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhA4-zg
rev: 2026-10-16T07:23:15Z
url: https://github.com/georgesp/Educator/issues/277
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB027601 — Appairage de la tablette par QR code
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#276"
order: 5060
---

## User story

En tant que parent, je veux associer la tablette à l'ordinateur en scannant un QR code, sans taper d'adresse ni de mot de passe.

## Travail

- Depuis l'espace parent de l'ordinateur (derrière la barrière « adulte » existante) : « Associer une tablette » affiche un **QR code** contenant l'adresse de l'ordinateur, le port, l'empreinte du certificat et un **secret d'appairage à usage unique**, valable quelques minutes.
- La tablette scanne le code (plugin Tauri de lecture de QR code / caméra, à confirmer dans l'étude), vérifie l'empreinte, échange le secret contre un **jeton durable** propre à cette tablette, puis le range dans le stockage sécurisé du système (Keychain iOS, Keystore Android) si possible.
- **Plan B sans caméra** : saisie manuelle d'un code court affiché sous le QR code (si la découverte automatique ne suffit pas).
- Nom de l'appareil modifiable, proposé par défaut sous une forme neutre (« Tablette 1 ») : pas de prénom d'enfant.
- Le secret ne sert qu'une fois ; après usage ou expiration, un nouveau QR code est demandé.

## Critères d'acceptation

- [ ] Appairage de bout en bout entre l'ordinateur et la tablette de test, en moins d'une minute.
- [x] Un QR code expiré, déjà utilisé, ou photographié puis rejoué plus tard ne permet pas de s'appairer (test automatisé côté serveur).
- [x] Le jeton n'apparaît ni dans les logs ni dans l'interface.
- [x] Caméra refusée par l'utilisateur : message bienveillant et plan B disponible.
- [x] Permissions mobiles déclarées (caméra, réseau local sur iOS) avec un texte d'explication en français.

## Réalisation (2026-10-15)

- QR code `educator://pair?v=1&h=…&p=…&f=<empreinte>&s=<secret>` dessiné en SVG par le cœur ; secret 128 bits,
  5 minutes, usage unique ; code court `ABCD-EFGH` + adresse pour le plan B ; jeton 256 bits, empreinte seule gardée
  par l'ordinateur.
- Lecture du QR code : `tauri-plugin-barcode-scanner` (iPad) ; `Info.plist` / `project.yml` : `NSCameraUsageDescription`,
  `NSLocalNetworkUsageDescription`, `NSBonjourServices`.
- Jeton de l'iPad rangé dans sa base (hors sauvegarde) : le Keychain est une fiche de suivi.
- Reste : l'appairage de bout en bout sur le Mac et l'iPad réels (recette SUB027609).
- 2026-10-16 : développement terminé et testé (tests automatiques verts) ; passée en revue. Reste la vérification sur le Mac et l'iPad réels, jouée avec la recette SUB027609.
