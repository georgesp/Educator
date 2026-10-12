---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACalo
rev: 2026-10-11T18:50:55Z
url: https://github.com/georgesp/Educator/issues/208
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012120 — Leçon CM2 — Les grands nombres jusqu'au milliard
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 1660
---

## User story

En tant qu'enfant de CM2, je veux une leçon courte et animée qui m'explique comment lire et écrire des nombres de millions, avant de m'entraîner.

## Leçon

- **Classe / période** : CM2, période 3.
- **Compétences liées** (`skillIds`) : `cm2-num-read-write`, `cm2-num-compare` — exercices : la fiche « Exercices CM2 — Les grands nombres jusqu'au milliard ».
- **Objectif** : ajouter la classe des millions, lire et écrire des nombres de 7 à 9 chiffres, les comparer.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « classe des millions », « million », « milliard (découverte) ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Les grands nombres jusqu'au milliard » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Rappel** — 999 999 : la plus grande à 6 chiffres. + 1 → un million. | `placeValueTable` (classes) |
| 3 | **La classe des millions** — Trois nouvelles colonnes : u, d, c de millions. | `placeValueTable` |
| 4 | **Lire** — 452 380 100 : quatre cent cinquante-deux millions trois cent quatre-vingt mille cent. | `placeValueTable` |
| 5 | **Les zéros** — 23 400 005 : des classes presque vides. | `placeValueTable` |
| 6 | **À toi** — Interaction : écris dans le tableau le nombre dicté. | `placeValueTable` (interactif) |
| 7 | **Comparer** — Nombre de chiffres, puis classe par classe. | `compareNumbers` |
| 8 | **Encadrer** — 23 000 000 < 23 400 005 < 24 000 000. | `numberLine` (pas d'un million) |
| 9 | **Écrire en lettres** — Interaction : choisis la bonne écriture de 23 400 005. « million » prend un s (vingt-trois millions), « mille » jamais ; la graphie de 1990 avec des traits d'union partout est aussi juste. | `equation` (QCM, interactif) |
| 10 | **Je retiens** — « Trois classes : millions, milliers, unités. Je lis chaque classe suivie de son nom : « millions », « mille ». » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `compareNumbers`, `equation`, `numberLine`, `placeValueTable`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Écris en chiffres : vingt-trois millions quatre cent mille cinq.
2. Combien de millions dans 452 380 100 ? (QCM : 2, 452, 452 380)
3. Le plus grand : 98 765 432 ou 123 456 789 ?
4. Encadre 23 400 005 entre deux millions.

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm2-grands-nombres-milliard.json`, validée par `lesson.schema.json`, `skillIds` = `cm2-num-read-write`, `cm2-num-compare`.
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
