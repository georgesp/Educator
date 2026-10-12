---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wi4
rev: 2026-10-10T19:39:22Z
url: https://github.com/georgesp/Educator/issues/33
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001417 — Mascotte
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#14"
order: 500
---

## User story

En tant qu'enfant, je veux un personnage qui m'accompagne, me présente les leçons et me félicite afin de me sentir encouragé.

## Comportement attendu

- Une mascotte originale, dessinée en SVG **dans le même style** que les personnages des profils (lot 1), distincte d'eux (ex. un hibou ou une chouette, à valider).
- Quelques expressions (content, réfléchit, encourage, fait la fête) et petites animations (Framer Motion).
- Présente : au début des leçons (« Aujourd'hui, on apprend… »), sur l'écran de fin de série (félicitations ou encouragement), lors des montées de niveau (lot 3).
- Ses phrases sont lues par Piper et tirées au hasard dans une liste par situation (`content/mascot/phrases.json`), pour éviter la répétition.

## Critères d'acceptation

- [ ] 4 expressions minimum, rendu homogène avec les personnages.
- [ ] Phrases toujours bienveillantes, jamais moqueuses, relues.
- [ ] La mascotte peut être masquée (Options) si elle gêne.
