---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXvSA
url: https://github.com/georgesp/Educator/issues/326
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012047 — Leçon CM1 — Probabilités
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 5230
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique ce qui est certain, possible ou impossible, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 2.
- **Compétences liées** (`skillIds`) : `cm1-proba-events`, `cm1-proba-scale` — exercices : la fiche « Exercices CM1 — Probabilités ».
- **Objectif** : classer un événement (certain, possible, impossible) puis le placer sur l'échelle des probabilités.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « certain », « possible », « impossible », « probable », « peu probable », « une chance sur deux ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Probabilités » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Certain** — Le sac ne contient que des billes rouges : je tirerai forcément une rouge. | `drawBag` |
| 3 | **Impossible** — Il n'y a aucune bille verte : tirer une verte est impossible. | `drawBag` |
| 4 | **Possible** — Rouges et bleues mélangées : on ne peut pas savoir à l'avance. | `drawBag` |
| 5 | **À toi** — Interaction : range chaque événement dans le bon garage (certain, possible, impossible). | `groupObjects` (tri, interactif) |
| 6 | **L'échelle** — De « impossible » à « certain » : une piste avec quatre zones. | `probabilityScale` |
| 7 | **Peu probable ou probable** — 3 rouges et 1 bleue : la bleue est peu probable, la rouge probable. | `drawBag` + `probabilityScale` |
| 8 | **Une chance sur deux** — Pile ou face : autant de chances pour l'un que pour l'autre. | `probabilityScale` |
| 9 | **À toi** — Interaction : place trois événements sur l'échelle. | `probabilityScale` (interactif) |
| 10 | **Je retiens** — « Certain : ça arrive à coup sûr. Impossible : ça n'arrive jamais. Sinon, c'est possible, plus ou moins probable. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `drawBag`, `groupObjects`, `probabilityScale`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » ; `drawBag` et `probabilityScale` sont ajoutées par la fiche « Animations des leçons CM1 — heures, mesures, probabilités et données ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Avec un dé à 6 faces, obtenir 7 est… (QCM : certain, possible, impossible)
2. Sac de 3 billes rouges et 1 bleue : tirer la bleue est… (QCM : impossible, peu probable, probable, certain)
3. Pile ou face : combien de chances d'obtenir pile ? (QCM : aucune, une chance sur deux, c'est certain)
4. Lequel est certain : tirer une rouge dans un sac de billes toutes rouges, ou obtenir 6 avec un dé ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-probabilites.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-proba-events`, `cm1-proba-scale`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 3, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » et fiche « Animations des leçons CM1 — heures, mesures, probabilités et données » (`drawBag`, `probabilityScale`).
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
