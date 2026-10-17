---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoxM
url: https://github.com/georgesp/Educator/issues/308
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011951 — Leçon CE2 — Payer et rendre la monnaie
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1248
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'apprend à compter l'argent et à rendre la monnaie,
avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 2.
- **Compétences liées** (`skillIds`) : `ce2-money-make`, `ce2-money-change`, `ce2-measure-problems` — exercices : la fiche « Exercices CE2 — Payer et rendre la monnaie ».
- **Objectif** : savoir que 1 € = 100 c, écrire et composer une somme, rendre la monnaie en complétant, résoudre un problème de mesures.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « euro », « centime », « prix », « payer », « rendre la monnaie ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Payer et rendre la monnaie » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Pièces et billets** — Les pièces en centimes, les pièces en euros, les billets ; 1 € = 100 c. | `coins` |
| 3 | **Écrire une somme** — 3 € et 68 c s'écrivent 3,68 € : la virgule sépare les euros des centimes. | `coins` + `placeValueTable` |
| 4 | **À toi** — Interaction : mets 2,35 € dans le porte-monnaie. | `coins` (interactif) |
| 5 | **Le moins de pièces** — 2,35 € = 2 € + 20 c + 10 c + 5 c. | `coins` |
| 6 | **Rendre la monnaie** — 3,68 € payés avec 5 € : on complète comme le marchand, 3,70 € (2 c), 4 € (30 c), 5 € (1 €) → 1,32 €. | `coins` + `numberLine` (bonds) |
| 7 | **À toi** — Interaction : rends la monnaie sur 10 € pour un livre à 7,40 €. | `coins` (interactif) |
| 8 | **Vérifier** — Prix + monnaie rendue = somme donnée : 3,68 € + 1,32 € = 5 €. | `calcChain` |
| 9 | **Un problème de mesures** — Un ruban de 2 m, on coupe 75 cm : 200 cm − 75 cm = 125 cm = 1 m 25 cm ; on calcule dans la même unité. | `barModel` + `conversionTable` |
| 10 | **Je retiens** — « 1 € = 100 c. Pour rendre la monnaie, je complète jusqu'à la somme donnée. Dans un problème, je calcule avec la même unité. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive se fait dans le fichier JSON.

## Animations

- Utilisées : `barModel`, `calcChain`, `coins`, `conversionTable`, `numberLine`, `placeValueTable`, `recapCard`, `titleCard`.
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) ; pièces et billets dessinés pour Educator.

## Quiz de fin de leçon (SUB001411)

1. 1 € = … c
2. 3 € et 68 c s'écrivent : 3,68 €, 36,8 € ou 368 € ? (QCM)
3. On paie 3,68 € avec 5 € : combien de centimes rend-on ? (132)
4. 2 m − 75 cm = … cm

Chaque question a une explication courte, affichée après la réponse et lisible au clic.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-monnaie.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-money-make`, `ce2-money-change`, `ce2-measure-problems`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `coins`, `conversionTable`, `barModel` ; quiz à choix en mots.
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
