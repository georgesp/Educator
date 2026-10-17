---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdqTI
url: https://github.com/georgesp/Educator/issues/341
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB033506 — Réglage parent : anecdotes activées ou non"
status: Backlog
size: XS
estimate:
startDate:
endDate:
parent: "#335"
order: 656
---

## User story

En tant que parent, je veux pouvoir désactiver les anecdotes pour un enfant qui se disperse facilement.

## Comportement attendu

- Espace parent → « Réglages de … » : interrupteur **Anecdotes « Le savais-tu ? »** (activé par défaut), stocké dans `ProfileSetting` (SUB001512).
- Désactivé : aucune carte dans les leçons, aucune tuile en fin de série ; le carnet reste consultable.

## Critères d'acceptation

- [ ] Réglage pris en compte tout de suite, sans redémarrage.
- [ ] Test : avec le réglage désactivé, aucune anecdote n'est proposée.
