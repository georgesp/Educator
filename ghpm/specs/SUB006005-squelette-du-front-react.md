---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fQA
rev: 2026-10-11T15:22:44Z
url: https://github.com/georgesp/Educator/issues/65
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006005 — Squelette du front React
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#60"
order: 60
---

## User story

En tant que développeur, je veux un front prêt à accueillir les écrans enfant afin de démarrer le lot 1 directement sur les fonctionnalités.

## Contenu

- Vite + React 19 + TypeScript, routing, TanStack Query branché sur les commandes Tauri (types générés).
- Framer Motion installé (animations), dnd-kit prévu pour le lot 2.
- Textes de l'interface en français, centralisés dans un fichier de ressources.
- Vitest pour les tests de composants, Playwright pour quelques parcours.
- Pas de bibliothèque de composants généraliste : composants enfant maison (lot 1).

## Critères d'acceptation

- [x] `npm run build` (vérification TypeScript + build Vite) passe.
- [x] Un test Vitest et un test Playwright d'exemple tournent en CI.

## Implémentation (2026-10-10)

React Router 7 (routes en hash), Motion (nouveau nom de Framer Motion), appels `invoke` dans `src/api/ui.ts`, textes dans
`src/i18n/fr.ts`. Vérifié : build, tests Vitest, parcours Playwright (commandes Tauri simulées dans la page).
TypeScript 5.9 conservé (choisi au lot 0 pour `openapi-typescript`, retiré au lot 1bis ; passage à la 6 possible).

*Première version en .NET (2026-10-10), portée en Rust au lot 1bis (2026-10-11, ADR 0003).*
