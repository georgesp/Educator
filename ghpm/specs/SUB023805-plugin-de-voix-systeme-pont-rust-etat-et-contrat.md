---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDgw
rev: 2026-10-15T20:19:00Z
url: https://github.com/georgesp/Educator/issues/244
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023805 — Plugin de voix système — pont Rust, état et contrat"
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#238"
order: 3050
---

## User story

En tant qu'enfant, je veux entendre les consignes lues à voix haute sur la tablette comme sur l'ordinateur.

## Dépend de

Fiche 04 (contrat retenu).

## Comportement attendu

- Plugin Tauri mobile (`tauri-plugin-*` local au dépôt, sources Swift et Kotlin dans les fiches 06 et 07) exposant au Rust : disponibilité de la voix française, synthèse d'un texte à une vitesse donnée.
- Implémentation du trait `SpeechSynthesizer` (`educator-core`) pour mobile, dans l'application (`web/src-tauri`) ou un module dédié : pas de règle métier, simple adaptateur. Remplace `UnavailableSpeechSynthesizer` sur iOS / Android.
- Le cache de phrases (`SpeechCache`) est conservé s'il sert (contrat WAV) ; normalisation du texte inchangée.
- `get_status` : l'état de la voix explique le cas « voix française absente » (« Installe la voix française dans les réglages de la tablette ») ; l'application fonctionne sans son, comme sans Piper aujourd'hui.
- Préchauffage au démarrage si le moteur du système en bénéficie.
- Le mot d'une dictée n'est jamais envoyé au front avant la réponse (`speak_series_word`), comme sur le bureau.

## Implémentation (2026-10-15)

- Plugin `plugins/tauri-plugin-system-voice/` (Swift : voix `fr-FR`, `AVSpeechSynthesizer.write`, SSML avec pause après les deux-points, session `.playback` ; Rust : appels, modèles, WAV) et adaptateur `web/src-tauri/src/system_voice.rs` (trait `VoiceBridge`, faux pont dans les tests). Contrat WAV retenu (ADR 0005).
- `get_status` : champ `voiceToInstall` (`SpeechEngineStatus::voice_missing`) ; la barre d'état affiche « Voix : installe une voix française (Réglages › Accessibilité › Contenu énoncé › Voix) ».
- Pas de cache des phrases sur l'iPad pour l'instant : WAV reçu en 300 à 335 ms, mots des dictées préparés à l'avance (`SpeechPreparer`).
- Préchauffage : « Bonjour. » lu au démarrage, liste des voix dans les logs.
- Tests : plugin (9 tests : WAV, SSML, réponses du Swift) ; adaptateur avec un faux pont (6 tests) ; `readings_answer_503_without_a_voice` et `a_word_alone_is_cut_after_its_carrier_and_its_sentence_is_read_whole` (`speak_word` suit désormais le chemin des dictées).

## Critères d'acceptation

- [ ] Tests unitaires du pont avec un faux moteur (sans tablette), dans le style de `piper_protocol_with_a_fake_engine`.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` vert ; compilation bureau inchangée.
- [ ] Les commandes `speak`, `speak_word` et `speak_series_word` répondent `503` quand la voix est indisponible.

## Corrections après relecture (2026-10-15)

- La barre d'état affiche le nom seul de la voix (« Thomas ») : la qualité (« default », « enhanced »…) est en anglais.
- `status()` relit l'état de la voix et met à jour les examens (`set_listening`) à chaque appel : une voix française installée pendant que l'application tourne est prise en compte sans redémarrer.
- À vérifier : `cargo test -p educator-shell` (test `the_status_names_the_voice_chosen_by_the_ipad`) ; `npm test` (`StatusBar.test.tsx`).

## Tests à faire (revue)

1. Racine : `cargo fmt --all --check && cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings` → tout vert. Tests à repérer : `system_voice::tests` (6), `tauri-plugin-system-voice` (9), `readings_answer_503_without_a_voice`, `a_word_alone_is_cut_after_its_carrier_and_its_sentence_is_read_whole`.
2. Si `bindings::types_ts_is_up_to_date` échoue : `cd web && npm run api:generate`, vérifier que `web/src/api/types.ts` ne diffère que par le champ `voiceToInstall`, puis commiter.
3. `cd web && npm run build && npm test` → vert (dont `StatusBar.test.tsx`, 3 tests).
4. iPad (`npm run tauri ios dev`) : barre d'état « Voix : Thomas » ; page `#/dev/voix`, section 5 : chaque bouton « joué », WAV reçu en moins de 0,5 s.
5. Bureau (`npm run tauri dev`) : la voix Piper marche comme avant (consigne d'un calcul, mot d'une dictée).
