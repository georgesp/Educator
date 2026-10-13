# Animations des leçons de mathématiques (SUB000117)

Bibliothèque des animations utilisables dans `slides[].animation` d'une leçon (`content/lessons/math/*.json`).
Le schéma `content/schemas/lesson.schema.json` décrit les paramètres ; le chargeur Rust (`educator-content::lessons`)
ajoute les règles de cohérence qu'un schéma ne sait pas dire. Une leçon invalide fait échouer les tests et n'est
jamais montrée à l'enfant.

Principe : **le cœur calcule, le front dessine**. Toutes les valeurs affichées (chiffres d'une opération posée, blocs à
chaque étape, cases d'une table, signe d'une comparaison, parts d'une fraction, réponse d'un schéma) sont calculées par
`educator-core::lessons` et envoyées toutes prêtes par la commande `get_lesson` (`AnimationResponse`).

Charte « S1 · Grand Prix » : couleurs et polices par variables CSS du thème (`styles.css`, `lesson.css`), rouge course
pour ce qui est nouveau ou important, objets aux couleurs de la palette des personnages. Mouvements courts
(`useSteps`, `useMove`), remplacés par des fondus quand le système demande de réduire les animations.

Une slide « interactive » bloque le bouton « Suivant » tant que l'enfant n'a pas terminé ; une erreur donne un indice
bienveillant (« Presque ! … »), jamais de son ou d'image punitifs.

## Catalogue

| Animation | Ce qu'elle montre | Paramètres (fichier) | Interactive |
|---|---|---|---|
| `titleCard` | Écran-titre : damier, titre, sous-titre, voiture | `title`, `subtitle` | — |
| `recapCard` | « Je retiens » : la règle, un exemple | `text`, `example` | — |
| `calcChain` | Calcul en chaîne de flèches | `chains[]` (`start`, `steps`, `crossed`), `mode` : `show`, `match`, `pick` | `match` : relier chaque calcul à son chemin ; `pick` : cliquer le seul bon chemin (les autres sont `crossed`) |
| `numberLine` | Piste graduée, la voiture fait des bonds | `from`, `to`, `step`, `start`, `jumps`, `flags`, `hoverValues` | cliquer où la voiture arrive après chaque bond |
| `tensUnits` | Plaques, barres, cubes | `value`, `completeTen`, un changement au plus : `add`, `remove`, `share` (2 à 4 parts) | fabriquer `value` ; ajouter, enlever (casser une barre) ou partager jusqu'au résultat |
| `columnOperation` | Opération posée pas à pas | `operation` (`add`, `subtract`), `operands`, `method` (`cassage`, `compensation`, `both`), `computedColumns` | écrire les chiffres du résultat depuis les unités |
| `equation` | Égalité, case à trou, famille de nombres | `left`, `operation` (`add`, `subtract`, `multiply`), `right`, `missing`, `family`, `choices` | choisir le nombre de la case |
| `compareNumbers` | Deux nombres comparés rang par rang (jusqu'à 9 999) | `a`, `b` | choisir `<`, `>` ou `=` |
| `sequencePattern` | Suite à compléter, ou nombres à ranger | `start`, `step`, `count`, `hidden` ; ou `values`, `descending` | ranger : cliquer les voitures dans l'ordre |
| `arrayGrid` | Grille de départ : rangées de voitures | `rows`, `cols`, `showSum`, `split`, `rotate`, `removed` | placer les rangées ; ou cliquer où couper (`split`) |
| `timesTable` | Table d'addition ou de multiplication | `operation`, `size`, `highlights[]` (`row`, `doubles`, `result`) | — |
| `placeValueTable` | Tableau de numération | `value`, `focus`, `countOf`, `decomposition`, `words` | écrire le nombre lu, chiffre par chiffre |
| `groupObjects` | Objets de course | `mode` (`join`, `remove`, `groupBy`, `share`, `sortParity`), `object`, `counts`, `total`, `removed`, `size`, `parts`, `numbers`, `commute` | `join`, `remove` : déplacer les objets puis choisir le résultat ; `sortParity` : garer chaque nombre |
| `tenFrame` | Garage de 10 places | `filled`, `added`, `removed`, `frames`, `friends` | garer les voitures qui arrivent, faire partir celles qui partent |
| `fractionBar` | Bandes partagées en parts égales | `bars[]` (`denominator`, `numerators`), `unequal`, `notation` | colorier les parts |
| `barModel` | Schéma en barres d'un problème | `kind` (`parts`, `compare`, `groups`), `parts`, `small`, `difference`, `groups`, `size`, `unknown`, `labels`, `solve` | placer les nombres de l'histoire |

`proportionTable` (CM) sera ajoutée avec les leçons des lots 5 et 6.

## Règles du chargeur (extraits)

- `numberLine` : au plus 41 graduations ; départ, bonds et drapeaux tombent sur une graduation.
- `calcChain` : `match` demande des chemins qui ajoutent des nombres différents ; `pick` demande un seul chemin non
  barré, tous partant du même nombre.
- `tensUnits` : un seul changement ; un partage tombe juste (`value` divisible par `share`) ; pas de `completeTen` avec
  un changement ni sur une slide interactive.
- `columnOperation` : nombres jusqu'à 9 999 ; une soustraction a une méthode et le plus grand nombre en haut ; pas de
  multiplication posée (CE2).
- `equation` : une multiplication n'a pour famille que l'échange des facteurs (8 × 5 = 5 × 8) ; une égalité
  interactive a sa case parmi les propositions.
- `sequencePattern` : les valeurs à ranger ne sont pas déjà rangées.
- `placeValueTable` : une table interactive n'a ni `countOf`, ni `decomposition`, ni `words`.
- `arrayGrid` : `split`, `showSum`, `rotate` et `removed` ne se combinent pas.
- `barModel` : `labels` (2 au plus) seulement pour une comparaison.
- `groupObjects` : 20 objets au plus à dessiner un par un ; `sortParity` mélange pairs et impairs ; pas de `commute` en interactif.
- `fractionBar` : `unequal` sur une seule bande de 2 à 4 parts ; l'interaction porte sur une seule bande.

## Quiz

`quiz[].answer` (et `choices`) : un nombre, un signe (`<`, `>`, `=`), `oui` / `non`, ou une fraction `a/b` (toute
fraction égale est acceptée). Les propositions sont du même type que la réponse et une seule est juste ; une question
oui / non a exactement les deux propositions. Sans propositions, l'enfant tape un nombre.

## Où regarder

- Modèle et calculs : `crates/educator-core/src/lessons.rs` et `lessons/{blocks,figures,objects}.rs`.
- Chargement et validation : `crates/educator-content/src/lessons.rs` et `lessons/figures.rs`.
- Contrats : `web/src-tauri/src/contracts.rs` et `contracts/figures.rs`.
- Composants : `web/src/components/lesson/animations/`, aiguillés par `LessonAnimation.tsx`.
- Démonstrations : page de développement `#/dev/composants`, section « Leçons » (`web/src/screens/lessonDemos.ts`).
