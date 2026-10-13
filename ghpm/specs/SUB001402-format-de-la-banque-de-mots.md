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
status: Done
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

- [x] Schéma documenté ; un mot invalide fait échouer le build.
- [x] `get_skills` pour l'orthographe (`subject: "spelling"`, `grade: "CP"`) fonctionne comme pour les maths.
- [x] Test : chaque compétence d'orthographe dispose d'au moins 30 mots.

## Implémentation (2026-10-14)

- Mots regroupés par thème dans `content/words/*.json` (un tableau `words` par fichier), validés par
  `content/schemas/words.schema.json` ; champ `example` (phrase courte) pour lever les homophones (SUB001404).
- Sons et graphies dans `content/spelling/sounds.json` (`sounds.schema.json`) : graphies de chaque son et
  confusions fréquentes, utilisées pour les propositions (SUB001407, SUB001408).
- Compétences d'orthographe dans `content/skills/spelling/*.json` (`subject: "spelling"`, filtre sur la classe,
  le son ou la graphie) ; `get_skills` les sert comme celles de maths.
- Le chargement refuse un mot invalide (position de graphie fausse, son inconnu) : les tests échouent.
- Test `every_spelling_skill_has_at_least_thirty_words` (`content_validation.rs`).
