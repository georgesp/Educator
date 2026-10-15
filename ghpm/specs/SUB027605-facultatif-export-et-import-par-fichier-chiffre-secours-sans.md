---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhA4_DQ
url: https://github.com/georgesp/Educator/issues/281
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB027605 — Facultatif — export et import par fichier chiffré (secours sans réseau)
status: Todo
size: S
estimate:
startDate:
endDate:
parent: "#276"
order: 5090
---

## User story

En tant que parent, je veux pouvoir transférer les données à la main (AirDrop, câble, clé) quand le Wi-Fi ne fonctionne pas ou pour faire une sauvegarde.

## Travail

- Export d'un fichier unique au **même format d'échange** que la synchronisation, chiffré par un mot de passe choisi par le parent ; import avec la **même fusion** (idempotente), jamais un écrasement.
- Aucun mot de passe enregistré ; fichier sans nom d'enfant dans son nom.
- À ne faire que si le reste du lot est stable : fiche facultative, Georges a retenu la synchronisation Wi-Fi comme solution principale.

## Critères d'acceptation

- [ ] Export sur un appareil, import sur l'autre : mêmes statistiques des deux côtés.
- [ ] Un mauvais mot de passe ou un fichier altéré est refusé sans modifier la base.
- [ ] Importer deux fois le même fichier ne change rien.
