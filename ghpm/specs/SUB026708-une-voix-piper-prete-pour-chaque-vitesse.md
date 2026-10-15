---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAzCi0
rev: 2026-10-14T19:30:50Z
url: https://github.com/georgesp/Educator/issues/275
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB026708 — Une voix Piper prête pour chaque vitesse
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#267"
order: 520
---

## User story

En tant qu'enfant, je veux entendre la consigne puis le mot à écrire sans attendre, même si le mot est lu plus lentement.

## Comportement attendu

- `PiperSpeechSynthesizer` (`educator-content::speech`) garde **un processus Piper par vitesse** au lieu d'un seul : passer de la consigne (vitesse normale) au mot dicté (ralenti) ne redémarre plus Piper.
- Vitesses utilisées (décision du 2026-10-14) : consignes (0,9 — inchangée, décision du 2026-10-15), mots dictés **et** leurs phrases d'exemple (0,85, la même vitesse pour que le mot sonne pareil seul et dans la phrase), écoute lente (0,5). `WORD_AUDIO_SPEED` et `EXAMPLE_AUDIO_SPEED` deviennent une seule vitesse des mots.
- Au démarrage : préchauffage des processus « consignes » et « mots ». Le processus « écoute lente » est lancé à la première demande.
- Mémoire : ≈ 110 Mo par processus. Au plus 2 processus restent chauds en permanence ; un processus inutilisé depuis 2 minutes s'arrête (sauf les deux préchauffés).
- Un processus planté ou bloqué est relancé, comme aujourd'hui ; les demandes à une même vitesse restent servies une à la fois.

## Critères d'acceptation

- [x] Le mot et sa phrase sont lus à la même vitesse (0,85) ; test.
- [ ] Plus aucun redémarrage de Piper lors du passage consigne → mot → consigne (vérifié dans les logs).
- [ ] Mémoire totale mesurée et notée dans la fiche (macOS, Rosetta).
- [x] Tests : plusieurs vitesses servies sans redémarrage ; arrêt après inactivité (horloge injectée) ; relance après un plantage.

## Implémentation (2026-10-15)

- `PiperSpeechSynthesizer` garde un processus par vitesse (`length_scale` arrondi au centième) ; les demandes d'une
  même vitesse passent une à une, deux vitesses travaillent en parallèle. `warm_up` démarre le processus d'une vitesse
  et le garde chaud ; au démarrage de l'application : consignes (0,9) et mots (0,85). Un fil `piper-idle` arrête
  toutes les 15 s les autres processus inutilisés depuis 2 min (`IDLE_TIMEOUT`, horloge injectée :
  `PiperSpeechSynthesizer::with_clock`, `stop_idle_processes`).
- Vitesses dans `educator_core::speech::speed` : `INSTRUCTIONS` 0,9 (décision du 2026-10-15 : les consignes restent
  à 0,9), `WORDS` 0,85 (mot et phrase), `SLOW` 0,5. `WORD_AUDIO_SPEED` et `EXAMPLE_AUDIO_SPEED` sont retirées.
- Tests (`crates/educator-content/tests/speech.rs`) avec un faux Piper : `several_speeds_are_served_without_restarting_piper`,
  `an_idle_process_stops_unless_it_is_kept_warm`, `a_crashed_process_is_restarted` ; avec le vrai Piper quand il est
  installé : `piper_serves_several_speeds_without_restarting` (3 processus pour 3 vitesses, aucun redémarrage).
  Test de la vitesse du mot et de sa phrase : `the_word_and_its_sentence_are_read_at_the_same_speed` (`educator-core`).
- Mémoire mesurée dans l'environnement de Claude (Linux x64) : ≈ 105 Mo par processus.
- Reste (Georges) : vérifier dans les logs l'absence de « Piper started » au passage consigne → mot → consigne, et
  mesurer la mémoire totale des processus `piper` sur le Mac (Moniteur d'activité).
