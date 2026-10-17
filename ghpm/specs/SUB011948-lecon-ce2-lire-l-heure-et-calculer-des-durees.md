---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoog
url: https://github.com/georgesp/Educator/issues/305
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011948 — Leçon CE2 — Lire l'heure et calculer des durées
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1246
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte et animée qui m'apprend à lire l'heure et à calculer une durée, avant
de m'entraîner.

## Leçon

- **Classe / période** : CE2, période 1.
- **Compétences liées** (`skillIds`) : `ce2-clock-read`, `ce2-clock-set`, `ce2-durations` — exercices : la fiche « Exercices CE2 — Lire l'heure et calculer des durées ».
- **Objectif** : lire et placer les aiguilles, savoir que 1 h = 60 min, calculer une durée dans une journée.
- **Durée visée** : 3 à 5 minutes, 11 slides.
- **Vocabulaire du programme** : « heure », « minute », « grande aiguille », « petite aiguille », « durée », « matin », « après-midi ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Lire l'heure et calculer des durées » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Deux aiguilles** — La petite montre les heures, la grande les minutes ; la grande fait le tour en 60 minutes pendant que la petite avance d'une heure. | `clock` |
| 3 | **Heures pile, demies, quarts** — 3 h, 3 h 30 (la grande sur le 6), 3 h 15, 3 h 45. | `clock` |
| 4 | **Compter de 5 en 5** — Chaque chiffre vaut 5 minutes pour la grande aiguille : sur le 7, c'est 35 min → 4 h 35. | `clock` + `sequencePattern` |
| 5 | **À toi** — Interaction : place les aiguilles sur 8 h 45. | `clock` (interactif) |
| 6 | **Matin et après-midi** — La petite aiguille fait deux tours par jour : 2 h 30 de l'après-midi s'écrit 14 h 30. | `clock` + `numberLine` (0 à 24 h) |
| 7 | **1 h = 60 min** — 2 h 20 min = 60 + 60 + 20 = 140 min. | `calcChain` |
| 8 | **Une durée** — Départ 9 h 45, arrivée 11 h 10 : 15 min jusqu'à 10 h, 1 h jusqu'à 11 h, 10 min → 1 h 25 min. | `clock` (arc de durée) + `numberLine` |
| 9 | **À toi** — Interaction : le film commence à 14 h 30 et dure 1 h 45 ; avance l'horloge et lis l'heure de fin. | `clock` (interactif) |
| 10 | **Vérifier** — L'heure de fin moins la durée redonne l'heure de début. | `calcChain` |
| 11 | **Je retiens** — « La petite aiguille montre les heures, la grande les minutes. 1 h = 60 min. Pour une durée, j'avance par bonds jusqu'aux heures pile. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive se fait dans le fichier JSON.

## Animations

- Utilisées : `calcChain`, `clock`, `numberLine`, `recapCard`, `sequencePattern`, `titleCard`.
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105), comme les autres leçons de mathématiques.

## Quiz de fin de leçon (SUB001411)

1. Combien de minutes dans 1 heure ?
2. La grande aiguille est sur le 7, la petite entre le 4 et le 5 : quelle heure est-il ? (QCM : 4 h 35, 7 h 20, 4 h 07)
3. 2 h 20 min = … min
4. De 9 h 45 à 11 h 10, combien de minutes ?

Chaque question a une explication courte, affichée après la réponse et lisible au clic.

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/ce2-heure-durees.json`, validée par `lesson.schema.json`, `skillIds` = `ce2-clock-read`, `ce2-clock-set`, `ce2-durations`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `clock` ; quiz à choix en mots (heures).
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
