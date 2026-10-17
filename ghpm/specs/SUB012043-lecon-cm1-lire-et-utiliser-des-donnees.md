---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXvK4
url: https://github.com/georgesp/Educator/issues/322
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012043 — Leçon CM1 — Lire et utiliser des données
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 5190
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique comment lire un tableau, un diagramme en barres et une courbe, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 3.
- **Compétences liées** (`skillIds`) : `cm1-data-read`, `cm1-data-problems` — exercices : la fiche « Exercices CM1 — Lire et utiliser des données ».
- **Objectif** : croiser ligne et colonne, lire la graduation d'un axe, utiliser les données pour répondre à une question.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « tableau », « ligne », « colonne », « diagramme en barres », « courbe », « graduation ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Lire et utiliser des données » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Un tableau** — Les tours bouclés par chaque pilote : une ligne par pilote. | `dataTable` |
| 3 | **Double entrée** — Pilotes et courses : je suis la ligne de Sam et la colonne de la 2e course. | `dataTable` (croisement) |
| 4 | **À toi** — Interaction : clique sur la case des points de Léa à la 3e course. | `dataTable` (interactif) |
| 5 | **Le diagramme en barres** — Les points par écurie : plus la barre est haute, plus il y a de points. | `barChart` |
| 6 | **Lire la graduation** — L'axe va de 5 en 5 : la barre s'arrête sur 35. | `barChart` (zoom sur l'axe) |
| 7 | **À toi** — Interaction : monte la barre de l'écurie bleue à la valeur du tableau. | `barChart` (interactif) |
| 8 | **Une courbe** — La vitesse de la voiture minute après minute : le point le plus haut est le plus rapide. | `lineChart` |
| 9 | **Un problème** — Combien de points d'écart entre la 1re et la 2e écurie ? 45 − 35 = 10. | `barChart` + `calcChain` |
| 10 | **Je retiens** — « Je lis le titre, puis les lignes et les colonnes, ou la graduation de l'axe. Ensuite je cherche le calcul qui répond à la question. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `barChart`, `calcChain`, `dataTable`, `lineChart`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » ; `dataTable`, `barChart` et `lineChart` sont ajoutées par la fiche « Animations des leçons CM1 — heures, mesures, probabilités et données ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Dans le tableau, combien de tours pour Léa ?
2. Le diagramme va de 5 en 5 : quelle est la valeur de la barre ? (QCM : 7, 30, 35)
3. Quelle écurie a le plus de points ?
4. Combien de points d'écart entre la 1re et la 2e écurie ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-lire-des-donnees.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-data-read`, `cm1-data-problems`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 3, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » et fiche « Animations des leçons CM1 — heures, mesures, probabilités et données » (`dataTable`, `barChart`, `lineChart`).
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
