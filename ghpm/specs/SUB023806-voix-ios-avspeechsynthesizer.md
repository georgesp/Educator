---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDms
rev: 2026-10-15T19:44:22Z
url: https://github.com/georgesp/Educator/issues/245
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB023806 — Voix iOS — AVSpeechSynthesizer
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#238"
order: 3060
---

## User story

En tant qu'enfant sur iPad, je veux entendre les mots et les consignes en français, clairement.

## Dépend de

Fiches 04 et 05. À laisser de côté si l'iPad n'est pas retenu à la fiche 01.

## Comportement attendu

- Code Swift du plugin : choix de la voix `fr-FR` (la plus qualitative installée), vitesse, synthèse selon le contrat retenu, arrêt immédiat quand une nouvelle lecture démarre.
- Session audio : la voix passe aussi quand le commutateur silencieux est activé (tranché le 2026-10-15 : la voix passe en mode silencieux, session `.playback`, ADR 0005), reprise correcte après une interruption (appel, autre application).
- Détection de l'absence de voix française et message pour le parent.

## Critères d'acceptation

- [x] Sur l'iPad de test : consigne d'un calcul et mot de dictée lus en moins de 0,5 s (mesure notée : 300 à 335 ms jusqu'au WAV, ADR 0005).
- [ ] Une nouvelle lecture coupe la précédente ; la mise en arrière-plan arrête la voix.
- [ ] Sans voix française installée : application utilisable, message clair.

## Implémentation (2026-10-15)

- Swift (`plugins/tauri-plugin-system-voice/ios/Sources/…/SystemVoicePlugin.swift`) : meilleure voix `fr-FR` installée (premium, puis améliorée, puis basique ; jamais `fr-CA`), vitesse = `AVSpeechUtteranceDefaultSpeechRate` × vitesse du cœur, synthèse par `write` en SSML (pause de 400 ms après les deux-points pour la coupe des mots), délai maximal de 15 s.
- Session audio `.playback` / `.spokenAudio` : voix entendue en mode silencieux (vérifié) ; session réactivée à la fin d'une interruption (appel) et au retour au premier plan.
- Une nouvelle lecture coupe la précédente (front, `playVoice`) ; la voix s'arrête quand l'application passe en arrière-plan (`useSpeech.ts`, événement `visibilitychange`, test `background.test.ts`).
- Sans voix française : état `voiceToInstall`, barre d'état « Voix : installe une voix française (Réglages › Accessibilité › Contenu énoncé › Voix) » (`StatusBar.test.tsx`).

## Corrections après relecture (2026-10-15)

- Session audio : `.playback` / `.spokenAudio` avec `.duckOthers` (une musique de l'iPad baisse pendant la voix au lieu d'être coupée).
- Pause SSML de 400 ms après « : » sauf entre deux chiffres (« 12 : 3 » reste une division lue d'un trait) ; test `a_division_gets_no_pause`.
- À vérifier sur l'iPad : lancer une musique (app Musique), puis écouter un mot dans Educator : la musique baisse puis remonte.

## Tests à faire (revue)

1. iPad, mode silencieux activé (Centre de contrôle) : section 5 de `#/dev/voix`, « Consigne » → voix entendue.
2. Arrière-plan : lancer « Consigne — Lente (0,5) », remonter aussitôt à l'écran d'accueil → la voix s'arrête ; revenir dans Educator, relancer un bouton → voix entendue.
3. Interruption : pendant qu'Educator est ouvert, se faire appeler en FaceTime (ou lancer une vidéo dans une autre app), raccrocher / revenir, relancer une lecture → voix entendue.
4. Nouvelle lecture : cliquer « Consigne » puis aussitôt « Mot » → la consigne s'arrête, seul le mot est lu.
5. Sans voix française : non testable sans retirer Thomas ; couvert par `without_a_french_voice_the_parent_is_asked_to_install_one` et `StatusBar.test.tsx`.
