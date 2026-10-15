---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAzCQM
url: https://github.com/georgesp/Educator/issues/271
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB026704 — Exercices d'écoute indisponibles sans voix
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#267"
order: 590
---

## User story

En tant qu'enfant, je ne veux pas tomber sur une dictée que je ne peux pas entendre.

## Comportement attendu

- Quand la voix est indisponible (`get_status`), les exercices qui reposent sur l'écoute du mot (dictée, « Choisis la bonne écriture ») sont grisés avec un message bienveillant (« La voix ne fonctionne pas sur cet ordinateur : demande à un adulte. ») ; les mots à trous restent possibles (le mot est écrit en partie).
- Un examen d'orthographe sans voix ne propose que des questions jouables sans écoute.
- La barre d'état explique déjà pourquoi la voix manque (inchangé).

## Critères d'acceptation

- [x] Test Vitest avec la voix indisponible : dictée grisée, message affiché, mots à trous possibles.
- [x] Test Rust : composition d'un examen sans les formats d'écoute.

## Implémentation (2026-10-15)

- `ExerciseFormat::needs_listening` (dictée, « Choisis la bonne écriture ») ; contrat `SkillResponse.listeningFormats`.
- Front : l'écran du choix de l'exercice grise ces formats (`BigButton locked`) et affiche « La voix ne fonctionne pas
  sur cet ordinateur : demande à un adulte. » quand `get_status` dit la voix indisponible ; une compétence à un seul
  exercice d'écoute passe par cet écran au lieu d'ouvrir la série.
- Voix indisponible : Piper absent, ou dont le préchauffage a échoué (statut `get_status` alors indisponible, nom de
  la voix gardé ; une synthèse réussie le rétablit).
- Rust : `SeriesService::set_listening` (au démarrage puis après le préchauffage, selon le statut de la voix) ; examens,
  évaluations de niveau et défis ne gardent que les compétences et formats jouables sans écoute
  (`exam::compose_exam(…, Listening)`).
- Tests : Vitest « without the voice, the dictation and the right spelling are greyed, the missing letters stay »,
  « with the voice, every kind of exercise can be chosen » ; Rust `without_the_voice_an_exam_asks_no_listening_question`.
