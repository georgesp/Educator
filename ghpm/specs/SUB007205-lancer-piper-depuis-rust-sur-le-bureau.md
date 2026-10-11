---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAv3Q
url: https://github.com/georgesp/Educator/issues/82
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB007205 — Lancer Piper depuis Rust sur le bureau
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#72"
order: 170
---

## User story

En tant qu'enfant, je veux toujours entendre les consignes lues à voix haute, avec la même voix et la même rapidité.

## Comportement attendu

- Portage de `PiperSpeechSynthesizer`, `PiperLocator`, `PiperInstallation`, `SpeechCache`, `UnavailableSpeechSynthesizer` et du préchauffage (`SpeechWarmUpService`).
- Processus Piper longue durée (`--json-input`), relancé si la vitesse change ou s'il s'arrête, comme dans l'ADR 0002.
- Le synthétiseur implémente le trait de `educator-core` ; il est absent sur iOS/Android (compilation conditionnelle), où le synthétiseur « indisponible » est utilisé en attendant le lot tablette.
- `scripts/fetch-piper.*` et l'emplacement du binaire et de la voix inchangés.

## Critères d'acceptation

- [ ] Énoncé d'une addition lu en moins de 0,5 s après le préchauffage sur macOS.
- [x] Le cache audio évite une nouvelle synthèse pour un texte déjà lu.
- [x] Sans Piper, l'application fonctionne et affiche les consignes sans voix, comme aujourd'hui.

## Notes de réalisation

- `educator-content::speech` (compilé seulement hors iOS / Android) : `locate`, `PiperSpeechSynthesizer`
  (processus longue durée, lecture de la sortie standard par un thread, délai de 20 s, relance si la vitesse change ou
  après une erreur), `SpeechCache` (SHA-256 voix + vitesse + texte, 2 000 fichiers au plus), `create_synthesizer`.
- Préchauffage lancé juste après le démarrage de l'application (`state.rs`). Sur tablette : `UnavailableSpeechSynthesizer`.
- Recherche de Piper : `EDUCATOR_PIPER_DIR`, puis `piper/` des ressources Tauri, puis premier `vendor/piper` au-dessus
  de l'exécutable (le dépôt en développement). `scripts/fetch-piper.*` inchangés.
- Protocole vérifié avec un faux moteur (test `piper_protocol_with_a_fake_engine`). La voix n'a pas pu être téléchargée
  dans l'environnement de Claude (Hugging Face refusé) : **le critère des 0,5 s reste à mesurer sur le Mac**
  (`cargo test -p educator-content --test speech -- --nocapture` avec `vendor/piper` installé).
