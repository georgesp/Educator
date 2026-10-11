---
type: Issue
title: Leçon CM1 — Comparer, ranger et encadrer les grands nombres
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-2-cm1.md"
order: 1740
---
## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique comment comparer et encadrer les grands nombres, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 4.
- **Compétences liées** (`skillIds`) : `cm1-num-compare`, `cm1-num-frame` — exercices : la fiche « Exercices CM1 — Comparer, ranger et encadrer les grands nombres ».
- **Objectif** : comparer par le nombre de chiffres puis rang par rang ; encadrer à une unité donnée.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « encadrer », « intercaler », « ordre croissant », « ordre décroissant ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Comparer, ranger et encadrer les grands nombres » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Nombre de chiffres** — 98 750 < 102 000 : 5 chiffres contre 6. | `compareNumbers` |
| 3 | **Rang par rang** — 452 310 et 452 301 : la différence est aux dizaines. | `compareNumbers` (rang en rouge) |
| 4 | **À toi** — Interaction : choisis le signe. | `compareNumbers` (interactif) |
| 5 | **Ranger** — Cinq voitures numérotées à placer sur la grille de départ. | `sequencePattern` (ordre, interactif) |
| 6 | **Encadrer** — 530 000 < 538 214 < 540 000 : deux drapeaux sur la piste. | `numberLine` (pas de 10 000) |
| 7 | **Changer de précision** — Au millier : 538 000 < 538 214 < 539 000. | `numberLine` (zoom) |
| 8 | **Intercaler** — Un nombre entre 452 300 et 452 310 ? | `numberLine` |
| 9 | **À toi** — Interaction : place 250 000 sur la piste de 0 à 1 000 000. | `numberLine` (interactif) |
| 10 | **Je retiens** — « Plus de chiffres = plus grand. Même nombre de chiffres : je compare de gauche à droite jusqu'au premier chiffre différent. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `compareNumbers`, `numberLine`, `recapCard`, `sequencePattern`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 452 310 … 452 301 (QCM : <, >, =)
2. Encadre 538 214 entre deux dizaines de mille.
3. Range dans l'ordre croissant : 120 450, 102 450, 120 405.
4. Le plus grand : 99 999 ou 100 001 ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-comparer-grands-nombres.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-num-compare`, `cm1-num-frame`.
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
