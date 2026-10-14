---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsEbo
url: https://github.com/georgesp/Educator/issues/256
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB023903 — Écriture manuscrite du système dans les champs de réponse
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#239"
order: 4030
---

## User story

En tant qu'enfant, je veux écrire avec mon stylet directement dans la case de réponse, et voir mon écriture se transformer en lettres.

## Dépend de

Fiche 01 (voie 1 retenue ou gardée comme repli) ; fiche 10 de ce lot.

## Comportement attendu

- Sur iPad (Scribble) et sur Android récent (écriture manuscrite du système), le champ de réponse d'orthographe accepte l'écriture au stylet, sans code de reconnaissance de notre côté.
- Les champs sont ceux de la fiche 10 du lot tablette 1 : sans correction automatique, sans suggestions, sans majuscule automatique ; la valeur reçue passe par la validation de SUB001405.
- Détection et aide : si la fonction du système est désactivée ou indisponible, message simple pour le parent dans les options (« Active l'écriture manuscrite dans les réglages de la tablette ») et repli au clavier.
- La confirmation de la fiche 07 s'applique aussi ici : le texte reconnu est affiché dans la case avant validation.

## Critères d'acceptation

- [ ] Sur la tablette de test : un mot avec accent écrit au stylet apparaît dans la case et peut être validé.
- [ ] Sans la fonction du système : le clavier fonctionne et le message d'aide s'affiche.
- [ ] Limites constatées (écriture d'enfant, accents, versions) notées dans l'ADR 0006.
