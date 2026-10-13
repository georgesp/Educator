---
type: Issue
title: Exercices d'écoute indisponibles sans voix
status: Todo
size: S
parent: specs/lot-2bis-voix-en-direct.md
order: 508
---

## User story

En tant qu'enfant, je ne veux pas tomber sur une dictée que je ne peux pas entendre.

## Comportement attendu

- Quand la voix est indisponible (`get_status`), les exercices qui reposent sur l'écoute du mot (dictée, « Choisis la bonne écriture ») sont grisés avec un message bienveillant (« La voix ne fonctionne pas sur cet ordinateur : demande à un adulte. ») ; les mots à trous restent possibles (le mot est écrit en partie).
- Un examen d'orthographe sans voix ne propose que des questions jouables sans écoute.
- La barre d'état explique déjà pourquoi la voix manque (inchangé).

## Critères d'acceptation

- [ ] Test Vitest avec la voix indisponible : dictée grisée, message affiché, mots à trous possibles.
- [ ] Test Rust : composition d'un examen sans les formats d'écoute.
