---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsEkg
rev: 2026-10-15T13:35:20Z
url: https://github.com/georgesp/Educator/issues/258
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB023905 — Reconnaissance de l'écriture Android (ML Kit Digital Ink)
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#239"
order: 4050
---

> **Mise de côté (2026-10-15, ADR 0006)** : le lot tablette part sur l'iPad seul. Fiche retirée du lot, conservée pour un futur lot « tablette Android ».

## User story

En tant qu'enfant sur tablette Android, je veux que mon écriture au stylet soit comprise en français, accents compris, sans connexion.

## Dépend de

Fiches 01 (voie 2 retenue pour Android) et 04. Conditionnelle : à supprimer ou à réécrire selon l'étude.

## Comportement attendu

- Plugin Tauri Kotlin : modèle `fr` de ML Kit Digital Ink Recognition, tracés convertis depuis le contrat de la fiche 04, candidats classés renvoyés au Rust.
- Modèle : téléchargement unique du modèle (taille à noter), puis hors ligne ; plan si le téléchargement est impossible (réseau absent au premier lancement : message au parent, repli au clavier).
- Contexte de reconnaissance (taille de la zone d'écriture, texte déjà écrit) transmis pour de meilleurs résultats.

## Critères d'acceptation

- [ ] Sur la tablette Android de test : reconnaissance des mots d'un jeu de test, taux comparé à celui de l'étude.
- [ ] Fonctionne hors ligne après le premier téléchargement ; état exposé dans `get_status`.
