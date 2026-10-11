---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAv2E
url: https://github.com/georgesp/Educator/issues/81
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB007204 — Remplacer l'API locale par des commandes Tauri
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#72"
order: 160
---

## User story

En tant que développeur, je veux que le front appelle la logique directement dans Tauri afin de supprimer le serveur HTTP local, son token et le lancement du host.

## Comportement attendu

- Une commande Tauri par opération de `UiApi` (13) : statut, compétences, personnages (liste, tirage, SVG), profils (liste, création, modification, sélection, nouveau tirage du personnage), séries (démarrer, répondre, abandonner), voix.
- Les types échangés reprennent les contrats de `Educator.Host/Api/Contracts.cs` (sérialisation `camelCase`), types TypeScript générés depuis Rust (`ts-rs` ou `specta`) à la place de `openapi.json`.
- `speak` renvoie le WAV en binaire (`tauri::ipc::Response`), converti en `Blob` côté front.
- `web/src/api/ui.ts` : nouvelle implémentation de `UiApi` avec `invoke()` ; les écrans ne changent pas.
- `BackendGate` / `waitForBackend` : simplifiés (plus de host à attendre, seulement l'initialisation du contenu et de la base).
- Les erreurs métier gardent les mêmes messages côté front.

## Critères d'acceptation

- [x] Les tests front (Vitest) passent, `UiApi` toujours injectée par `BackendContext` et mockée.
- [x] Le scénario Playwright `web/e2e/lot1.spec.ts` passe.
- [x] Plus de port HTTP ouvert par l'application.

## Notes de réalisation

- `web/src-tauri/src/` : `contracts.rs` (contrats, mêmes champs et valeurs qu'en C#), `services.rs` (adaptateurs
  testés directement), `commands.rs` (16 commandes : les 13 de `UiApi`, `get_startup`, `reroll_character`,
  `delete_profile`), `state.rs` (démarrage en arrière-plan), `bindings.rs` (génération de `web/src/api/types.ts`).
- Types TypeScript générés avec `ts-rs` (dépendance de test seulement) : `npm run api:generate` lance `cargo test`.
- Erreurs : `CommandError { status, title, detail, field }`, mêmes codes et messages qu'avant ; `ApiError` côté front.
- `speak` renvoie le WAV en binaire (`tauri::ipc::Response`), converti en `Blob`.
- `BackendGate` attend `get_startup` (contenu et base chargés dans un thread au lancement).
- Playwright : les commandes sont simulées dans la page (`window.__TAURI_INTERNALS__` + `page.exposeFunction`).
- `openapi-fetch` et `openapi-typescript` retirés de `package.json`.
