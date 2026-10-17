---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaDs
rev: 2026-10-11T18:50:44Z
url: https://github.com/georgesp/Educator/issues/178
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012024 — Leçon CM1 — Égalités à trous, programmes de calcul et suites"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1570
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique comment trouver un nombre caché, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 2.
- **Compétences liées** (`skillIds`) : `cm1-algebra-missing`, `cm1-calc-program` — exercices : la fiche « Exercices CM1 — Égalités à trous, programmes de calcul et suites ».
- **Objectif** : trouver un nombre manquant avec l'opération inverse, exécuter un programme de calcul, trouver la règle d'une suite.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « égalité », « nombre inconnu », « programme de calcul », « suite », « règle ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Égalités à trous, programmes de calcul et suites » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **La case vide** — … + 37 = 92 : sur le schéma, le tout est 92, une partie 37. | `barModel` + `equation` |
| 3 | **Le calcul à l'envers** — … = 92 − 37 = 55 ; vérification : 55 + 37 = 92. | `equation` (famille) |
| 4 | **Avec × et ÷** — 6 × ★ = 54 → ★ = 54 ÷ 6 = 9. | `equation` |
| 5 | **À toi** — Interaction : 2 boîtes identiques et 3 font 11 ; combien vaut une boîte ? Le schéma en barres montre le calcul. | `barModel` (interactif) |
| 6 | **Programme de calcul** — Choisis 7 → × 3 → + 5 : la voiture passe trois stands, 21 puis 26. | `calcChain` |
| 7 | **À toi** — Interaction : applique le programme à 10. | `calcChain` (interactif) |
| 8 | **Suites** — 4, 7, 10, 13… : on ajoute 3 à chaque fois. | `sequencePattern` |
| 9 | **Motifs** — Des drapeaux : 1, 3, 5… combien à l'étape 6 ? | `sequencePattern` (motifs) |
| 10 | **Je retiens** — « Pour trouver un nombre caché, je fais le calcul « à l'envers » : … + 37 = 92, donc … = 92 − 37. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `barModel`, `calcChain`, `equation`, `recapCard`, `sequencePattern`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. … + 37 = 92
2. 6 × ★ = 54 (QCM : 8, 9, 48)
3. Choisis 7, multiplie par 3, ajoute 5 : résultat ?
4. 4, 7, 10, 13… nombre suivant ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-egalites-programmes-de-calcul.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-algebra-missing`, `cm1-calc-program`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 3, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
