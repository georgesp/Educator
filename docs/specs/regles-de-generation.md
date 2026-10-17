# Règles de génération des exercices de calcul

Fiche : SUB000106. Schéma : [`content/schemas/skill.schema.json`](../../content/schemas/skill.schema.json).

Une **compétence** est une notion travaillée à une classe et à une période. Elle est décrite par un fichier
`content/skills/math/<id>.json`. Les exercices ne sont **jamais stockés** : le moteur
(`crates/educator-core`, module `exercises`) les génère en direct à partir de ces règles, avec une graine aléatoire
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
| `alsoOffered` (facultatif) | liste de `{ "grade", "period" }` | Autres classes où la compétence est aussi proposée, en révision (une seule fois par classe, jamais la sienne) |

Les propriétés inconnues sont refusées. Le résultat d'une soustraction n'est jamais négatif.

## Ce que fait le moteur

- **Série** : 10 exercices par défaut, sans doublon (7 + 5 et 5 + 7 comptent comme un seul).
- **Variété** (SUB000112) :
  - une compétence `mixed` a autant d'additions que de soustractions (5 et 5) ;
  - jamais deux fois de suite la même forme (même opération et même format) quand la compétence en offre
    plusieurs : une compétence d'une seule opération avec les deux formats alterne donc `result` et
    `missingOperand` ;
  - un même nombre ne revient pas plus de 3 fois comme opérande dans une série, quand la règle laisse assez
    d'autres nombres (les opérandes à moins de 4 valeurs possibles, comme « 1 ou 2 », ne sont pas comptés).
- **Format** : tiré à 60 % `result` et 40 % `missingOperand` quand la compétence permet les deux, sauf quand la règle
  de variété impose de changer.
- **QCM** : la bonne réponse et 3 erreurs crédibles (±1, erreur de retenue ±10, opération inversée), jamais
  négatives ni en double, dans un ordre aléatoire.
- **Graine** : la même compétence et la même graine redonnent exactement la même série.
- La réponse attendue reste dans le cœur Rust : le front ne la reçoit qu'après la réponse de l'enfant.

## Validation

Au démarrage, l'application charge les compétences ; un fichier invalide est ignoré et consigné dans les logs.
L'enfant ne voit donc jamais une règle cassée. Ce sont les **tests** qui échouent (`crates/educator-content/tests`,
lancés par la CI) :

- chaque fichier respecte le schéma JSON ;
- chaque règle permet **au moins 20 exercices différents** (règle trop étroite sinon) ;
- 10 000 tirages par compétence respectent toutes les contraintes (20 000 pour les compléments du CE1 et les tables) ;
- le CP et le CE1 ont des compétences dans chacune des 5 périodes ;
- en CE1 période 1, les compétences du CP période 5 sont proposées en révision, sans doublon.

`cargo run -p educator-content --example samples` régénère `docs/specs/echantillons-lot1.md` (additions et
soustractions) et `docs/specs/echantillons-ce1.md` (compléments du CE1), 20 exercices par compétence, pour la relecture.

## Familles des compléments du CE1 (SUB000118 à SUB000124)

Le champ `operation` choisit la famille. Les calculs (`addition`, `subtraction`, `mixed`, `multiplication`) gardent
`operands` et `constraints` ; les autres familles ne les ont pas et décrivent leurs nombres autrement. Les règles de
variété restent les mêmes : pas de doublon dans une série, formats équilibrés et jamais deux fois de suite le même
quand la compétence en a plusieurs.

