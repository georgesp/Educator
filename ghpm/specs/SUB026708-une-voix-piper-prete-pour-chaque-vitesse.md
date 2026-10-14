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
status: Todo
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
- Vitesses utilisées (décision du 2026-10-14) : consignes (1), mots dictés **et** leurs phrases d'exemple (0,85, la même vitesse pour que le mot sonne pareil seul et dans la phrase), écoute lente (0,5). `WORD_AUDIO_SPEED` et `EXAMPLE_AUDIO_SPEED` deviennent une seule vitesse des mots.
- Au démarrage : préchauffage des processus « consignes » et « mots ». Le processus « écoute lente » est lancé à la première demande.
- Mémoire : ≈ 110 Mo par processus. Au plus 2 processus restent chauds en permanence ; un processus inutilisé depuis 2 minutes s'arrête (sauf les deux préchauffés).
- Un processus planté ou bloqué est relancé, comme aujourd'hui ; les demandes à une même vitesse restent servies une à la fois.

## Critères d'acceptation

- [ ] Le mot et sa phrase sont lus à la même vitesse (0,85) ; test.
- [ ] Plus aucun redémarrage de Piper lors du passage consigne → mot → consigne (vérifié dans les logs).
- [ ] Mémoire totale mesurée et notée dans la fiche (macOS, Rosetta).
- [ ] Tests : plusieurs vitesses servies sans redémarrage ; arrêt après inactivité (horloge injectée) ; relance après un plantage.
