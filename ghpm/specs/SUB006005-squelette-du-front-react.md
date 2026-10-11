---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fQA
url: https://github.com/georgesp/Educator/issues/65
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006005 — Squelette du front React
status: In progress
size: S
estimate:
startDate:
endDate:
parent: "#60"
order: 6
---

## User story

En tant que développeur, je veux un front prêt à accueillir les écrans enfant afin de démarrer le lot 1 directement sur les fonctionnalités.

## Contenu

- Vite + React 19 + TypeScript, routing, TanStack Query branché sur le client API généré.
- Framer Motion installé (animations), dnd-kit prévu pour le lot 2.
- Textes de l'interface en français, centralisés dans un fichier de ressources.
- Vitest pour les tests de composants, Playwright pour quelques parcours.
- Pas de bibliothèque de composants généraliste : composants enfant maison (lot 1).

## Critères d'acceptation

- [x] `npm run build` (vérification TypeScript + build Vite) passe.
- [x] Un test Vitest et un test Playwright d'exemple tournent en CI.

## Implémentation (2026-10-10)

React Router 7 (routes en hash), Motion (nouveau nom de Framer Motion), client `openapi-fetch`, textes dans `src/i18n/fr.ts`. Vérifié : build, 4 tests Vitest, 1 test Playwright (API simulée). La CI les exécute une fois le workflow ajouté.
TypeScript 5.9 (et non 6) : `openapi-typescript` 7 n'accepte que TypeScript 5 ; avec la 6, `npm ci` échoue sur un clone neuf.
