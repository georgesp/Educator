---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdqG4
rev: 2026-10-17T13:47:07Z
url: https://github.com/georgesp/Educator/issues/334
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012224 — Leçon CP — Problèmes en deux étapes et problèmes variés
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2072
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'aide à résoudre des histoires en deux étapes et à trouver moi-même quelle opération faire, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 5.
- **Compétences liées** (`skillIds`) : `cp-problems-two-steps`, `cp-problems-mixed` — exercices : la fiche « Exercices CP — Résoudre des problèmes ».
- **Objectif** : résoudre une histoire en deux étapes (nombres ≤ 30) ; reconnaître une histoire où l'on ajoute, où l'on enlève, où l'on fait des paquets égaux ou un partage.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « problème », « étape », « paquet », « partager », « autant ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Problèmes en deux étapes et problèmes variés » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Une histoire en deux temps** — « Sur la grille, il y a 29 voitures. 12 partent au stand. Puis 7 reviennent. » (au clic) | `groupObjects` |
| 3 | **Première étape** — 12 voitures partent : il en reste 17. | `groupObjects` (remove) |
| 4 | **Deuxième étape** — 7 reviennent : 17 + 7 = 24 voitures. | `groupObjects` (join) + `calcChain` |
| 5 | **À toi** — Interaction : joue l'histoire « 25 casques, on en range 10, puis on en apporte 4 ». | `groupObjects` (interactif) |
| 6 | **Des paquets égaux** — 3 boîtes de 7 pneus : 7 + 7 + 7 = 21 pneus. | `groupObjects` (groupBy 7) + `calcChain` |
| 7 | **Un partage** — 18 drapeaux pour 3 pilotes, autant chacun : 6 drapeaux chacun. | `groupObjects` (share) |
| 8 | **Quelle histoire ?** — Ajouter, enlever, faire des paquets ou partager : on écoute ce qui se passe. | `barModel` (parts / groups) |
| 9 | **À toi** — Interaction : choisis le schéma de l'histoire « 24 enfants font des équipes de 4 ». | `barModel` (QCM) |
| 10 | **Je retiens** — « Une histoire peut avoir deux étapes : je fais la première, puis la deuxième. Je cherche ce qui se passe : on ajoute, on enlève, on fait des paquets égaux ou on partage. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `barModel`, `calcChain`, `groupObjects`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 29 voitures ; 12 partent, puis 7 reviennent. Combien de voitures ? (saisie)
2. 3 boîtes de 7 pneus : combien de pneus ? (saisie)
3. 18 drapeaux pour 3 pilotes : combien chacun ? (QCM : 6, 15, 21)
4. Quel schéma correspond à l'histoire ? (QCM)

Chaque question a une explication courte, affichée après la réponse et lisible au clic (rien n'est lu tout seul, décision du 2026-10-15).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [x] Leçon `content/lessons/math/cp-problemes-deux-etapes.json`, validée par `lesson.schema.json`, `skillIds` = `cp-problems-two-steps`, `cp-problems-mixed`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [x] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [x] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [x] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
