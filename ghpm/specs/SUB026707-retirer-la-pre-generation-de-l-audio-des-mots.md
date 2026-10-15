---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAzCdw
rev: 2026-10-14T19:31:05Z
url: https://github.com/georgesp/Educator/issues/274
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB026707 — Retirer la pré-génération de l'audio des mots
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#267"
order: 580
---

## User story

En tant que développeur, je veux un seul chemin pour la voix, sans étape de build à ne pas oublier.

## Comportement attendu

- Suppression de l'exemple `generate_word_audio`, des scripts `scripts/generate-word-audio.sh` / `.ps1`, du dossier `content/audio/` (et de sa ligne dans `.gitignore`), du manifeste et de la recherche de fichiers pré-générés dans `services.rs` (`word_audio`).
- `WORD_AUDIO_SPEED` et `EXAMPLE_AUDIO_SPEED` deviennent les vitesses de la voix pour les mots et les phrases.
- Mises à jour : README (section audio des mots), `CLAUDE.md` (structure, commandes), ADR 0002 (renvoi vers l'ADR 0004), fiche SUB001404 (note : remplacée par ce lot).
- Sans voix (Piper absent ou en panne), les exercices qui font écouter un mot ne peuvent pas se faire : voir la fiche « Exercices d'écoute indisponibles sans voix ».

## Critères d'acceptation

- [x] Plus aucune référence à l'audio pré-généré dans le code, les scripts et la documentation (recherche `generate-word-audio`, `audio/words`).
- [x] L'installeur ne contient plus `content/audio`.

## Implémentation (2026-10-15)

- Supprimés : `crates/educator-content/examples/generate_word_audio.rs`, `scripts/generate-word-audio.sh` / `.ps1`,
  le dossier `content/audio/` et sa ligne dans `.gitignore`, `word_audio_file`, `WORD_AUDIO_FOLDER`,
  `WORD_AUDIO_MANIFEST`, la recherche de fichiers pré-générés dans `services.rs`.
- `WORD_AUDIO_SPEED` / `EXAMPLE_AUDIO_SPEED` remplacées par `speech::speed::WORDS` (SUB026708).
- Mis à jour : README, `CLAUDE.md`, ADR 0002 (renvoi vers l'ADR 0004), fiche SUB001404 (note de remplacement),
  fiche SUB023803 (plus de WAV à exclure du paquet mobile).
- Recherche `generate-word-audio`, `audio/words` : ne reste que dans l'historique (fiches du lot 2bis, SUB001404,
  ADR 0004).
