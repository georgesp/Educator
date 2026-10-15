---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDzI
rev: 2026-10-14T11:29:06Z
url: https://github.com/georgesp/Educator/issues/247
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023808 — Adapter les écrans à la tablette (mise en page, zones sûres, barre d'état, options)"
status: In Review
size: L
estimate:
startDate:
endDate:
parent: "#238"
order: 3080
---

## User story

En tant qu'enfant, je veux des écrans lisibles et confortables sur ma tablette, tenue en main ou posée sur une table.

## Dépend de

Fiche 02.

## Comportement attendu

- Mise en page des écrans existants (profils, menu, sujets, séries, leçons, examen, défi chrono, tables, options) à la taille d'une tablette (≈ 8 à 13 pouces) dans l'orientation retenue à la fiche 01 ; vérifier les points d'arrêt de `styles.css` (un seul `@media (max-width: 760px)` aujourd'hui).
- `viewport` : `viewport-fit=cover`, zones sûres (encoche, barre d'accueil) via `env(safe-area-inset-*)`.
- Clavier à l'écran : le champ de réponse et le bouton de validation restent visibles quand le clavier s'affiche (`visualViewport`).
- Barre d'état (`StatusBar.tsx`) et options : retirer ou adapter ce qui est propre au bureau (mention de Piper et de Rosetta, quitter, fenêtre) ; l'état de la voix reflète la voix du système.
- Textes toujours dans `web/src/i18n/fr.ts`.

## Critères d'acceptation

- [ ] Aucun écran n'est tronqué ni n'exige de défilement horizontal, dans l'orientation retenue, sur la tablette de test.
- [ ] Le champ de réponse n'est jamais masqué par le clavier à l'écran.
- [ ] Tests Vitest des écrans modifiés ; parcours Playwright avec un viewport de tablette.
- [ ] Rendu bureau inchangé (comparaison visuelle rapide par Georges).

## Implémentation (2026-10-15)

- `web/index.html` : `viewport-fit=cover`, `maximum-scale=1`, `user-scalable=no`.
- `web/src/styles.css` (section « Tablet ») : zones sûres `env(safe-area-inset-*)` sur `.layout` et sur les éléments placés dans les coins (`.back`, `.sound-toggle`, `.break-reminder`) ; `env()` vaut 0 sur le bureau (rendu inchangé).
- Clavier à l'écran : les champs de réponse n'ouvrent plus le clavier du système sur l'iPad (SUB023810), il ne peut donc plus masquer le champ ; restent au clavier du système le prénom d'un profil et l'espace parent.
- Barre d'état : rien de propre au bureau à retirer (pas de mention de Piper ni de Rosetta) ; voix du système et « voix à installer » (SUB023805).
- Playwright : projet `ipad-landscape` (1 366 × 1 024, tactile) qui rejoue `e2e/lot1.spec.ts`.

## Tests à faire (revue)

1. `cd web && npx playwright install chromium` (une fois), puis `npm run test:e2e` → 9 tests verts, dont 2 du projet `ipad-landscape`.
2. iPad en paysage, chaque écran : Qui joue ?, création de profil, menu, sujets, carte de progression, série, leçon, examen, défi chrono, Mes tables, options, espace parent. Rien de tronqué, aucun défilement horizontal, rien sous la barre d'état (heure, batterie) ni sous la barre d'accueil (trait en bas) ; bouton « ← » et bouton des sons bien visibles dans les coins.
3. Prénom d'un profil (clavier de l'iPad) : le champ reste visible quand le clavier s'ouvre.
4. Bureau (`npm run tauri dev`) : comparaison rapide de 3 ou 4 écrans avec avant (marges, coins, barre d'état) → identiques.
