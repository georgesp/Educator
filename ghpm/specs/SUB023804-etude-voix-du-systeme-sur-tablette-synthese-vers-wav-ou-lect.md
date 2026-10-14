---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDYw
url: https://github.com/georgesp/Educator/issues/243
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB023804 — Étude — voix du système sur tablette (synthèse vers WAV ou lecture directe)
status: Todo
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

- Synthèse vers un tampon audio : `AVSpeechSynthesizer.write(_:toBufferCallback:)` sur iOS (formats de tampon, comportement connu sur simulateur, voix « améliorées »), `TextToSpeech.synthesizeToFile` sur Android. Faisable et assez fiable pour reconstruire un WAV 16 bits mono ?
- Sinon : lecture directe par le système (`speak` natif) avec un contrat différent : commande « lire » plutôt que « fournir le WAV », arrêt, fin de lecture notifiée au front.
- Vitesse : l'écoute lente (fiche « Écouter le mot plus lentement » du lot 2bis, 0,5×) est-elle naturelle avec le débit du système ?
- Voix française : disponibilité par défaut, voix à télécharger, qualité ; que faire si elle est absente (message au parent, voir fiche 05) ?
- Latence d'un mot et d'une phrase ; hors ligne.

## Livrable

- Section « Voix » de l'ADR 0005 avec la recommandation, les mesures et le contrat retenu ; ajustement des fiches 05 à 07 si la lecture directe est préférée.

## Critères d'acceptation

- [ ] Prototype essayé sur iPad et/ou Android selon la fiche 01 : latence d'un mot et d'une phrase mesurée.
- [ ] Écoute de contrôle par Georges (qualité, vitesse 0,5×) notée dans l'ADR.
