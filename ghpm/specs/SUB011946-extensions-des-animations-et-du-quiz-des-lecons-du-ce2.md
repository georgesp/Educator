---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoi8
url: https://github.com/georgesp/Educator/issues/303
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011946 — Extensions des animations et du quiz des leçons du CE2
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 921
---

Source : revue du lot 4 du 2026-10-16 — plusieurs leçons du CE2 demandent des animations ou des questions de quiz que le
format actuel (`content/schemas/lesson.schema.json`, lecteur SUB001410, quiz SUB001411) ne sait pas encore afficher.
Cette fiche les regroupe pour qu'elles soient faites une fois, avant les leçons qui en dépendent.

## User story

En tant qu'enfant de CE2, je veux des leçons qui me montrent les opérations posées, les fractions, les mesures, l'heure,
la monnaie et la grammaire avec les mêmes animations que les autres leçons, afin de comprendre en manipulant.

## Animations existantes à étendre (mathématiques)

| Animation | Ajout | Leçons concernées |
|---|---|---|
| `columnOperation` | Opérandes jusqu'à 9 999, trois termes, prix avec virgule (euros sous euros, centimes sous centimes) ; `operation: "multiply"` : produits partiels, zéro de décalage, retenues en rouge, somme finale | Additions et soustractions posées, Multiplication posée |
| `equation` | Division (`operation: "divide"`, signe « ÷ ») et famille × / ÷ (4 × 6 = 24 → 24 ÷ 4 = 6, 24 ÷ 6 = 4) ; opérandes fractions (1/10 + 3/10) | Comprendre la division, Comparer, ajouter et soustraire des fractions |
| `compareNumbers` | Comparer deux fractions (4/10 … 1/2), avec `fractionBar` en appui | Comparer, ajouter et soustraire des fractions |
| `numberLine` | Graduation en fractions de l'unité (moitiés, quarts, cinquièmes, dixièmes), point « 1 unité + 3/10 » ; même composant que l'exercice `numberLinePoint` | Les fractions : partager l'unité |
| `fractionBar` | Bandes empilées interactives (colorier, comparer des longueurs) | Les fractions : partager l'unité ; Comparer, ajouter et soustraire des fractions |
| `placeValueTable` | Glissement des chiffres d'un ou deux rangs (× 10, × 100) et retour (facteur manquant) ; mot des deux écritures en lettres côte à côte (traditionnelle et rectifiée de 1990) | Multiplier par 10 et par 100, Nombres jusqu'à 10 000 |
| `tensUnits` | Bloc du millier ; valeurs jusqu'à 9 999 | Nombres jusqu'à 10 000 |
| `sequencePattern` | Mode « ranger » : valeurs quelconques à remettre dans l'ordre (glisser) | Comparer, ranger et encadrer jusqu'à 10 000 |
| `timesTable` | Mode interactif (cases à compléter, course aux produits) ; mise en évidence de la symétrie (`highlights.kind: "symmetry"`) | Tables de 2, 3, 4, 5 et 10 ; Tables de 6, 7, 8 et 9 |
| `barModel` | « Fois plus » / « fois moins » : la petite barre reportée n fois (`kind: "times"`) | Résoudre des problèmes |

## Nouvelles animations (grandeurs et mesures)

| Animation | Rôle | Leçons concernées |
|---|---|---|
| `ruler` | Règle graduée en cm et mm (et en fractions d'unité) ; segment à mesurer, interactif (déplacer l'extrémité) | Longueurs |
| `conversionTable` | Tableau des unités (km … m, dm, cm, mm ; t, kg, g ; L, dL, cL) : un nombre s'écrit, les unités se décomposent (215 cm = 2 m 1 dm 5 cm) | Longueurs, Masses et contenances |
| `scale` | Balance à plateaux et masses marquées ; balance à aiguille graduée | Masses et contenances |
| `measuringJug` | Récipient gradué (L, dL, cL) qui se remplit | Masses et contenances |
| `clock` | Horloge à aiguilles synchronisée avec l'affichage numérique ; aiguilles à déplacer (interactif) ; arc coloré pour une durée | Lire l'heure et les durées |
| `coins` | Pièces et billets en euros (dessins originaux, pas de reproduction des pièces réelles) : composer une somme, rendre la monnaie en complétant | La monnaie |

