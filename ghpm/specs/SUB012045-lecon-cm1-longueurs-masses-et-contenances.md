---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXvOk
url: https://github.com/georgesp/Educator/issues/324
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012045 — Leçon CM1 — Longueurs, masses et contenances"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 5210
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique les unités de longueur, de masse et de contenance, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 5.
- **Compétences liées** (`skillIds`) : `cm1-measure-units`, `cm1-measure-convert` — exercices : la fiche « Exercices CM1 — Longueurs, masses et contenances ».
- **Objectif** : choisir l'unité adaptée, connaître les relations entre les unités et convertir avec le tableau.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « longueur », « masse », « contenance », « unité », « kilomètre », « tonne », « litre ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Longueurs, masses et contenances » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Les longueurs** — Du millimètre (l'épaisseur d'une pièce) au kilomètre (un tour de circuit). | `placeValueTable` (unités) |
| 3 | **1 km = 1 000 m** — La voiture parcourt 1 000 bonds de 1 m. | `numberLine` |
| 4 | **À toi** — Interaction : range chaque objet dans le garage de la bonne unité (mm, cm, m, km). | `groupObjects` (tri, interactif) |
| 5 | **Le tableau des unités** — 2 m 45 cm : les chiffres se placent, on lit 245 cm. | `placeValueTable` (unités) |
| 6 | **Les masses** — Le camion de l'écurie : 3 t = 3 000 kg ; un casque, environ 1 kg = 1 000 g. | `placeValueTable` (unités) |
| 7 | **Les contenances** — Le bidon : 1 L = 100 cL = 1 000 mL ; 1 hL = 100 L. | `placeValueTable` (unités) |
| 8 | **Comparer** — 3 km et 2 900 m : on écrit les deux en mètres, 3 000 m > 2 900 m. | `compareNumbers` |
| 9 | **À toi** — Interaction : choisis le signe entre 4 kg et 3 800 g. | `compareNumbers` (interactif) |
| 10 | **Je retiens** — « Pour comparer ou calculer, j'écris les mesures dans la même unité. 1 km = 1 000 m ; 1 m = 100 cm ; 1 kg = 1 000 g ; 1 L = 100 cL. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `compareNumbers`, `groupObjects`, `numberLine`, `placeValueTable` (variante « unités de mesure », ajoutée par la fiche « Animations des leçons CM1 — heures, mesures, probabilités et données »), `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Quelle unité pour la longueur d'un circuit ? (QCM : mm, m, km)
2. 2 m 45 cm = … cm
3. 3 km … 2 900 m (QCM : <, >, =)
4. 1 L = … cL (QCM : 10, 100, 1 000)

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-longueurs-masses-contenances.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-measure-units`, `cm1-measure-convert`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 3, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » et fiche « Animations des leçons CM1 — heures, mesures, probabilités et données » (variante « unités de mesure » de `placeValueTable`).
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
