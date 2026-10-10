---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WwM
url: https://github.com/georgesp/Educator/issues/44
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001511 — Espace parent — statistiques de progression
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#15"
order: 510
---

## User story

En tant que parent, je veux voir les progrès de chaque enfant afin de savoir où il en est et sur quoi l'aider.

## Comportement attendu

Pour chaque profil :

- temps passé par jour et par semaine ;
- taux de réussite par compétence, avec un état **maîtrisée / en cours / à revoir** ;
- évolution dans le temps (graphique simple de la réussite par semaine) ;
- historique des examens et défis chrono ;
- erreurs fréquentes (mots ou opérations les plus ratés).

Toutes les données viennent des tables enregistrées depuis le lot 2.

## Critères d'acceptation

- [ ] Endpoints `/api/ui/parent/...` testés sur des jeux de données simulés.
- [ ] Affichage correct pour un profil sans aucune séance.
- [ ] Graphiques lisibles et accessibles (couleurs + libellés).
