---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDms
url: https://github.com/georgesp/Educator/issues/245
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB023806 — Voix iOS — AVSpeechSynthesizer
status: Todo
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
- Session audio : la voix passe aussi quand le commutateur silencieux est activé (à trancher : respecter ou non le mode silencieux), reprise correcte après une interruption (appel, autre application).
- Détection de l'absence de voix française et message pour le parent.

## Critères d'acceptation

- [ ] Sur l'iPad de test : consigne d'un calcul et mot de dictée lus en moins de 0,5 s (mesure notée).
- [ ] Une nouvelle lecture coupe la précédente ; la mise en arrière-plan arrête la voix.
- [ ] Sans voix française installée : application utilisable, message clair.
