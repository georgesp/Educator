---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5RxI
url: https://github.com/georgesp/Educator/issues/7
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000106 — Format des règles de génération des exercices
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 70
---

## User story

En tant que concepteur du contenu, je veux décrire chaque compétence par des règles dans un fichier afin d'ajouter ou d'ajuster des exercices sans modifier le code.

## Comportement attendu

- Une **compétence** = une notion travaillée à une classe et une période (1 à 5), décrite par des règles de génération.
- Les exercices ne sont **jamais stockés** : ils sont générés en direct à partir de ces règles.
- Fichiers `content/skills/math/*.json`, validés par un schéma JSON `content/schemas/skill.schema.json`.

Exemple :

```json
{
  "id": "cp-add-sum-10",
  "subject": "math",
  "grade": "CP",
  "period": 1,
  "title": "Additions jusqu'à 10",
  "instruction": "Calcule.",
  "operation": "addition",
  "operands": [ { "min": 0, "max": 10 }, { "min": 0, "max": 10 } ],
  "constraints": { "maxResult": 10, "carry": "any", "excludeTrivial": ["plusZero"] },
  "formats": [ "result", "missingOperand" ],
  "answerModes": [ "keyboard", "choice" ],
  "source": "Repères annuels CP, période 1 (eduscol)"
}
```

- `operation` : `addition` ou `subtraction` au lot 1 (extensible).
- `carry` : `forbidden`, `required` ou `any` (retenue / passage de la dizaine).
- `formats` : `result` (7 + 5 = ?), `missingOperand` (7 + ? = 12).
- `source` : référence du programme officiel justifiant la règle.

## Critères d'acceptation

- [x] Schéma JSON écrit et documenté (`docs/specs/` + exemples).
- [x] Chargement des compétences au démarrage du host ; une règle invalide fait **échouer les tests et le build**, pas l'application devant l'enfant.
- [x] `GET /api/ui/skills?subject=math&grade=CE1` renvoie les compétences de la classe, triées par période, avec la période « en cours » calculée depuis la date du jour.
- [x] Test : chaque règle peut produire au moins 20 exercices distincts (règle trop étroite détectée).

## Notes techniques

- Projet `Educator.Content` : chargement, validation, exposition en lecture seule.
- Calcul de la période en cours : découpage de l'année scolaire en 5 périodes (rentrée → Toussaint → Noël → hiver → printemps → été), dates paramétrées dans un fichier, sans dépendre de la zone de vacances.

## Implémentation (2026-10-11)

- Schéma : `content/schemas/skill.schema.json`, documenté dans `docs/specs/regles-de-generation.md`. Ajouts au format prévu :
  - `operation: "mixed"` ;
  - `operands[].multipleOf` et `operands[].values` ;
  - `constraints.minResult`, `exactResult` et `resultMultipleOf` ;
  - `excludeTrivial` : `plusZero`, `minusZero`, `minusSelf`.
- Chargement : `Educator.Content/Skills/SkillLoader` refuse les propriétés inconnues et contrôle tout ce que dit le schéma, plus l'ordre des bornes et la règle des 20 exercices. Un fichier invalide est ignoré et consigné dans les logs, sans planter. Ce sont les tests qui échouent, et la CI avec eux.
- Les fichiers sont aussi validés contre leur schéma JSON par les tests (validateur minimal dans `Educator.Content.Tests`).
- Période en cours : `content/calendar/periods.json` (1er sept., 4 nov., 5 janv., 9 mars, 4 mai). Ces dates tombent après les vacances de toutes les zones ; juillet et août comptent en période 5.
- Une règle de plus de 2 millions de paires d'opérandes est refusée (« Rule too wide »). Sinon, l'énumération ralentirait le démarrage du host.
- Le contenu est copié à côté du host à la compilation. `EDUCATOR_CONTENT_DIR` permet de pointer ailleurs.