| Famille | Champs propres | Formats |
|---|---|---|
| `multiplication` | `operands`, `constraints`, `focus` (tables travaillées), `picture: "arrayGrid"`, `progression` | `result`, `missingOperand` (aussi `30 = 3 × ?`), `phrase` (`5 + 5 + 5 = ? × 5`) |
| `numeration` | `numbers` (même forme qu'un opérande), `lineSteps` (pas des droites graduées, `[1]` au CP) | `recompose`, `decompose`, `placeValue`, `writeNumber`, `numberDictation` (CP : le nombre est seulement entendu, au clic), `readNumber`, `compare`, `order`, `frame`, `numberLinePoint`, `yesNo`, `multiChoice` |
| `doubleHalf` | `doubles`, `halves`, `timesTwentyFive`, `progression` | `phrase` (« Quel est le double de 35 ? »), `result` (« Le double de 35 = ? ») |
| `groups` (CP) | `groups: { kinds, count, size, maxTotal }` (`times`, `share`, `groupBy`) | `phrase` (« 4 fois 5 = 5 + 5 + 5 + 5 = ? », « 18 images pour 3 enfants… »), `result` (« 3 fois 20 = ? », « 3 fois ? = 18 ») |
| `fraction` | `fractions: { denominators, unitOnly }` | `fractionPicture`, `fractionWords`, `compare`, `result` (somme ou différence) |
| `problem` | `problems: { kinds, maxNumber, startMaxNumber, templates, noCarry }` | `problem` |

- **Multiplication** : les facteurs sont montrés dans un ordre ou l'autre (sauf avec l'image des rangées) ; avec
  `focus`, 7 exercices sur 10 portent sur les tables travaillées, les autres révisent les autres tables ; `0 × ? = 0`
  n'est jamais demandé (toutes les réponses seraient justes). Pour 11 à 19 × 2 à 9, la correction montre la
  décomposition (`13 × 7 = 10 × 7 + 3 × 7 = 70 + 21 = 91`) et le quadrillage coupé après 10 colonnes.
- **`progression: "easyFirst"`** : les exercices les plus faciles d'abord (× 0, 1, 2, 5, 10, puis un petit facteur ;
  moitiés des dizaines paires avant les impaires, puis au-delà de 100).
- **Numération** : les comparaisons et les rangements portent sur des nombres proches (mêmes chiffres dans un autre
  ordre, un seul chiffre changé), pour que le bon rang décide ; la correction dit à quel rang. Les nombres en lettres
  sont écrits en orthographe traditionnelle ; la graphie de 1990 (`deux-cent-quatre-vingts`) est toujours acceptée et
  n'est jamais proposée comme mauvaise réponse. La droite graduée a 11 graduations, de 1 en 1, de 10 en 10 ou de
  100 en 100 (`lineSteps` choisit les pas). Avec des nombres à deux chiffres (CP), aucune proposition n'a trois
  chiffres, sauf l'erreur d'écoute de la dictée (612 pour « soixante-douze »). Sans voix, une série ne propose pas
  `numberDictation`.
- **Moitiés au CP** : la correction décompose en moitiés connues au CP (52 = 40 + 12 → 20 + 6), jamais avec la moitié
  d'une dizaine impaire ; le CE1 décompose par rang (56 = 50 + 6).
- **Groupes (CP)** : jamais les signes × et ÷ ; « fois » est montré en mots et en addition répétée, les paquets
  dessinés (barres de dix au-delà de 10 par paquet) ; pour un partage ou des groupes, les paquets sont dessinés après
  une erreur. Propositions : les deux nombres ajoutés (3 fois 20 → 23), un paquet de trop ou de moins.
- **Fractions** : toute fraction égale est acceptée (2/4 pour 1/2) ; la correction montre la forme attendue.
- **Réponses** : nombre, fraction, signe (`<`, `>`, `=`), oui / non, suite ordonnée (rangement, encadrement),
  ensemble (tous les nombres pairs), texte (écriture en lettres). Le mode `keyboard` fait produire la réponse (pavé,
  signes, glisser-déposer) ; le mode `choice` la fait choisir parmi des propositions. Une compétence qui n'a qu'un
  mode ouvre directement la série.

## Problèmes (SUB000124)

Les énoncés sont générés à partir de **gabarits** (`content/problems/*.json`, schéma
`problem-templates.schema.json`) et de mots tirés au hasard (`content/problems/words.json`, schéma
`problem-words.schema.json` : prénoms, autant de filles que de garçons, objets, contenants).

