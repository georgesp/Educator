---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsE9I
url: https://github.com/georgesp/Educator/issues/263
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023910 — Réglage « Écrire au stylet » (par profil, détection, gaucher)"
status: Todo
size: S
estimate:
startDate:
endDate:
parent: "#239"
order: 4100
---

## User story

En tant que parent, je veux choisir si mon enfant écrit au stylet ou au clavier, et adapter la zone d'écriture à sa main.

## Comportement attendu

- Option dans Options (SUB001512) et par profil : « Écrire au stylet » (automatique si un stylet est détecté, toujours, jamais) ; « Écrire aussi au doigt » ; main d'écriture (gauche / droite) qui décale les boutons et le rejet de la paume.
- Stockage : réglage de profil dans `educator-data`, nouvelle migration (une migration publiée n'est jamais modifiée), supprimé avec le profil.
- Message d'aide quand l'écriture manuscrite du système est indisponible (fiche 03).
- Textes dans `fr.ts` ; contrats et types générés.

## Critères d'acceptation

- [ ] Réglage conservé après relance, par profil ; test d'intégration de la migration.
- [ ] Sans stylet détecté en mode automatique : aucun écran ne change.
