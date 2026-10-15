---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wqc
rev: 2026-10-10T19:39:31Z
url: https://github.com/georgesp/Educator/issues/40
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001507 — Objectif du jour
status: In Review
size: S
estimate:
startDate:
endDate:
parent: "#15"
order: 617
---

## User story

En tant qu'enfant, je veux un petit objectif chaque jour afin de prendre l'habitude de m'entraîner un peu.

## Comportement attendu

- Objectif court et atteignable (par défaut : 3 séries), affiché dans le menu avec sa progression.
- Récompense : points bonus et compteur de jours d'affilée.
- Pas de punition en cas de jour manqué (le compteur reprend sans message négatif).

## Critères d'acceptation

- [x] Objectif paramétrable dans les Options parent.
- [x] Passage de jour basé sur la date locale.

## Réalisation

- Objectif par défaut dans `content/rewards/daily-goal.json` : 3 séries, 30 points bonus.
- Compte toutes les séances terminées du jour (séries, quiz de leçon, bilans, défis, révisions ; ni une série abandonnée ni l'évaluation de niveau).
- Le jour où l'objectif est atteint, le bonus est ajouté une seule fois (table `DailyGoalDay`, une ligne par jour atteint) et annoncé en fin de séance (« Objectif du jour atteint : + 30 points ! »).
- Jours d'affilée : jours consécutifs atteints jusqu'à aujourd'hui (ou hier tant qu'aujourd'hui n'est pas atteint). Un jour manqué remet simplement le compteur à zéro, sans aucun message.
- Menu : carte « Objectif du jour » (« 2 séries sur 3 », barres, « 4 jours d'affilée »), cachée quand les parents ont choisi « Aucun ».
- Espace parent → réglages de l'enfant : objectif de 0 (aucun) à 6 séries.
- Jours calculés avec la date locale de l'ordinateur (même fonction que les statistiques, passée au cœur par `Services::start`).