```json
{
  "id": "comparison-more",
  "kind": "comparison",
  "text": "{name} a {a} {objects}. {name2} en a {b} de plus. Combien de {objects} {name2} a-t-{il2} ?",
  "variables": { "a": { "min": 5 }, "b": { "min": 2 } },
  "answer": "a + b",
  "solution": "{a} + {b} = {answer}. {name2} a {answer} {objects}.",
  "mistakes": ["a - b"],
  "help": { "bars": [["a"], ["a", "b"]] }
}
```

- `kind` : `partWhole`, `transformation`, `comparison` (une étape), `twoSteps`, `product`, `sharing`, `mixed`.
- `variables` : plages des nombres ; sans `max`, la taille de la compétence (`maxNumber`, ou `startMaxNumber` pour
  les 6 premiers exercices de la série). `derived` : valeurs intermédiaires (`"p": "a * b"`), utilisables dans le
  texte. `checks` : conditions du tirage (`"c < p"`). Tous les nombres (variables, valeurs intermédiaires, réponse)
  restent entre 0 et la taille de la compétence ; une division doit tomber juste.
- Dans `text` et `solution` : `{a}`… les nombres, `{name}`, `{name2}` les prénoms, `{il}`/`{Il}` et `{il2}`/`{Il2}`
  leurs pronoms, `{ils}` (« elles » pour deux filles), `{object}`/`{objects}`, `{container}`/`{containers}`.
  L'élision est faite à la génération (« combien d'images », « qu'Inès »). Le texte ne contient jamais la réponse.
- `mistakes` : erreurs typiques proposées en QCM (mauvaise opération, une seule étape faite). Les nombres de
  l'énoncé sont aussi proposés (nombre recopié).
- `help` : schéma montré avec la correction, en barres (`bars`, `total`) ou en groupes (`groups`, `perGroup`).
- `grades` : classes qui utilisent le gabarit (absent : toutes). Les gabarits multiplicatifs du CP (`-cp`) disent
  « 3 fois 5, c'est 15 » au lieu de `3 × 5 = 15`.
- Dans une compétence : `templates` limite aux gabarits listés (par leur `id`) ; `noCarry` refuse les retenues et les
  cassages de dizaine dans le calcul (CP, période 2 : le cassage arrive en période 3).

Un gabarit invalide (variable inconnue, expression incorrecte, aucun tirage possible à la taille de la compétence)
est ignoré au démarrage et fait échouer les tests.

## Périodes de l'année

`content/calendar/periods.json` donne la date de début de chaque période (1er septembre, 4 novembre, 5 janvier,
9 mars, 4 mai). Ces dates sont placées après les vacances de toutes les zones, si bien que la période calculée ne
dépend pas de la zone. Juillet et août comptent dans la période 5. La commande `get_skills` (`grade: "CE1"`) renvoie les
compétences triées par période et indique la période en cours.

## Révisions entre classes (SUB000113)

Les compétences de la **période 5** d'une classe sont aussi proposées en **période 1 de la classe suivante**, sous
le titre « Révisions », avec l'étiquette de leur classe d'origine (`revisionFrom: "CP"` dans
`get_skills`). Une compétence peut déclarer d'autres rattachements avec `alsoOffered` ; un rattachement déclaré
remplace la règle par défaut pour cette classe. Les points gagnés sur une révision comptent normalement.

Les classes qui n'ont pas encore de compétences (CE2 à CM2 au lot 1) reçoivent ce qui est proposé au CE1, révisions
comprises.

## Programme de référence

Les compétences CP–CE1 suivent le **programme de mathématiques du cycle 2** (arrêté du 22 octobre 2024, BO n°41
du 31 octobre 2024), en vigueur depuis la rentrée 2025. Il remplace, pour les mathématiques, les repères annuels
de progression de 2020. Quand le programme ne place pas une notion dans une période précise, la période est un
choix d'Educator, indiqué dans le champ `source`.
