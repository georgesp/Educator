---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACXzI
rev: 2026-10-12T19:49:07Z
url: https://github.com/georgesp/Educator/issues/85
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000117 — Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix)
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#1"
order: 770
---

## User story

En tant qu'enfant, je veux des leçons animées qui ressemblent au reste de l'application (la course, la voiture, le
damier) afin de me sentir dans le même univers quand je passe des exercices aux leçons.

## Objectif

Regrouper dans une seule bibliothèque React les animations des leçons de mathématiques, toutes dessinées dans la charte
« S1 · Grand Prix » (SUB000105), paramétrées par les fichiers de leçon (`slides[].animation`, SUB001409). Les fiches
leçon des lots 1 à 1.4 n'utilisent que ces animations : une nouvelle animation est d'abord ajoutée ici.

Elle complète la fiche SUB001410 (lecteur de leçons), qui prévoyait `numberLine`, `groupObjects` et `tensUnits` : ces
trois-là sont reprises ici avec le style Grand Prix ; `letterTiles` et `soundSort` (orthographe) restent au lot 2.

## Patte graphique (commune à toutes les animations)

- Couleurs et polices uniquement par les variables CSS du thème (`[data-theme="sport"]`, `web/src/styles.css`) :
  fond `--bg`, surfaces `--surface`, texte `--text`, rouge course `--primary` pour ce qui est **nouveau ou important**
  (retenue, chiffre qui change, case à trouver), `--correct` / `--incorrect` pour les retours, or `--firework-gold`
  pour les réussites. Le thème personnalisable du lot 3 s'applique donc aussi aux leçons.
- Titres et grands nombres en Barlow Condensed italique, texte en Lexend ; taille ≥ 20 px, grands nombres ≥ 48 px.
- Motifs : bandeau damier (`--checker-*`), formes en parallélogramme (`--slant`), drapeau à damier pour une arrivée ou un
  résultat, la **voiture** de la piste de progression (`RaceTrack`) comme personnage qui se déplace sur les droites
  graduées.
- Objets à manipuler : voitures, casques, pneus, drapeaux, cônes, en SVG maison, couleurs de
  `content/characters/palette.json` (rouge, bleu, jaune, vert…) ; contour `outline` comme les personnages.
- Mouvements : Framer Motion, < 600 ms par étape, déclenchés au rythme de la voix (repères `cue` dans le texte lu) ;
  remplacés par des fondus si `prefers-reduced-motion` ; aucune animation en boucle pendant que l'enfant réfléchit.
- Interactions : glisser-déposer (dnd-kit) et clic, cibles ≥ 64 px ; en cas d'erreur, l'objet revient doucement et
  un indice s'affiche (jamais de son ou d'image punitifs).
- Aucune ressource externe (images, polices, sons) : tout est dessiné en SVG / CSS ou synthétisé (Web Audio).

## Catalogue

| Animation | Rôle | Paramètres principaux |
|---|---|---|
| `titleCard` | Écran-titre : bandeau damier, titre, mascotte qui présente (si activée) | `title`, `subtitle` |
| `recapCard` | « Je retiens » : la règle en une phrase, encadré rouge course, drapeau à damier | `text`, `example` |
| `groupObjects` | Objets à réunir, retirer, grouper par 10, partager, mettre en paquets | `mode` (`join`, `remove`, `groupBy`, `share`), `objects`, `counts` |
| `tenFrame` | « Garage de 10 places » (2 × 5) pour les amis de 10 et le passage de la dizaine | `filled`, `added`, `frames` |
| `numberLine` | Piste graduée : la voiture fait des bonds (avant, arrière, écart), drapeaux sur les repères | `from`, `to`, `step`, `jumps`, `marks`, `decimals`, `fractions` |
| `tensUnits` | Cubes, barres de dix, plaques de cent, blocs de mille ; échanges (casser une dizaine, 10 → 1) | `value`, `actions` |
| `placeValueTable` | Tableau de numération (classes et rangs, partie décimale) ; chiffres qui glissent (× 10, ÷ 10) | `value`, `classes`, `shift` |
| `columnOperation` | Opération posée pas à pas (+, −, ×, ÷) : alignement, retenues en rouge, résultat ; soustraction selon les deux techniques (`cassage` : on barre et on casse une dizaine ; `compensation` : on ajoute 10 unités en haut et 1 dizaine en bas) | `operation`, `operands`, `steps`, `method` (`cassage`, `compensation`) |
| `calcChain` | Calcul mental en chaîne de flèches (47 → +3 → 50 → +5 → 55) | `start`, `steps` |
| `equation` | Égalité qui se construit, case à trou, famille de nombres (triangle +/−, ×/÷) | `terms`, `missing`, `family` |
| `compareNumbers` | Deux nombres face à face, comparaison rang par rang, apparition de `<`, `>` ou `=` | `a`, `b`, `highlightRank` |
| `arrayGrid` | « Grille de départ » : voitures en rangées × colonnes ; découpage pour la distributivité | `rows`, `cols`, `split` |
| `timesTable` | Table (addition ou multiplication) dont les cases s'allument, motifs, symétrie | `operation`, `table`, `highlight` |
| `fractionBar` | Bande-unité partagée en parts égales, parts colorées, bandes empilées pour comparer | `denominator`, `numerators`, `stack` |
| `barModel` | Schéma en barres pour les problèmes (tout / parties, comparaison, fois plus) | `kind`, `values`, `unknown` |
| `proportionTable` | Tableau de proportionnalité avec flèches × k et + entre colonnes | `rows`, `arrows` |
| `sequencePattern` | Suites de nombres ou de motifs (compter de 2 en 2, de 10 en 10, à rebours) | `start`, `step`, `count`, `hidden` |

Le type d'animation et ses paramètres sont validés par `lesson.schema.json` : une leçon qui cite une animation inconnue
ou des paramètres invalides fait échouer les tests, jamais l'application devant l'enfant.

## Critères d'acceptation

- [ ] Les 17 animations existent dans `web/src/components/lesson/animations/`, chacune avec une démonstration sur la
      page de développement (`#/dev/composants`, section Leçons).
- [ ] Couleurs et polices exclusivement par variables CSS du thème ; relecture visuelle par Georges des 17 animations
      côte à côte avec l'écran de série (même univers).
- [ ] Contraste WCAG AA sur tous les textes et nombres ; `prefers-reduced-motion` respecté.
- [ ] Tests Vitest des animations interactives (glisser-déposer, clic, retour en cas d'erreur).
- [ ] Paramètres décrits dans `lesson.schema.json` et documentés dans `docs/specs/animations-lecons.md`.

## Dépendances

- Charte graphique (SUB000105), format des leçons (SUB001409), lecteur de leçons (SUB001410).
