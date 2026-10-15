---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDYw
rev: 2026-10-15T18:20:47Z
url: https://github.com/georgesp/Educator/issues/243
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB023804 — Étude — voix du système sur tablette (synthèse vers WAV ou lecture directe)
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#238"
order: 3040
---

## User story

En tant que parent, je veux que la tablette lise les consignes et les mots de dictée avec une voix française de qualité correcte, sans rien installer de compliqué.

## Contexte

Aujourd'hui le contrat est : le Rust renvoie des **octets WAV** (`SpeechSynthesizer::synthesize`, commande `speak`), le front les joue avec `Audio` (`web/src/audio/useSpeech.ts`), et le mot d'une dictée n'est jamais envoyé au front avant la réponse (`speak_series_word`). Conserver ce contrat laisserait le front, le cache et la sécurité de la dictée inchangés.

## Questions à trancher (par prototype sur tablette)

- Synthèse vers un tampon audio : `AVSpeechSynthesizer.write(_:toBufferCallback:)` sur iPadOS (formats de tampon, comportement connu sur simulateur, voix « améliorées »). Faisable et assez fiable pour reconstruire un WAV 16 bits mono ?
- Sinon : lecture directe par le système (`speak` natif) avec un contrat différent : commande « lire » plutôt que « fournir le WAV », arrêt, fin de lecture notifiée au front.
- Vitesse : l'écoute lente (fiche « Écouter le mot plus lentement » du lot 2bis, 0,5×) est-elle naturelle avec le débit du système ?
- Voix française : disponibilité par défaut, voix à télécharger, qualité ; que faire si elle est absente (message au parent, voir fiche 05) ?
- Latence d'un mot et d'une phrase ; hors ligne.

## Livrable

- Section « Voix » de l'ADR 0005 avec la recommandation, les mesures et le contrat retenu ; ajustement des fiches 05 à 07 si la lecture directe est préférée.

## Critères d'acceptation

- [x] Prototype essayé sur l'iPad de test (iPad Air 13 pouces M2, iPadOS 27) : latence d'un mot et d'une phrase mesurée (voix du système par l'API Web, ADR 0005).
- [x] Lecture du WAV par le front après l'appel asynchrone vérifiée dans la vue web de l'iPad (risque noté dans l'ADR 0006) : jouée.
- [x] Écoute de contrôle par Georges (qualité, vitesse 0,5×) notée dans l'ADR.
- [x] `AVSpeechSynthesizer.write` essayé en natif (WAV fiable ?) : décide entre le contrat WAV et la lecture native. **Contrat WAV retenu** (ADR 0005).

## Implémentation (2026-10-15)

- Page de développement `#/dev/voix` (`web/src/screens/VoiceLabScreen.tsx`, route `dev/voix` seulement en développement) : voix de l'API Web, WAV joué après un délai, puis voix native par le cœur (section 5).
- Mesures et décision dans l'ADR 0005 : premier son de la voix du système en 4 à 39 ms ; WAV joué après un appel asynchrone ; `AVSpeechSynthesizer.write` fiable, WAV reçu en 300 à 335 ms ; voix entendue en mode silencieux (session `.playback`) ; **contrat WAV retenu**, lecture native écartée.
- À retirer en fin de lot : la page `#/dev/voix` et sa route.
