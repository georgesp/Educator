# Règles de génération des exercices de calcul

Fiche : SUB000106. Schéma : [`content/schemas/skill.schema.json`](../../content/schemas/skill.schema.json).

Une **compétence** est une notion travaillée à une classe et à une période. Elle est décrite par un fichier
`content/skills/math/<id>.json`. Les exercices ne sont **jamais stockés** : le moteur
(`Educator.Core/Exercises`) les génère en direct à partir de ces règles, avec une graine aléatoire
qui permet de rejouer une série à l'identique.

## Exemple

```json
{
  "$schema": "../../schemas/skill.schema.json",
  "id": "cp-sub-from-tens",
  "subject": "math",
  "grade": "CP",
  "period": 5,
  "title": "Enlever un petit nombre à une dizaine",
  "instruction": "Calcule ou trouve le nombre qui manque.",
  "operation": "subtraction",
  "operands": [ { "min": 20, "max": 90, "multipleOf": 10 }, { "min": 1, "max": 9 } ],
  "constraints": { "carry": "any" },
  "formats": [ "result", "missingOperand" ],
  "answerModes": [ "keyboard", "choice" ],
  "source": "Programme de mathématiques du cycle 2 (2024), CP — soustraire un nombre inférieur à 10 à un nombre entier de dizaines (50 − 6)."
}
```

## Champs

| Champ | Valeurs | Rôle |
|---|---|---|
| `id` | minuscules et tirets | Identifiant stable, **identique au nom du fichier** |
| `subject` | `math` | Matière |
| `grade` | `CP` … `CM2` | Classe |
| `period` | 1 à 5 | Période de l'année scolaire (voir plus bas) |
| `title` | 1 à 60 caractères | Titre montré à l'enfant |
| `instruction` | 1 à 120 caractères | Consigne affichée et lue au premier exercice |
| `operation` | `addition`, `subtraction`, `mixed` | `mixed` : chaque exercice est une addition ou une soustraction |
| `operands` | 2 règles | Opérande de gauche puis de droite ; pour une soustraction, la gauche est le nombre dont on enlève |
| `operands[].min`, `max` | 0 à 100 000 | Bornes incluses |
| `operands[].multipleOf` | ≥ 1 | Seulement les multiples (10 : dizaines entières) |
| `operands[].values` | liste | Valeurs exactes (ex. `[9, 19, 29]`), comprises entre `min` et `max` |
| `constraints.minResult`, `maxResult` | entiers ≥ 0 | Bornes du résultat |
| `constraints.exactResult` | entier ≥ 0 | Résultat imposé |
| `constraints.resultMultipleOf` | ≥ 1 | Le résultat est un multiple (10 : complément à la dizaine) |
| `constraints.carry` | `any`, `forbidden`, `required` | Retenue (addition) ou emprunt (soustraction) dans au moins une colonne |
| `constraints.excludeTrivial` | `plusZero`, `minusZero`, `minusSelf` | Calculs triviaux jamais proposés |
| `formats` | `result`, `missingOperand` | `7 + 5 = ?` ou `7 + ? = 12` (le trou est à gauche ou à droite, au hasard) |
| `answerModes` | `keyboard`, `choice` | Saisie au clavier ou QCM à 4 propositions |
| `source` | texte | Référence du programme officiel justifiant la règle |

Les propriétés inconnues sont refusées. Le résultat d'une soustraction n'est jamais négatif.

## Ce que fait le moteur

- **Série** : 10 exercices par défaut, sans doublon (7 + 5 et 5 + 7 comptent comme un seul).
- **Format** : 60 % `result` et 40 % `missingOperand` quand la compétence permet les deux.
- **QCM** : la bonne réponse et 3 erreurs crédibles (±1, erreur de retenue ±10, opération inversée), jamais
  négatives ni en double, dans un ordre aléatoire.
- **Graine** : la même compétence et la même graine redonnent exactement la même série.
- La réponse attendue reste dans le host : le front ne la reçoit qu'après la réponse de l'enfant.

## Validation

Au démarrage, le host charge les compétences ; un fichier invalide est ignoré et consigné dans les logs.
L'enfant ne voit donc jamais une règle cassée. Ce sont les **tests** qui échouent (`Educator.Content.Tests`,
lancés par la CI) :

- chaque fichier respecte le schéma JSON ;
- chaque règle permet **au moins 20 exercices différents** (règle trop étroite sinon) ;
- 10 000 tirages par compétence respectent toutes les contraintes ;
- le CP et le CE1 ont des compétences dans chacune des 5 périodes.

## Périodes de l'année

`content/calendar/periods.json` donne la date de début de chaque période (1er septembre, 4 novembre, 5 janvier,
9 mars, 4 mai). Ces dates sont placées après les vacances de toutes les zones, si bien que la période calculée ne
dépend pas de la zone. Juillet et août comptent dans la période 5. `GET /api/ui/skills?grade=CE1` renvoie les
compétences triées par période et indique la période en cours.

## Programme de référence

Les compétences CP–CE1 suivent le **programme de mathématiques du cycle 2** (arrêté du 22 octobre 2024, BO n°41
du 31 octobre 2024), en vigueur depuis la rentrée 2025. Il remplace, pour les mathématiques, les repères annuels
de progression de 2020. Quand le programme ne place pas une notion dans une période précise, la période est un
choix d'Educator, indiqué dans le champ `source`.
