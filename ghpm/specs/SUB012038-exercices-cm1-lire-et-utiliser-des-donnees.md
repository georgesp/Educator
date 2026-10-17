---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXvAA
url: https://github.com/georgesp/Educator/issues/317
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012038 — Exercices CM1 — Lire et utiliser des données
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 5140
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à lire un tableau, un diagramme en barres ou une courbe, et à résoudre des problèmes avec ces données, afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1, organisation et gestion de données : recueillir des données et les présenter dans un tableau, un diagramme en barres ou un nuage de points ; lire et interpréter les données d'un tableau à simple ou double entrée, d'un diagramme en barres ou d'une courbe ; résoudre des problèmes en une ou plusieurs étapes à partir de ces données.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-data-read` | 3 | Lire un tableau, un diagramme ou une courbe | Lire une valeur dans un tableau à simple ou double entrée, un diagramme en barres (axe gradué de 1 en 1, 2 en 2, 5 en 5 ou 10 en 10), une courbe simple ; compléter une barre manquante à partir d'un tableau | `result`, `choice`, `barChartFill` |
| `cm1-data-problems` | 5 | Résoudre des problèmes avec des données | Problèmes en une ou plusieurs étapes (écart, total, comparaison, plus grande ou plus petite valeur) à partir d'un tableau, d'un diagramme en barres ou d'une courbe | `problem` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Tableau des tours bouclés par pilote : combien de tours pour Léa ?
- Tableau à double entrée (pilotes × courses) : combien de points pour Sam à la 2e course ?
- Diagramme en barres des points par écurie : quelle écurie a le plus de points ? Combien de points d'écart entre la 1re et la 2e ?
- Courbe de la vitesse d'une voiture : à quelle minute roule-t-elle le plus vite ?

## Évolutions du moteur et de l'écran de série

- Tableaux, diagrammes en barres et courbes SVG générés à partir des données de l'exercice (couleurs du thème, motifs en plus des couleurs).
- Format `barChartFill` : l'enfant règle la hauteur d'une barre (glisser, pas de la graduation).
- La consigne est lue par Piper ; les données restent affichées (pas de lecture du tableau entier).
- Propriété testée : la réponse attendue est lisible sur la graduation (pas de valeur entre deux graduations sans repère).

## Distracteurs (QCM)

Mauvaise ligne ou mauvaise colonne, graduation de l'axe mal lue (de 5 en 5 lue de 1 en 1), total donné au lieu de l'écart, plus grande barre confondue avec la plus grande valeur d'un autre axe.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Lire et utiliser des données » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
