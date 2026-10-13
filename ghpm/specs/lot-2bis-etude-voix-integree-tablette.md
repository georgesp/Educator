---
type: Issue
title: Étude — moteur de voix intégré pour la tablette
status: Todo
size: S
parent: specs/lot-2bis-voix-en-direct.md
order: 509
---

## User story

En tant que parent, je veux que la future version tablette ait la même voix que l'ordinateur.

## Contexte

Sur iOS, une application ne peut pas lancer Piper comme processus annexe. La voix en direct (consignes et mots) demande donc un moteur intégré à l'application, ce qui supprimerait aussi le besoin de Rosetta sur Mac.

## Questions à trancher

- Moteur : Piper intégré en Rust (modèle ONNX via `onnxruntime` / crate `ort`), ou `sherpa-onnx` qui sait lire les voix Piper ; taille ajoutée, performance sur iPad.
- Phonétisation : Piper s'appuie sur **espeak-ng, sous licence GPL-3**. Compatibilité avec la distribution sur l'App Store à vérifier ; alternatives (autre phonétiseur, dictionnaire de prononciation, voix système `AVSpeechSynthesizer` pour la tablette seulement).
- Voix plus légère pour la tablette (`fr_FR-siwis-low`) : qualité à écouter.

## Livrable

- ADR « Voix sur tablette » avec la recommandation, les mesures et les licences ; fiche(s) pour le lot tablette.

## Critères d'acceptation

- [ ] Prototype mesuré : temps de synthèse d'un mot et d'une phrase, mémoire, taille ajoutée.
- [ ] Question de licence espeak-ng / App Store documentée avec ses sources.
