---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoq4
url: https://github.com/georgesp/Educator/issues/306
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011949 — Leçon CE2 — Longueurs
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1242
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'explique les unités de longueur, la règle graduée et
le périmètre, avant de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 1.
- **Compétences liées** (`skillIds`) : `ce2-length-measure`, `ce2-length-convert`, `ce2-perimeter` — exercices : la fiche « Exercices CE2 — Longueurs ».
- **Objectif** : mesurer à la règle, connaître les relations entre km, m, dm, cm et mm, choisir l'unité, calculer un périmètre.
- **Durée visée** : 3 à 5 minutes, 11 slides.
- **Vocabulaire du programme** : « longueur », « mètre », « décimètre », « centimètre », « millimètre », « kilomètre », « périmètre ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Les longueurs » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **La règle** — On place le 0 au bout du segment ; on lit le nombre à l'autre bout : 7 cm. | `ruler` |
| 3 | **Les millimètres** — Entre deux centimètres, 10 petits traits : 1 cm = 10 mm ; 7 cm 4 mm = 74 mm. | `ruler` (zoom) |
| 4 | **À toi** — Interaction : allonge le segment jusqu'à 9 cm 5 mm. | `ruler` (interactif) |
| 5 | **Le mètre** — 1 m = 10 dm = 100 cm : une piste de 1 m découpée en 10 puis en 100. | `conversionTable` + `ruler` |
| 6 | **Le kilomètre** — Un tour de circuit de 1 000 m = 1 km ; 6 km = 6 000 m. | `numberLine` (pas de 1 000) |
| 7 | **Décomposer** — 215 cm = 2 m 1 dm 5 cm : chaque chiffre dans sa colonne d'unité. | `conversionTable` |
| 8 | **Quelle unité ?** — Une fourmi (mm), une trousse (cm), une piste (m), une course de voitures (km). | `calcChain` (association) |
| 9 | **Le périmètre** — La voiture fait le tour du rectangle 7 cm × 3 cm : 7 + 3 + 7 + 3 = 20 cm. | `ruler` + `calcChain` |
| 10 | **À toi** — Interaction : calcule le tour d'un triangle de côtés 4 cm, 5 cm et 6 cm. | `calcChain` (interactif) |
| 11 | **Je retiens** — « 1 km = 1 000 m ; 1 m = 10 dm = 100 cm ; 1 cm = 10 mm. Le périmètre, c'est la longueur du tour de la figure. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `conversionTable`, `numberLine`, `recapCard`, `ruler`, `titleCard`.
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105), comme les autres leçons de mathématiques.

## Quiz de fin de leçon (SUB001411)

1. Combien de millimètres dans 1 cm ?
2. 6 km = … m
3. Une trousse mesure : 2 cm, 20 cm ou 2 m ? (QCM)
4. Périmètre d'un rectangle de 7 cm sur 3 cm, en cm ?

Chaque question a une explication courte, affichée après la réponse et lisible au clic.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-longueurs.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-length-measure`, `ce2-length-convert`, `ce2-perimeter`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `ruler`, `conversionTable`.
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
