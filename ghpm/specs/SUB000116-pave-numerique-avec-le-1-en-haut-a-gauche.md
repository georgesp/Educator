---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAvwY
rev: 2026-10-11T15:22:49Z
url: https://github.com/georgesp/Educator/issues/77
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000116 — Pavé numérique avec le 1 en haut à gauche
status: Done
size: XS
estimate:
startDate:
endDate:
parent: "#1"
order: 123
---

Source : retours de Georges après les premiers essais du lot 1 (2026-10-11).

## User story

En tant qu'enfant de CP, je veux trouver les chiffres dans l'ordre où je les lis afin de taper ma réponse sans chercher.

## Comportement attendu

- Le pavé numérique de saisie (`NumberPad`) suit l'ordre de lecture, comme un téléphone :

  | | | |
  |---|---|---|
  | 1 | 2 | 3 |
  | 4 | 5 | 6 |
  | 7 | 8 | 9 |
  | Effacer | 0 | Valider |

- Aujourd'hui, l'ordre est celui d'une calculatrice (7 8 9 en haut).
- Le clavier physique continue de fonctionner comme avant.

## Critères d'acceptation

- [x] Le 1 est en haut à gauche, les chiffres se lisent de gauche à droite puis de haut en bas.
- [x] Ordre de tabulation et lecteur d'écran dans le même ordre.
- [x] `NumberPad.test.tsx` vérifie l'ordre des touches.

## Notes techniques

- Constante `DIGITS` de `web/src/components/child/NumberPad.tsx`.

## Implémentation (2026-10-11)

- `DIGITS` de `NumberPad.tsx` suit l'ordre de lecture : 1 2 3 / 4 5 6 / 7 8 9 / Effacer 0 Valider.
- Les touches sont dans l'ordre du DOM : la tabulation et le lecteur d'écran suivent le même ordre.
- `NumberPad.test.tsx` vérifie l'ordre des touches et l'ordre de tabulation. Le clavier physique ne change pas.
