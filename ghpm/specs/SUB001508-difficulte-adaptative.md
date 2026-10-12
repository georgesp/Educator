---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WsU
rev: 2026-10-10T19:39:32Z
url: https://github.com/georgesp/Educator/issues/41
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001508 — Difficulté adaptative
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#15"
order: 590
---

## User story

En tant qu'enfant, je veux des exercices ni trop faciles ni trop difficiles afin de progresser sans me décourager.

## Comportement attendu

- Niveau de départ : classe + période en cours.
- Ajustement selon la maîtrise (`SkillMastery`) : au-dessus de ~85 % de réussite récente, proposer la compétence suivante ou des paramètres plus difficiles ; en dessous de ~60 %, revenir à la compétence précédente.
- Proposition « Je te conseille… » sur l'écran Exercices, l'enfant garde le choix.
- Seuils en fichier de contenu.

## Critères d'acceptation

- [ ] Algorithme testé sur des historiques simulés.
- [ ] L'enfant n'est jamais bloqué : toutes les compétences restent accessibles.
