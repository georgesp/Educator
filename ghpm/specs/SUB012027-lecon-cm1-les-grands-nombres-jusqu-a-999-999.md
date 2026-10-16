---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaIE
rev: 2026-10-11T18:50:02Z
url: https://github.com/georgesp/Educator/issues/181
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012027 — Leçon CM1 — Les grands nombres jusqu'à 999 999
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1290
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique comment lire et écrire les grands nombres, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 3.
- **Compétences liées** (`skillIds`) : `cm1-num-read-write`, `cm1-num-decompose` — exercices : la fiche « Exercices CM1 — Les grands nombres jusqu'à 999 999 ».
- **Objectif** : comprendre les classes (unités, milliers), lire un nombre classe par classe et décomposer jusqu'à 999 999.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « classe des milliers », « classe des unités », « centaine de mille », « dizaine de mille », « chiffre », « nombre ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Les grands nombres jusqu'à 999 999 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Rappel** — 1 000 = 10 centaines : le gros bloc. | `tensUnits` (bloc) |
| 3 | **10 milliers** — 10 blocs de mille : une dizaine de mille, 10 000. | `tensUnits` + `placeValueTable` |
| 4 | **Les classes** — Le tableau à deux classes : milliers (c, d, u) et unités (c, d, u). | `placeValueTable` (classes) |
| 5 | **Lire 538 214** — « cinq cent trente-huit mille deux cent quatorze » : on lit la classe, puis « mille ». | `placeValueTable` |
| 6 | **Les zéros** — 204 030 : les zéros gardent les places vides. | `placeValueTable` |
| 7 | **À toi** — Interaction : place les chiffres du nombre dicté dans le tableau. | `placeValueTable` (interactif) |
| 8 | **Chiffre ou nombre** — Dans 538 214, chiffre des milliers 8, nombre de milliers 538. | `placeValueTable` |
| 9 | **Écrire en lettres** — Interaction : choisis la bonne écriture de 204 030. « mille » est invariable : deux cent quatre mille trente ; la graphie de 1990 (deux-cent-quatre-mille-trente) est aussi juste. | `equation` (QCM, interactif) |
| 10 | **Je retiens** — « Je sépare les chiffres par classes de trois depuis la droite, puis je lis chaque classe suivie de son nom : « mille ». » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `equation`, `placeValueTable`, `recapCard`, `tensUnits`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Écris en chiffres : deux cent quatre mille trente.
2. Combien de milliers dans 538 214 ? (QCM : 8, 538, 5)
3. 300 000 + 5 000 + 70 + 2 = ?
4. Quel est le chiffre des dizaines de mille de 746 205 ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-grands-nombres-999999.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-num-read-write`, `cm1-num-decompose`.
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
