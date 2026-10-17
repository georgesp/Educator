---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXouk
url: https://github.com/georgesp/Educator/issues/307
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011950 — Leçon CE2 — Masses et contenances
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1244
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique les grammes, les kilogrammes, la tonne et les
litres, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 4.
- **Compétences liées** (`skillIds`) : `ce2-mass`, `ce2-capacity` — exercices : la fiche « Exercices CE2 — Masses et contenances ».
- **Objectif** : lire une balance et un récipient gradué, connaître les relations entre g, kg, t et entre L, dL, cL, estimer.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « masse », « gramme », « kilogramme », « tonne », « contenance », « litre », « décilitre », « centilitre ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Masses et contenances » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Peser** — Un casque sur la balance, des masses marquées de l'autre côté : équilibre à 500 g + 200 g + 50 g = 750 g. | `scale` |
| 3 | **À toi** — Interaction : pose les masses pour équilibrer un pneu de 1 kg 300 g. | `scale` (interactif) |
| 4 | **Les unités** — 1 kg = 1 000 g ; une voiture pèse environ 1 t = 1 000 kg. | `conversionTable` |
| 5 | **Décomposer** — 5 462 g = 5 kg 462 g. | `conversionTable` |
| 6 | **Estimer** — Une plume (g), un sac de courses (kg), un camion (t). | `calcChain` (association) |
| 7 | **La contenance** — Le réservoir se remplit : 1 L = 10 dL = 100 cL. | `measuringJug` |
| 8 | **À toi** — Interaction : remplis la gourde jusqu'à 75 cL. | `measuringJug` (interactif) |
| 9 | **Décomposer** — 780 cL = 7 L 80 cL ; un verre ≈ 20 cL, une bouteille ≈ 1 L. | `conversionTable` |
| 10 | **Je retiens** — « 1 t = 1 000 kg ; 1 kg = 1 000 g. 1 L = 10 dL = 100 cL. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive se fait dans le fichier JSON.

## Animations

- Utilisées : `calcChain`, `conversionTable`, `measuringJug`, `recapCard`, `scale`, `titleCard`.
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105), comme les autres leçons de mathématiques.

## Quiz de fin de leçon (SUB001411)

1. 1 kg = … g
2. 500 g + 200 g + 50 g sur la balance : combien pèse l'objet, en g ?
3. Un verre d'eau contient : 20 cL, 20 L ou 2 L ? (QCM)
4. 1 L = … cL

Chaque question a une explication courte, affichée après la réponse et lisible au clic.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-masses-contenances.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-mass`, `ce2-capacity`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `scale`, `measuringJug`, `conversionTable`.
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
