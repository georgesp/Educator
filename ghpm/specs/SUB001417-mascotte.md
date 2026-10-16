---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wi4
rev: 2026-10-16T19:32:37Z
url: https://github.com/georgesp/Educator/issues/33
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001417 — Mascotte
status: Done
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

- [x] 4 expressions minimum, rendu homogène avec les personnages.
- [ ] Phrases toujours bienveillantes, jamais moqueuses, relues.
- [x] La mascotte peut être masquée (Options) si elle gêne.

## Implémentation (2026-10-14)

- Plume, une chouette originale dans le style des personnages : 4 expressions (`content/mascot/*.svg` : content,
  réfléchit, encourage, fait la fête), petites animations Framer Motion (`Mascot`).
- Début des leçons, fin des séries, des examens et des défis ; montée de niveau prête pour le lot 3.
- Phrases dans `content/mascot/mascot.json` (validé par `mascot.schema.json`), tirées au hasard par situation,
  jamais deux fois de suite la même, lues par Piper ; formulées sans accord masculin ou féminin.
- Reste : relecture des phrases par Georges.
