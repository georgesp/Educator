---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WVI
rev: 2026-10-10T19:39:13Z
url: https://github.com/georgesp/Educator/issues/24
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001408 — Exercice « Choisis la bonne écriture »
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#14"
order: 410
---

## User story

En tant qu'enfant, je veux reconnaître le mot bien écrit parmi plusieurs afin de mémoriser son orthographe visuelle.

## Comportement attendu

- Le mot est lu ; 3 écritures sont proposées (« bato / bateau / batau »), une seule est correcte.
- Les écritures fausses sont générées en remplaçant une graphie par une graphie concurrente du même son.

## Critères d'acceptation

- [x] Les propositions fausses ne sont jamais des mots réels de la banque.
- [x] Tests du générateur.

## Implémentation (2026-10-14)

- Format `chooseSpelling` : le mot est lu ; 3 écritures, une seule juste.
- Écritures fausses : une graphie remplacée par une graphie concurrente du même son ; à défaut (son écrit d'une
  seule façon), par une confusion fréquente, puis par une consonne doublée ou dédoublée (décision de Georges du
  2026-10-14 : on garde les confusions fréquentes). Jamais un mot réel de la
  banque (test sur toute la banque : `spelling_proposals_are_never_real_words_and_compete_on_the_same_sound`).
