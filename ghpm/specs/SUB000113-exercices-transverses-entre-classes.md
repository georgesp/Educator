---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAvsc
rev: 2026-10-11T16:02:51Z
url: https://github.com/georgesp/Educator/issues/74
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000113 — Exercices transverses entre classes
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 250
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

- [x] Champ `alsoOffered` ajouté à `skill.schema.json` et chargé par `educator-content` (classe et période validées).
- [x] En CE1 période 1, la liste contient les compétences CP période 5 en « Révisions », sans doublon.
- [x] La règle de repli CE2 → CM2 (compétences du CE1 proposées) reste cohérente avec les révisions.
- [x] Tests du catalogue (trait `SkillCatalog`) pour la règle par défaut et pour `alsoOffered`.

## Notes techniques

- `SkillCatalog::for_grade` filtre aujourd'hui strictement sur la classe : à étendre pour renvoyer aussi les compétences rattachées, avec leur origine.
- À rapprocher de la révision espacée des erreurs (SUB001509, lot 3), qui pourra s'appuyer sur ces rattachements.

## Implémentation (2026-10-11)

- `alsoOffered` est ajouté à `skill.schema.json` et chargé par `educator-content`. La classe doit être connue et
  différente de celle de la compétence, une seule fois par classe ; la période va de 1 à 5.
- `SkillCatalog::offered_to` (`educator-core`) renvoie les compétences de la classe, puis en révision :
  - les compétences de la période 5 de la classe précédente, en période 1 ;
  - les compétences qui déclarent cette classe dans `alsoOffered`.
  Chaque compétence n'apparaît qu'une fois, et un rattachement déclaré l'emporte sur la règle par défaut. Le tri
  se fait par période, les compétences de la classe avant les révisions.
- `offered_or_fallback` : une classe sans compétences propres (CE2 à CM2) reçoit ce qui est proposé au CE1,
  révisions comprises. C'est cohérent avec le repli existant.
- La commande `get_skills` ajoute `revisionFrom` (classe d'origine, ou `null`). Une révision n'est jamais marquée
  « en ce moment ». Les types TypeScript sont régénérés.
- Front : la section « Révisions » s'affiche après la période en cours. Le titre est volontairement court, plutôt
  que « Révisions de l'année dernière » : du CE2 au CM2, qui reçoivent pour l'instant les exercices du CE1, les
  révisions du CP datent de deux ans. La section reste visible toute l'année. Chaque bouton porte
  l'étiquette de sa classe (« CP ») et l'indication « Révision du CP ». Les points comptent normalement : la série
  est la même.
- Tests :
  - catalogue : règle par défaut, `alsoOffered`, repli CE2 ;
  - chargement : `alsoOffered` valide et invalide ;
  - contenu : en CE1 période 1, les compétences du CP période 5, sans doublon ;
  - commande `get_skills` ;
  - Vitest : `SkillsScreen.test.tsx`.
