---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsEvU
url: https://github.com/georgesp/Educator/issues/260
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB023907 — Confirmation de la lecture et retours bienveillants en cas de doute
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#239"
order: 4070
---

## User story

En tant qu'enfant, je veux vérifier ce que la tablette a compris avant qu'elle ne corrige, pour que ce ne soit jamais sa faute à elle si je suis « faux ».

## Dépend de

Fiches 02 et 04 (ou 03 seule).

## Comportement attendu

- Après « Valider mon écriture », la lecture de la tablette s'affiche en grosses lettres sous le tracé : « Tu as écrit : **élève** ? » avec trois actions : « Oui », « Réécrire », « Taper au clavier ».
- Candidats alternatifs proposés quand la confiance est faible (ex. « élève », « eleve », « élevé ») : l'enfant touche le bon ; les candidats viennent du Rust, avec les mêmes règles de normalisation.
- Seule la lecture **confirmée** est envoyée à la validation (SUB001405) ; l'accent manquant reste une vraie erreur d'orthographe, une lecture rejetée par l'enfant n'en est jamais une.
- Textes dans `fr.ts`, ton encourageant, jamais culpabilisant ; voix qui épelle ou relit la lecture, option (à trancher).
- Statistiques : le nombre de lectures corrigées par l'enfant est enregistré à part (utile pour la mesure de la fiche 12), sans impact sur les points ni sur la maîtrise des compétences.

## Critères d'acceptation

- [ ] Tests Vitest du parcours « oui », « réécrire », « clavier », « autre candidat ».
- [ ] Une lecture refusée n'est comptée comme aucune réponse (ni juste ni fausse).
- [ ] Test Rust : seule la réponse confirmée est validée et enregistrée (`Answer`).
