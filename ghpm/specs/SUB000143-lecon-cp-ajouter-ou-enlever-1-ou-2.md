---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYg8
url: https://github.com/georgesp/Educator/issues/111
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000143 — Leçon CP — Ajouter ou enlever 1 ou 2
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 1020
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique comment ajouter ou enlever 1 ou 2 sans tout recompter, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 1.
- **Compétences liées** (`skillIds`) : `cp-add-sub-1-2` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : relier + 1 au nombre suivant, − 1 au nombre précédent, + 2 / − 2 à deux bonds.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « suivant », « précédent », « ajouter », « enlever ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Ajouter ou enlever 1 ou 2 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Le nombre d'après** — La voiture est sur 8. + 1 : elle avance d'une case, 9 : c'est le nombre suivant. | `numberLine` (0 à 14) |
| 3 | **Deux bonds** — 8 + 2 : deux bonds, 9 puis 10. | `numberLine` |
| 4 | **Le nombre d'avant** — 11 − 1 : la voiture recule d'une case, 10 : c'est le nombre précédent. | `numberLine` (arrière) |
| 5 | **Reculer de 2** — 11 − 2 : deux pas en arrière, 10 puis 9. | `numberLine` (arrière) |
| 6 | **À toi** — Interaction : l'enfant fait avancer ou reculer la voiture pour 6 + 2 puis 12 − 1. | `numberLine` (interactif) |
| 7 | **Avec des objets** — 7 casques, on en retire 2 : il en reste 5. | `groupObjects` (remove) |
| 8 | **Compter dans les deux sens** — On récite en avant 10, 11, 12… puis à rebours 12, 11, 10 ; un nombre se cache. | `sequencePattern` |
| 9 | **À toi** — Interaction : choisir la case d'arrivée pour 13 − 2. | `numberLine` (interactif) |
| 10 | **Je retiens** — « Ajouter 1, c'est le nombre d'après. Enlever 1, c'est le nombre d'avant. Pour 2, je fais deux pas. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `groupObjects`, `numberLine`, `recapCard`, `sequencePattern`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 9 + 1 = ?
2. 13 − 2 = ? (QCM : 10, 11, 12)
3. 10 + 2 = ?
4. Quel est le nombre juste avant 7 ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-add-sub-1-2.json`, validée par `lesson.schema.json`, `skillIds` = `cp-add-sub-1-2`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
