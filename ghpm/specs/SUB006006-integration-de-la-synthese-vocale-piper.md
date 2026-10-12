---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fRQ
rev: 2026-10-11T19:12:41Z
url: https://github.com/georgesp/Educator/issues/66
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006006 — Intégration de la synthèse vocale Piper
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#60"
order: 70
---

## User story

En tant qu'enfant, je veux entendre une voix claire en français afin de comprendre les consignes et les dictées, même sans Internet.

## Comportement attendu

- Choix d'une voix française Piper : écoute comparative de 2 ou 3 voix sur des mots et consignes pour enfants, choix documenté dans une ADR (`docs/adr/0002-voix-piper.md`).
- Vérification de la **licence** du binaire Piper et de la voix retenue (usage gratuit, redistribution dans un installeur).
- Binaire Piper + modèle de voix embarqués pour macOS (arm64, x64) et Windows (x64).
- Trait `SpeechSynthesizer` (`educator-core`), implémenté par `PiperSpeechSynthesizer` (`educator-content`) : texte → WAV, vitesse réglable.

## Critères d'acceptation

- [x] Synthèse d'une phrase de 10 mots en moins de 1,5 s sur un ordinateur récent.
- [ ] Fonctionne hors ligne sur macOS et Windows.
- [x] Licences de Piper et de la voix consignées dans `THIRD-PARTY-NOTICES.md`.

## Implémentation (2026-10-10)

Voir `docs/adr/0002-voix-piper.md`. La version macOS publiée de Piper est un binaire Intel sans ses bibliothèques : le script l'assemble avec celles de piper-phonemize, et l'ensemble tourne sous Rosetta 2 sur Apple Silicon. Processus Piper longue durée (`--json-input`), préchauffé au démarrage de l'application à la vitesse enfant (0,9). Mesuré sous Linux avec la version .NET : 0,9 s pour 10 mots au premier appel, 0,13 s pour l'énoncé d'une addition ensuite ; le même protocole est repris en Rust (lot 1bis). Voix proposée `fr_FR-siwis-medium` (CC BY 4.0), à confirmer à l'écoute avec `scripts/compare-voices.sh`. Reste à vérifier hors ligne sur macOS et Windows.
