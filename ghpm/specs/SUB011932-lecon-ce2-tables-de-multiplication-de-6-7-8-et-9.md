---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZj4
url: https://github.com/georgesp/Educator/issues/154
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011932 — Leçon CE2 — Tables de multiplication de 6, 7, 8 et 9"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1440
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique des astuces pour retenir les tables les plus difficiles, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 3.
- **Compétences liées** (`skillIds`) : `ce2-mul-tables-6-9`, `ce2-mul-tables-all` — exercices : la fiche « Exercices CE2 — Tables de multiplication de 6, 7, 8 et 9 ».
- **Objectif** : retrouver les produits des tables de 6 à 9 avec la commutativité, les doubles et l'astuce du 9.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « produit », « facteur », « double », « table ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Tables de multiplication de 6, 7, 8 et 9 » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Ce que je sais déjà** — Dans la table, les cases déjà connues (2, 3, 4, 5, 10) s'allument : il en reste peu ! | `timesTable` |
| 3 | **Commutativité** — 6 × 4 = 4 × 6 : la table est symétrique. | `timesTable` (symétrie) |
| 4 | **Table de 6** — 6 × 7 = 5 × 7 + 7 = 42. | `arrayGrid` (découpage) |
| 5 | **Table de 8** — 8 × 7 = double de 4 × 7 = 56. | `arrayGrid` (deux moitiés) |
| 6 | **Table de 9** — 9 × 6 = 10 × 6 − 6 = 54 ; les chiffres font toujours 9. | `calcChain` + `timesTable` |
| 7 | **À toi** — Interaction : découpe la grille 7 × 8 en deux parties connues. | `arrayGrid` (interactif) |
| 8 | **Facteur manquant** — … × 7 = 63 : je cherche dans la table de 7. | `timesTable` + `equation` |
| 9 | **À toi** — Interaction : course aux produits de la table de 7. | `timesTable` (interactif) |
| 10 | **Je retiens** — « Ce que je sais déjà m'aide : 7 × 6 = 6 × 7, 8 × … = double de 4 × …, 9 × … = 10 × … − …. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `arrayGrid`, `calcChain`, `equation`, `recapCard`, `timesTable`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. 7 × 8 = ?
2. 9 × 6 = ? (QCM : 45, 54, 56)
3. … × 7 = 63
4. 6 × 7 = 5 × 7 + … ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-tables-de-multiplication-6-a-9.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-mul-tables-6-9`, `ce2-mul-tables-all`.
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
