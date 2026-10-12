---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fM4
rev: 2026-10-11T15:22:42Z
url: https://github.com/georgesp/Educator/issues/63
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006003 — Commandes Tauri typées entre le front et le cœur
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#60"
order: 40
---

## User story

En tant que développeur, je veux un contrat typé entre le front et le cœur Rust afin de faire évoluer les deux sans casser le contrat.

## Comportement attendu

- Le front appelle des commandes Tauri (`invoke`) : aucun port réseau ouvert, donc ni token ni CORS à gérer.
- Contrats dans `web/src-tauri/src/contracts.rs` ; types TypeScript générés (`ts-rs`, `web/src/api/types.ts`), jamais écrits à la main.
- Erreurs : `CommandError { status, title, detail, field }`, avec les codes 400 / 404 / 409 / 503.
- Logs (`tauri-plugin-log`) dans la console et dans `logs/` du dossier de données.
- Commande `get_status` : version, état de la base, état de Piper.

## Critères d'acceptation

- [x] Aucun port HTTP ouvert par l'application.
- [x] Script `npm run api:generate` régénère les types TypeScript depuis les contrats Rust.
- [x] Tests des commandes (`services.rs`, base SQLite temporaire et contenu du dépôt).

## Implémentation

`commands.rs` (une commande par opération, exécutée avec `spawn_blocking`), `services.rs` (adaptateurs testés sans
Tauri), `bindings.rs` : le test `types_ts_is_up_to_date` régénère `web/src/api/types.ts` et échoue s'il n'était pas à
jour (vérifié aussi en CI).

*Première version en .NET (2026-10-10), portée en Rust au lot 1bis (2026-10-11, ADR 0003).*
