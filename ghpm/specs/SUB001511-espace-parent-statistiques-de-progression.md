---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WwM
rev: 2026-10-14T19:30:46Z
url: https://github.com/georgesp/Educator/issues/44
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001511 — Espace parent — statistiques de progression
status: Done
size: L
estimate:
startDate:
endDate:
parent: "#15"
order: 621
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

- [x] Commandes Tauri de l'espace parent testées sur des jeux de données simulés.
- [x] Affichage correct pour un profil sans aucune séance.
- [x] Graphiques lisibles et accessibles (couleurs + libellés).

## Réalisation

- Calcul dans `educator-core::statistics` (code pur, testé sur des séances simulées), lu par la commande `get_profile_statistics`.
- Un onglet par enfant : temps cette semaine et au total, points, réussite globale, jours d'activité sur 14 jours, **3 exercices à revoir en priorité**, temps par jour (14 jours), réussite par semaine (8 semaines).
- **Tableau des exercices** : une ligne par exercice avec le nombre de fois fait (et les séries non terminées), les points gagnés (total et moyenne), une **mini-courbe des points** des 20 dernières séries avec la tendance des points (en progrès / stable / en baisse : écart d'au moins 10 % entre les 5 dernières séries et les 5 précédentes), la réussite sur les 20 dernières réponses et l'état maîtrisé (≥ 80 %) / en cours / à revoir (< 50 %) / trop tôt (< 5 réponses). Un clic sur la courbe ouvre le détail série par série. Tri : récents, moins réussis, plus pratiqués, plus de points.
- Historique des bilans et défis chrono, erreurs fréquentes (question, bonne réponse, réponse donnée le plus souvent).
- Les points d'un bilan ou d'un défi chrono, qui mélangent les exercices, ne sont pas répartis par exercice ; leurs réponses comptent dans la réussite de chaque exercice.
- Le temps compté est celui des séances enregistrées (exercices, bilans, défis, quiz de leçon), plafonné à 30 min par séance ; la lecture d'une leçon sans quiz n'est pas enregistrée.
