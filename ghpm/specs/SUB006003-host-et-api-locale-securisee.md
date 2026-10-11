---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fM4
rev: 2026-10-10T19:56:51Z
url: https://github.com/georgesp/Educator/issues/63
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006003 — Host et API locale sécurisée
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#60"
order: 4
---

## User story

En tant que développeur, je veux une API locale typée et sécurisée entre le front et le backend afin de faire évoluer les deux sans casser le contrat.

## Comportement attendu

- `Educator.Host` : ASP.NET Core, écoute en loopback sur un port libre, logs Serilog dans le dossier de données.
- Toutes les routes `/api/ui/*` exigent l'en-tête `X-Local-Token` (token aléatoire généré au démarrage).
- CORS limité aux origines du front (`http://localhost:1420` en dev, `tauri://localhost`, `http://tauri.localhost`).
- OpenAPI exposée par le host ; client TypeScript généré (`openapi-typescript` + `openapi-fetch`), jamais écrit à la main.
- `GET /api/ui/status` : version, état de la base, état de Piper.

## Critères d'acceptation

- [x] Requête sans token ou avec un mauvais token → `401`.
- [x] Script `npm run api:generate` régénère le client depuis l'OpenAPI.
- [x] Tests d'intégration de l'API (WebApplicationFactory).

## Implémentation (2026-10-10)

Tests d'intégration sur TestServer (`Microsoft.AspNetCore.TestHost`) plutôt que WebApplicationFactory, pour garder le `Main` du host. L'OpenAPI est exportée dans `web/openapi/openapi.json` (`--export-openapi`) ; le test `CommittedOpenApiFile_MatchesTheApi` vérifie qu'elle est à jour.
