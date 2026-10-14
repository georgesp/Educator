---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDgw
url: https://github.com/georgesp/Educator/issues/244
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023805 — Plugin de voix système — pont Rust, état et contrat"
status: Todo
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

## Critères d'acceptation

- [ ] Tests unitaires du pont avec un faux moteur (sans tablette), dans le style de `piper_protocol_with_a_fake_engine`.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` vert ; compilation bureau inchangée.
- [ ] Les commandes `speak`, `speak_word` et `speak_series_word` répondent `503` quand la voix est indisponible.