Ces composants servent aussi aux exercices (fiches de grandeurs et mesures) : un seul composant par objet, dans la charte
Grand Prix (variables CSS du thème, aucune couleur en dur).

## Nouvelles animations (français)

| Animation | Rôle | Leçons concernées |
|---|---|---|
| `sentenceGroups` | Phrase dont les groupes s'entourent : groupe sujet, verbe, compléments ; mots cliquables | La phrase et les classes de mots, Les accords |
| `agreementChain` | Chaîne d'accord : flèches du nom vers le déterminant et l'adjectif, du sujet vers le verbe ; les marques (-s, -x, -e, -nt) s'allument | Les accords |
| `wordClassSort` | Étiquettes de mots à ranger dans des colonnes (nom, verbe, adjectif… ; singulier / pluriel ; présent / futur), généralisation de `soundSort` | La phrase et les classes de mots, Les accords, la conjugaison |
| `conjugationTable` | Tableau d'un verbe : radical et terminaison de couleurs différentes, personne mise en évidence ; passage d'un temps à l'autre | Le présent ; L'imparfait, le futur et le passé composé |
| `homophoneSwap` | Le mot se remplace par son « test » (a → avait, est → était, son → mon, sont → étaient, on → il, ont → avaient) : la phrase reste juste ou devient fausse | Les homophones |
| `wordFamily` | Arbre d'une famille de mots : radical commun, préfixes et suffixes de couleurs différentes, lettre muette qui « s'entend » dans le dérivé (grand → grande) | Familles de mots ; Les sons et leurs écritures |

## Quiz de fin de leçon

- **Choix en mots dans les leçons de mathématiques** : une question à propositions peut avoir des réponses en mots courts
  (« différence », « le produit », « j'enlève 2 ») ou en signes d'opération (« + », « − », « × », « ÷ ») ; aujourd'hui,
  un mot n'est accepté que dans une leçon d'orthographe (`educator-content::lessons`). Les réponses en mots restent
  toujours des QCM en mathématiques (jamais à écrire).
- **Réponse avec unités** : une réponse peut être une mesure en plusieurs cases (2 h 20 min, 3 m 5 cm, 3,68 €), validée
  par la même règle que les exercices (valeur convertie dans la plus petite unité).
- **Leçons de français** : réponse « phrase » courte (≤ 60 caractères) pour une forme verbale ou un groupe nominal
  (« nous avons joué », « les beaux chevaux »), sans tolérance sur les accents.
- Les questions qui demandaient une réponse non gérée ont été reformulées dans les fiches de leçons (encadrement → une
  seule borne à écrire ; rangement → le plus petit nombre ; choix d'un schéma → problème à résoudre).

## Critères d'acceptation

- [ ] `lesson.schema.json` et le chargeur Rust acceptent chaque ajout ; une valeur incohérente (fraction de dénominateur nul, heure > 23 h 59, unité inconnue) fait échouer `content_validation.rs`.
- [ ] Chaque animation nouvelle ou étendue a un test Vitest (rendu, interaction, réduction des animations) et une démo dans `docs/specs/animations-lecons.md`.
- [ ] Interactions utilisables à la souris et au doigt (iPad, ADR 0006), cibles de 44 points au moins.
- [ ] Charte Grand Prix : couleurs et polices par variables du thème ; mouvements < 600 ms, fondus si le système réduit les animations.
- [ ] Rien n'est lu tout seul : la voix part au clic (décision du 2026-10-15).
- [ ] Quiz : choix en mots et en signes acceptés en mathématiques, réponses avec unités, réponse « phrase » en français ; tests `educator-content` et `educator-core`.
- [ ] Types TypeScript régénérés (`npm run api:generate`).

## Dépendances

- Bibliothèque d'animations des leçons de mathématiques (SUB000117), format des leçons (SUB001409), lecteur (SUB001410), quiz (SUB001411).
- À faire avant les leçons du lot qui citent cette fiche dans leurs dépendances.
