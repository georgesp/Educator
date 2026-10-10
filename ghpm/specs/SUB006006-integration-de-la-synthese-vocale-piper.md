---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fRQ
url: https://github.com/georgesp/Educator/issues/66
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006006 — Intégration de la synthèse vocale Piper
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#60"
order: 7
---

## User story

En tant qu'enfant, je veux entendre une voix claire en français afin de comprendre les consignes et les dictées, même sans Internet.

## Comportement attendu

- Choix d'une voix française Piper : écoute comparative de 2 ou 3 voix sur des mots et consignes pour enfants, choix documenté dans une ADR (`docs/adr/0002-voix-piper.md`).
- Vérification de la **licence** du binaire Piper et de la voix retenue (usage gratuit, redistribution dans un installeur).
- Binaire Piper + modèle de voix embarqués pour macOS (arm64, x64) et Windows (x64).
- Service `ISpeechSynthesizer` dans `Educator.Content` : texte → fichier WAV, vitesse réglable.

## Critères d'acceptation

- [ ] Synthèse d'une phrase de 10 mots en moins de 1,5 s sur un ordinateur récent.
- [ ] Fonctionne hors ligne sur macOS et Windows.
- [ ] Licences de Piper et de la voix consignées dans `THIRD-PARTY-NOTICES.md`.
