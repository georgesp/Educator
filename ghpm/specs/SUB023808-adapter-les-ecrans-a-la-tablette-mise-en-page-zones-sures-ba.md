---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDzI
url: https://github.com/georgesp/Educator/issues/247
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023808 — Adapter les écrans à la tablette (mise en page, zones sûres, barre d'état, options)"
status: Todo
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
