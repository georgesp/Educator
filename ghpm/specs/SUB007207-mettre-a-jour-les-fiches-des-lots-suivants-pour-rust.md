---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAv6E
rev: 2026-10-11T15:23:00Z
url: https://github.com/georgesp/Educator/issues/84
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB007207 — Mettre à jour les fiches des lots suivants pour Rust
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#72"
order: 320
---

## User story

En tant que développeur, je veux des fiches qui décrivent la stack réelle afin de ne pas repartir sur du C# au lot 2.

## Comportement attendu

- Remplacer les références à .NET dans les fiches suivantes :
  - SUB001401 (enregistrement des séances) : « Migration EF Core » → migration `educator-data` ;
  - SUB001405 (validation orthographique) : « `Educator.Core` » → crate `educator-core` ;
  - SUB006008 (installeurs) : retirer .NET des prérequis à vérifier.
- Mettre à jour la description d'architecture des fiches de lot (US0060, US0001) si besoin, en indiquant qu'elle est remplacée par le lot 1bis.

## Critères d'acceptation

- [x] Plus aucune fiche ouverte ne mentionne .NET, C#, EF Core ou les projets `Educator.*`.

## Notes de réalisation

- SUB001401 (migration `educator-data`), SUB001405 (crate `educator-core`), SUB006008 et US0069 (plus de .NET dans
  les installeurs, contenu en ressource Tauri), SUB001513 (« host » → application) mis à jour.
- US0060 et US0001 : section « Lot 1bis — migration vers Rust » ajoutée.
- Fiches des lots 0 et 1 réécrites pour la version Rust (types, commandes, tests), avec une mention de la première
  version .NET ; celles dont tous les critères sont remplis sont passées à `Done`.
