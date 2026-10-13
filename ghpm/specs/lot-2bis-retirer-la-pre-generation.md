---
type: Issue
title: Retirer la pré-génération de l'audio des mots
status: Todo
size: S
parent: specs/lot-2bis-voix-en-direct.md
order: 507
---

## User story

En tant que développeur, je veux un seul chemin pour la voix, sans étape de build à ne pas oublier.

## Comportement attendu

- Suppression de l'exemple `generate_word_audio`, des scripts `scripts/generate-word-audio.sh` / `.ps1`, du dossier `content/audio/` (et de sa ligne dans `.gitignore`), du manifeste et de la recherche de fichiers pré-générés dans `services.rs` (`word_audio`).
- `WORD_AUDIO_SPEED` et `EXAMPLE_AUDIO_SPEED` deviennent les vitesses de la voix pour les mots et les phrases.
- Mises à jour : README (section audio des mots), `CLAUDE.md` (structure, commandes), ADR 0002 (renvoi vers l'ADR 0004), fiche SUB001404 (note : remplacée par ce lot).
- Sans voix (Piper absent ou en panne), les exercices qui font écouter un mot ne peuvent pas se faire : voir la fiche « Exercices d'écoute indisponibles sans voix ».

## Critères d'acceptation

- [ ] Plus aucune référence à l'audio pré-généré dans le code, les scripts et la documentation (recherche `generate-word-audio`, `audio/words`).
- [ ] L'installeur ne contient plus `content/audio`.
