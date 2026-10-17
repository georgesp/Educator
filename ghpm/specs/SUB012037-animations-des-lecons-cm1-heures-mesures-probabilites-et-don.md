---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXu-I
url: https://github.com/georgesp/Educator/issues/316
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012037 — Animations des leçons CM1 — heures, mesures, probabilités et données"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 5130
---

## User story

En tant qu'enfant de CM1, je veux des animations claires pour comprendre l'heure, les mesures, les figures, les probabilités et les données, dans le même univers que les autres leçons.

## Objectif

Ajouter à la bibliothèque d'animations des leçons (SUB000117, `web/src/components/lesson/animations/`) les animations
demandées par les leçons CM1 des nouveaux thèmes. SUB000117 reste inchangée (16 animations du CP et du CE1, plus
`proportionTable`) ; cette fiche suit les mêmes règles : charte « S1 · Grand Prix » (SUB000105), couleurs et polices
par variables CSS du thème, valeurs calculées par `educator-core::lessons` (jamais par le front), paramètres validés
par `lesson.schema.json`.

## Catalogue

| Animation | Rôle | Paramètres principaux | Leçon |
|---|---|---|---|
| `clockFace` | Horloge à aiguilles : lecture, graduations des minutes, aiguilles déplaçables (la petite avance avec les minutes) | `hours`, `minutes`, `showMinuteMarks`, `interactive` | Lire l'heure et calculer des durées |
| `polygonFigure` | Polygone coté dont le contour s'allume côté par côté (périmètre) | `sides`, `unit`, `highlight`, `interactive` | Périmètres et aires |
| `gridFigure` | Figure sur quadrillage : carreaux et demi-carreaux coloriés, comptage, comparaison de deux figures | `cells`, `halfCells`, `compareWith`, `interactive` | Périmètres et aires |
| `drawBag` | Sac de billes de couleurs (avec un motif en plus de la couleur) ; aucun tirage aléatoire | `balls`, `highlight` | Probabilités |
| `probabilityScale` | Échelle impossible → peu probable → probable → certain ; événements à glisser | `events`, `interactive` | Probabilités |
| `dataTable` | Tableau à simple ou double entrée ; croisement ligne / colonne qui s'allume | `rows`, `columns`, `values`, `highlight`, `interactive` | Lire et utiliser des données |
| `barChart` | Diagramme en barres, axe gradué ; barre réglable par l'enfant | `labels`, `values`, `step`, `editable` | Lire et utiliser des données |
| `lineChart` | Courbe simple (points reliés), lecture d'un point | `labels`, `values`, `step`, `highlight` | Lire et utiliser des données |
| `placeValueTable` (variante) | Colonnes d'unités de mesure (km … mm, t … mg, hL … mL) en plus des rangs | `units` | Longueurs, masses et contenances |

## Critères d'acceptation

- [ ] Les 8 nouvelles animations et la variante de `placeValueTable` existent, chacune avec une démonstration sur la page de développement (`#/dev/composants`, section Leçons).
- [ ] Couleurs et polices exclusivement par variables CSS du thème ; relecture visuelle par Georges côte à côte avec les animations existantes.
- [ ] Contraste WCAG AA sur tous les textes et nombres ; `prefers-reduced-motion` respecté ; couleurs jamais seules porteuses d'information (motifs).
- [ ] Tests Vitest des animations interactives (glisser, clic, retour en cas d'erreur), utilisables à la souris et au doigt (tablette).
- [ ] Paramètres décrits dans `lesson.schema.json` et documentés dans `docs/specs/animations-lecons.md`.

## Dépendances

- Bibliothèque d'animations des leçons de mathématiques (SUB000117), charte graphique (SUB000105), format des leçons (SUB001409), lecteur de leçons (SUB001410).
- Les composants d'exercice du même nom (horloge, figures, sac de billes, diagrammes) des fiches « Exercices CM1 » partagent le dessin SVG.
