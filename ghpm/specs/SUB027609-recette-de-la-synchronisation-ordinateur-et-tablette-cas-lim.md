---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhA4_OI
rev: 2026-10-15T05:31:49Z
url: https://github.com/georgesp/Educator/issues/285
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB027609 — Recette de la synchronisation (ordinateur et tablette, cas limites)"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#276"
order: 5100
---

## User story

En tant que Georges, je veux vérifier sur mes appareils réels que la synchronisation tient dans les situations de la vie quotidienne avant de l'utiliser avec les enfants.

## Scénarios

- Premier appairage avec des profils déjà existants des deux côtés (même prénom, prénoms différents).
- Travail hors ligne le même jour sur l'ordinateur et la tablette, puis synchronisation.
- Wi-Fi coupé en plein échange, application tuée en plein échange, ordinateur mis en veille.
- Profil renommé des deux côtés ; profil supprimé d'un côté puis utilisé de l'autre.
- Heure de la tablette décalée de plusieurs minutes par rapport à l'ordinateur.
- Mise à jour d'un seul des deux appareils (version de protocole ou de contenu différente).
- Tablette retirée de l'espace parent puis tentative de synchronisation.
- Volume : plusieurs mois d'historique (plusieurs milliers de réponses) ; durée et consommation de la première synchronisation.

## Livrable

- Rapport de recette dans `docs/` (scénario, résultat, anomalie éventuelle) et fiches d'anomalies créées dans le backlog.

## Critères d'acceptation

- [ ] Tous les scénarios sont joués sur l'ordinateur et la tablette de test, avec le résultat consigné.
- [ ] Aucune perte ni doublon constaté ; les anomalies restantes sont des fiches ouvertes.
- [ ] Les commandes de build et de test à lancer par Georges sont indiquées dans le rapport.

## Réalisation (2026-10-15)

Le rapport à remplir est prêt : `docs/recette-synchronisation.md` (20 scénarios, commandes, mesures).
