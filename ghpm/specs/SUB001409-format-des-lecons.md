---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WWc
rev: 2026-10-14T06:19:58Z
url: https://github.com/georgesp/Educator/issues/25
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001409 — Format des leçons
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#14"
order: 420
---

## User story

En tant que concepteur du contenu, je veux décrire une leçon dans un fichier afin d'ajouter des leçons sans toucher au code.

## Format

Fichiers `content/lessons/{subject}/*.json`, schéma `lesson.schema.json` :

- `id`, `subject`, `grade`, `period`, `skillIds` (compétences liées), `title`, `durationMinutes`.
- `slides[]` : `text` (affiché), `speech` (lu par Piper), `animation` (référence + paramètres, ex. `{ "type": "numberLine", "from": 0, "to": 20, "jumps": [7, 5] }`).
- `quiz[]` : 3 à 5 questions (QCM ou saisie), avec explication.
- `sources[]` : références consultées (titre + URL).

## Critères d'acceptation

- [x] Schéma documenté, validation au build.
- [x] Une leçon d'exemple complète.

## Implémentation (2026-10-14)

- Format complété au lot 2 pour l'orthographe : leçons `content/lessons/spelling/*.json`, réponses de quiz en
  mots (sans tolérance sur les accents), animations `letterTiles` et `soundSort`.
- Le chargement refuse une question de quiz qui affiche le mot à écrire : il doit être entendu, pas recopié.
- Champ `illustration` (SVG facultatif d'une slide) retiré du format le 2026-10-14 (décision de Georges) : les
  animations suffisent.
