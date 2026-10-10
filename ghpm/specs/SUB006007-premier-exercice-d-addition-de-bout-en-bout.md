---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fSg
url: https://github.com/georgesp/Educator/issues/67
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006007 — Premier exercice d'addition de bout en bout
status: Todo
size: S
estimate:
startDate:
endDate:
parent: "#60"
order: 8
---

## User story

En tant que développeur, je veux un exercice minimal qui traverse toutes les couches afin de valider l'architecture avant le lot 1.

## Comportement attendu

- Le host génère une addition aléatoire simple (deux nombres de 0 à 9) via `Educator.Core`.
- Le front l'affiche, l'utilisateur tape la réponse, le host la valide.
- Un bouton lit le calcul avec Piper.
- Code jetable ou réutilisé par le moteur du lot 1, sans contrainte de design.

## Critères d'acceptation

- [ ] Parcours fonctionnel sur macOS et Windows.
- [ ] Temps de réponse ressenti instantané (< 200 ms hors synthèse vocale).
