---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WNM
rev: 2026-10-11T15:22:35Z
url: https://github.com/georgesp/Educator/issues/18
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001402 — Format de la banque de mots
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#14"
order: 350
---

## User story

En tant que concepteur du contenu, je veux une banque de mots structurée par classe et par son afin de générer des exercices d'orthographe adaptés.

## Format

Fichiers `content/words/*.json` validés par `content/schemas/word.schema.json` :

```json
{
  "word": "bateau",
  "grades": ["CP", "CE1"],
  "sounds": ["o"],
  "graphemes": [{ "text": "eau", "sound": "o", "position": 3 }],
  "frequency": 1,
  "category": "nom",
  "audio": "bateau.wav",
  "source": "Liste de fréquence lexicale (eduscol)"
}
```

- `graphemes` : graphies travaillées dans le mot (servent à choisir les lettres à masquer).
- Les **compétences d'orthographe** suivent le format des règles du lot 1 avec `subject: "spelling"` et un filtre sur la banque (classe, son, graphie).

## Critères d'acceptation

- [ ] Schéma documenté ; un mot invalide fait échouer le build.
- [ ] `get_skills` pour l'orthographe (`subject: "spelling"`, `grade: "CP"`) fonctionne comme pour les maths.
- [ ] Test : chaque compétence d'orthographe dispose d'au moins 30 mots.
