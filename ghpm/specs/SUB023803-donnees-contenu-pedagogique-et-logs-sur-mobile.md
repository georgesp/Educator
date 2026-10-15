---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDSo
rev: 2026-10-15T18:20:47Z
url: https://github.com/georgesp/Educator/issues/242
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023803 — Données, contenu pédagogique et logs sur mobile"
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#238"
order: 3030
---

## User story

En tant que parent, je veux que les profils et les résultats de mon enfant soient conservés sur la tablette d'une séance à l'autre, sans rien configurer.

## Dépend de

Fiche 02.

## Comportement attendu

- Dossier de données : `educator_data::app_data_directory()` renvoyait les chemins du bureau et tombait sur `~/.config/Educator` sur l'iPad, non inscriptible (fermeture au lancement). Retenu (2026-10-15) : sur iPad, `<conteneur de l'app>/Library/Application Support/Educator/`, le même emplacement que celui de Tauri, sans dépendance à Tauri dans les crates ; résolution en fonction pure `data_directory_for(Platform, variables)`.
- Base `educator.db` : migrations, WAL et sauvegarde `VACUUM INTO` vérifiés sur le système de fichiers mobile.
- Contenu : le dossier `content/` est embarqué comme ressource ; `find_content_directory` le trouve dans le paquet iOS (bundle de l'application).
- Aucun WAV de mot à exclure du paquet mobile : depuis le lot 2bis (ADR 0004), les mots sont lus en direct partout.
- Logs : dossier de logs de l'application, consultable pour le dépannage (`tauri-plugin-log`).
- Les variables `EDUCATOR_*` ne s'appliquent qu'au bureau.

## Critères d'acceptation

- [x] Profils et résultats conservés après fermeture complète et relance de l'application.
- [x] Le contenu est chargé en entier : aucun « Invalid content skipped » dans les logs, compétences et personnages présents.
- [x] Taille de l'application mesurée et notée dans l'ADR 0006 (`.ipa` 4,9 Mo, application installée 11 Mo).
- [x] Test automatique de la résolution des chemins (sans Tauri) dans `educator-data` (`crates/educator-data/tests/paths.rs`).

## Implémentation (2026-10-15)

- `crates/educator-data/src/paths.rs` : l'iPad range ses données dans `<conteneur>/Library/Application Support/Educator/` (fermeture au lancement corrigée : `~/.config` n'est pas inscriptible sur iOS) ; résolution en fonction pure `data_directory_for(Platform, variables)`, 7 tests (`crates/educator-data/tests/paths.rs`).
- Contenu trouvé dans le paquet iOS (`assets/content`), base et logs dans le dossier de données ; persistance vérifiée par Georges sur l'iPad.
- Taille (version finale) : `.ipa` 4,9 Mo, application installée 11 Mo (exécutable 8,0 Mo, contenu 3,0 Mo), notée dans l'ADR 0006.
