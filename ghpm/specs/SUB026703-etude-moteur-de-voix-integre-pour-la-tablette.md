---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAzCME
rev: 2026-10-15T09:32:39Z
url: https://github.com/georgesp/Educator/issues/270
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB026703 — Étude — moteur de voix intégré pour la tablette
status: In progress
size: S
estimate:
startDate:
endDate:
parent: "#239"
order: 600
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
- [x] Question de licence espeak-ng / App Store documentée avec ses sources.

## Implémentation (2026-10-15)

- Étude documentaire : `docs/adr/0005-voix-sur-tablette.md` (options, licence espeak-ng / App Store avec sources,
  recommandation : voix du système au lot tablette 1, sherpa-onnx ensuite).
- Prototype sherpa-onnx mesuré dans l'environnement de Claude (Linux x64) avec la voix `siwis-medium` : même rendu
  et mêmes temps que Piper ; données espeak-ng du français réduites à 0,8 Mo.
- Reste : prototype mesuré **sur iPad** (temps d'un mot et d'une phrase, mémoire, taille ajoutée), écoute de
  `fr_FR-siwis-low` ; fiche(s) à créer dans le lot tablette.

## Suite (2026-10-15, lot tablette 1)

- La partie « tablette » est traitée par le lot tablette 1 avec la **voix du système** (SUB023804, ADR 0005) : mesurée sur l'iPad, contrat WAV retenu. US0238 prévoit de clôturer cette étude une fois le lot lancé.
- Non fait : le prototype sherpa-onnx sur iPad (temps, mémoire, taille) et l'écoute de `fr_FR-siwis-low`, utiles seulement si la voix Piper est voulue sur la tablette (fiche à créer alors).
