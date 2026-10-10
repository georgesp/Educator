---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WWc
url: https://github.com/georgesp/Educator/issues/25
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001409 — Format des leçons
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#14"
order: 290
---

## User story

En tant que concepteur du contenu, je veux décrire une leçon dans un fichier afin d'ajouter des leçons sans toucher au code.

## Format

Fichiers `content/lessons/{subject}/*.json`, schéma `lesson.schema.json` :

- `id`, `subject`, `grade`, `period`, `skillIds` (compétences liées), `title`, `durationMinutes`.
- `slides[]` : `text` (affiché), `speech` (lu par Piper), `animation` (référence + paramètres, ex. `{ "type": "numberLine", "from": 0, "to": 20, "jumps": [7, 5] }`), `illustration` (SVG optionnel).
- `quiz[]` : 3 à 5 questions (QCM ou saisie), avec explication.
- `sources[]` : références consultées (titre + URL).

## Critères d'acceptation

- [ ] Schéma documenté, validation au build.
- [ ] Une leçon d'exemple complète.
