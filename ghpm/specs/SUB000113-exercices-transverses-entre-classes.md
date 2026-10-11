---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAvsc
url: https://github.com/georgesp/Educator/issues/74
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000113 — Exercices transverses entre classes
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 125
---

Source : retours de Georges après les premiers essais du lot 1 (2026-10-11).

## User story

En tant qu'enfant de CE1, je veux retrouver en début d'année les exercices de la fin du CP afin de reprendre en douceur après les vacances.

## Comportement attendu

- Une compétence peut être proposée à plusieurs classes et périodes, sans être dupliquée.
- Règle par défaut : les compétences de la **période 5 de la classe N** sont aussi proposées en **période 1 de la classe N+1**, regroupées sous un titre « Révisions de l'année dernière ».
- Une compétence peut déclarer des rattachements supplémentaires dans son JSON, par exemple :

  ```json
  "alsoOffered": [ { "grade": "CE1", "period": 1 } ]
  ```

- Dans la liste des compétences, les révisions apparaissent après les compétences de la période en cours, avec une étiquette indiquant leur classe d'origine (« CP »).
- Les points gagnés sur une révision comptent normalement.

## Critères d'acceptation

- [ ] Champ `alsoOffered` ajouté à `skill.schema.json` et chargé par `educator-content` (classe et période validées).
- [ ] En CE1 période 1, la liste contient les compétences CP période 5 en « Révisions », sans doublon.
- [ ] La règle de repli CE2 → CM2 (compétences du CE1 proposées) reste cohérente avec les révisions.
- [ ] Tests du catalogue (trait `SkillCatalog`) pour la règle par défaut et pour `alsoOffered`.

## Notes techniques

- `SkillCatalog::for_grade` filtre aujourd'hui strictement sur la classe : à étendre pour renvoyer aussi les compétences rattachées, avec leur origine.
- À rapprocher de la révision espacée des erreurs (SUB001509, lot 3), qui pourra s'appuyer sur ces rattachements.
