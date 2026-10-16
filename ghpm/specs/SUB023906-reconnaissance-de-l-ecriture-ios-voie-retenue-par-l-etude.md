---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsEpc
rev: 2026-10-14T11:29:39Z
url: https://github.com/georgesp/Educator/issues/259
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB023906 — Reconnaissance de l'écriture iOS (voie retenue par l'étude)
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#239"
order: 4060
---

## User story

En tant qu'enfant sur iPad, je veux que mon écriture à l'Apple Pencil soit comprise en français, accents compris, sans connexion.

## Dépend de

Fiches 01 et 04. Conditionnelle et à préciser après l'étude : il n'existe pas d'API publique iOS de reconnaissance d'encre. Selon l'ADR 0006 : (a) rester sur Scribble (fiche 03, cette fiche est alors supprimée), (b) reconnaissance sur l'image du tracé avec Vision, (c) modèle embarqué (Core ML ou ONNX) partagé avec Android.

## Comportement attendu

- Implémentation du contrat de la fiche 04 pour la voie retenue ; plugin Swift ou module Rust selon la voie.
- Hors ligne ; poids et licence du modèle notés dans `THIRD-PARTY-NOTICES.md` s'il y en a un.

## Critères d'acceptation

- [ ] Sur l'iPad de test : reconnaissance des mots du jeu de test, taux comparé à celui de l'étude.
- [ ] Taille ajoutée à l'application mesurée et notée dans l'ADR 0006.
