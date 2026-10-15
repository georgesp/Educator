---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDSo
rev: 2026-10-14T11:28:55Z
url: https://github.com/georgesp/Educator/issues/242
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023803 — Données, contenu pédagogique et logs sur mobile"
status: Todo
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

- Dossier de données : `educator_data::app_data_directory()` renvoie aujourd'hui les chemins du bureau (`~/Library/Application Support/Educator/`, `%APPDATA%\Educator\`). Sur mobile, utiliser le dossier de données privé de l'application fourni par Tauri (résolution du chemin dans `web/src-tauri`, passée aux crates : pas de dépendance à Tauri dans les crates).
- Base `educator.db` : migrations, WAL et sauvegarde `VACUUM INTO` vérifiés sur le système de fichiers mobile.
- Contenu : le dossier `content/` est embarqué comme ressource ; `find_content_directory` le trouve dans le paquet iOS (bundle de l'application).
- Aucun WAV de mot à exclure du paquet mobile : depuis le lot 2bis (ADR 0004), les mots sont lus en direct partout.
- Logs : dossier de logs de l'application, consultable pour le dépannage (`tauri-plugin-log`).
- Les variables `EDUCATOR_*` ne s'appliquent qu'au bureau.

## Critères d'acceptation

- [ ] Profils et résultats conservés après fermeture complète et relance de l'application.
- [ ] Le contenu est chargé en entier : aucun « Invalid content skipped » dans les logs, compétences et personnages présents.
- [ ] Taille de l'application mesurée et notée dans l'ADR 0005.
- [ ] Test automatique de la résolution des chemins (sans Tauri) dans `educator-data`.
