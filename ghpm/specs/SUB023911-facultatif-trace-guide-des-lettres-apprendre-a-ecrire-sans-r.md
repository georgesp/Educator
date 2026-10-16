---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsFBg
rev: 2026-10-14T11:29:53Z
url: https://github.com/georgesp/Educator/issues/264
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023911 — Facultatif — tracé guidé des lettres (apprendre à écrire, sans reconnaissance)"
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#239"
order: 4110
---

## User story

En tant qu'enfant de CP, je veux suivre le modèle d'une lettre avec mon stylet pour apprendre à bien la tracer.

## Contexte

Fiche facultative, à arbitrer par Georges après l'étude. Elle donne de la valeur au stylet même si la reconnaissance se révèle insuffisante : aucun texte à reconnaître, seulement la comparaison d'un tracé à un modèle.

## Comportement attendu

- Modèles de lettres (SVG ou listes de points, sens et ordre des traits, lettres cursives du cycle 2) dans `content/`, validés par schéma JSON (une mauvaise définition fait échouer les tests).
- Écran de tracé : flèches de départ, tracé animé de démonstration, suivi du stylet, étoile au lieu d'un verdict ; jamais de message négatif.
- Comparaison tracé / modèle dans le Rust (tolérance généreuse, sens des traits), pas de score affiché à l'enfant.
- Intégré aux leçons d'orthographe ou à un menu « Écrire » (à trancher).

## Critères d'acceptation

- [ ] Alphabet minuscule et chiffres tracables ; tests de la comparaison sur des tracés types.
- [ ] Essais avec un enfant du CP notés dans la fiche.
