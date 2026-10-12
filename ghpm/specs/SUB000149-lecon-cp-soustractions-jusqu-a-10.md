---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACYq0
url: https://github.com/georgesp/Educator/issues/117
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000149 — Leçon CP — Soustractions jusqu'à 10
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 1030
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique ce que veut dire enlever et comment trouver ce qui reste, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 1.
- **Compétences liées** (`skillIds`) : `cp-sub-within-10` — exercices : compétences existantes du lot 1 (SUB000108, SUB000112).
- **Objectif** : comprendre la soustraction (enlever, trouver ce qui reste), utiliser le signe −, s'appuyer sur les décompositions (7, c'est 4 et 3).
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « enlever », « moins », « il reste », « soustraction ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Soustractions jusqu'à 10 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Enlever** — 7 voitures dans le garage, 3 partent : il en reste 4. | `groupObjects` (remove) |
| 3 | **Le signe −** — On l'écrit 7 − 3 = 4 : « − » veut dire « on enlève ». | `equation` |
| 4 | **7, c'est 4 et 3** — Le garage montre 4 voitures rouges et 3 bleues : 7 − 3 = 4 et 7 − 4 = 3. | `tenFrame` (deux couleurs) |
| 5 | **À toi** — Interaction : 8 voitures, l'enfant en fait sortir 5 et dit combien il en reste. | `tenFrame` (interactif) |
| 6 | **Reculer** — 9 − 2 : la voiture recule de 2 cases sur la piste. | `numberLine` (arrière) |
| 7 | **L'écart** — 9 − 7 : de 7 à 9, il y a 2 bonds ; c'est plus rapide que reculer de 7. | `numberLine` (écart) |
| 8 | **Addition et soustraction** — 3 + 4 = 7, donc 7 − 4 = 3 et 7 − 3 = 4 : une famille de nombres. | `equation` (famille) |
| 9 | **À toi** — Interaction : 8 − … = 5, l'enfant trouve le nombre caché sur la piste. | `equation` + `numberLine` (interactif) |
| 10 | **Je retiens** — « Enlever, c'est retirer : je cherche ce qui reste. Si 7, c'est 4 et 3, alors 7 − 4 = 3. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `equation`, `groupObjects`, `numberLine`, `recapCard`, `tenFrame`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 6 − 2 = ?
2. 10 − 7 = ? (QCM : 2, 3, 4)
3. 9 − … = 4
4. 5 + 3 = 8, donc 8 − 3 = ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-sub-within-10.json`, validée par `lesson.schema.json`, `skillIds` = `cp-sub-within-10`.
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
