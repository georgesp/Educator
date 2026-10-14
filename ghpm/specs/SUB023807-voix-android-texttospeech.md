---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsDtg
url: https://github.com/georgesp/Educator/issues/246
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB023807 — Voix Android — TextToSpeech
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#238"
order: 3070
---

## User story

En tant qu'enfant sur tablette Android, je veux entendre les mots et les consignes en français, clairement.

## Dépend de

Fiches 04 et 05. À laisser de côté si Android n'est pas retenu à la fiche 01.

## Comportement attendu

- Code Kotlin du plugin : initialisation asynchrone de `TextToSpeech`, langue `fr-FR`, vitesse, synthèse selon le contrat retenu, arrêt à la demande.
- Moteur de synthèse du système (Google, Samsung…) : comportement selon le constructeur, voix hors ligne disponibles ; message au parent quand la langue ou les données vocales manquent (avec renvoi vers les réglages de la tablette).
- Libération propre du moteur à la fermeture de l'application.

## Critères d'acceptation

- [ ] Sur la tablette Android de test : consigne d'un calcul et mot de dictée lus en moins de 0,5 s (mesure notée), hors ligne.
- [ ] Une nouvelle lecture coupe la précédente ; la mise en arrière-plan arrête la voix.
- [ ] Sans données vocales françaises : application utilisable, message clair.
