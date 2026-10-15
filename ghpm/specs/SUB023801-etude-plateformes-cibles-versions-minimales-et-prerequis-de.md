---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDB8
rev: 2026-10-15T15:35:12Z
url: https://github.com/georgesp/Educator/issues/240
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023801 — Étude — plateformes cibles, versions minimales et prérequis de la version tablette"
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#238"
order: 3010
---

## User story

En tant que Georges, je veux savoir précisément ce qu'il faut (matériel, comptes, outils) pour construire et installer Educator sur ma tablette, avant d'engager le développement.

## Questions à trancher

- iPad, Android, ou les deux ? Quelle tablette de test (modèle, version du système) ? Orientation : paysage seulement, ou portrait aussi ?
- Versions minimales : iPadOS et Android (le stylet du lot suivant pourra en demander une plus récente, par exemple Android 14 pour l'écriture manuscrite système : à vérifier).
- Outils : Xcode et compte Apple Developer, Android Studio / SDK / NDK, cibles Rust (`aarch64-apple-ios`, `aarch64-apple-ios-sim`, `aarch64-linux-android`), version de Tauri CLI.
- Taille de l'application : le dossier `content/` seul, sans l'audio des mots (voir fiche 03).

## Livrable

- ADR 0006 « Application tablette » (`docs/adr/0006-application-tablette.md` ; le numéro 0005 est pris par la voix sur tablette) : décisions ci-dessus, prérequis d'installation, risques ; sert de base aux fiches suivantes du lot (la partie voix est complétée par la fiche 04).
- `docs/tablette.md` : procédure pas à pas pour préparer le Mac et la tablette.

## Critères d'acceptation

- [x] ADR 0006 rédigé et validé par Georges.
- [x] La liste des fiches iOS / Android du lot est ajustée selon les plateformes retenues (iPad seul : SUB023807 et SUB023905 mises de côté ; SUB023804, SUB023812, SUB023813 limitées à iOS).

## Implémentation (2026-10-15)

- `docs/adr/0006-application-tablette.md` (acceptée) : iPad seul (pas d'iPhone, Android mis de côté), iPad Air 13 pouces M2 sous iPadOS 27, paysage seul, iPadOS 17 minimum, Mac M1 Pro (macOS 27.0.1, Xcode 27.0), Apple ID gratuit, même dépôt, saisie par les claviers de l'application, risques.
- `docs/tablette.md` : préparation du Mac et de l'iPad pas à pas, dépannage (signature, `libc`, confiance, réseau local, simulateur Device Hub).
- Fiches du lot ajustées : SUB023807 et SUB023905 mises de côté ; SUB023802, 03, 04, 12, 13 limitées à iOS.
