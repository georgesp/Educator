---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wx0
rev: 2026-10-16T19:31:19Z
url: https://github.com/georgesp/Educator/issues/45
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001512 — Options et réglages
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#15"
order: 630
---

## User story

En tant que parent ou enfant, je veux adapter l'application à mes besoins.

## Réglages

- Côté enfant : son on/off, vitesse de la voix, taille du texte, police adaptée à la dyslexie.
- Côté parent : activation du défi chrono et sa durée, objectif du jour, durée de séance conseillée (rappel doux de faire une pause).
- **Sauvegarde / restauration** de tous les profils dans un fichier (export avant changement d'ordinateur).

## Critères d'acceptation

- [x] Réglages stockés dans `AppSetting` (globaux) ou dans le profil (par enfant).
- [x] Export / import testés (aller-retour sans perte).
- [x] Police dyslexie libre de droits, licence consignée.

## Réalisation

- Réglages **par enfant** (table `ProfileSetting`, clé / valeur ; `educator-core::profile_settings`) ; une valeur inconnue est refusée (400), une valeur absente prend sa valeur par défaut. La carte des parents reste dans `AppSetting`.
- Options (enfant) : couper les sons et la voix, cacher la mascotte (ces deux réglages quittent le stockage du navigateur), vitesse de la voix (plus lente / normale / plus rapide : ± 0,1 sur les vitesses des consignes et des mots, l'écoute lente reste à 0,5), taille du texte (normale / grande / très grande : zoom de toute l'interface), police adaptée à la dyslexie (OpenDyslexic, SIL OFL 1.1, embarquée via `@fontsource/opendyslexic`, consignée dans `THIRD-PARTY-NOTICES.md`), thème (SUB001513). Tout s'applique tout de suite (`ProfileSettingsEffect`).
- Espace parent → « Réglages de … » : objectif du jour (SUB001507), défi chrono proposé ou non et sa durée (au choix de l'enfant, 1, 2 ou 3 minutes), durée de séance conseillée (aucune, 15, 20, 30 ou 45 min) : passé ce temps, un petit message propose une pause, sans bloquer.
- **Sauvegarde / restauration** dans l'espace parent : un fichier JSON avec toutes les tables (`educator-data::backup`), choisi par la fenêtre d'enregistrement du système (`tauri-plugin-dialog`), nommé `AAAA-MM-JJ_educator-sauvegarde.json`. La restauration remplace tous les profils après confirmation, garde d'abord une copie de la base (`educator.db.before-restore-…`), accepte une sauvegarde d'une version plus ancienne et refuse celle d'une version plus récente. Testée : `a_backup_round_trip_loses_nothing` (aller-retour identique), `a_backup_written_then_read_gives_the_profiles_back`.
