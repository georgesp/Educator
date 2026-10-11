---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAvtg
url: https://github.com/georgesp/Educator/issues/75
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000114 — Feu d'artifice quand toutes les réponses sont justes
status: Todo
size: S
estimate:
startDate:
endDate:
parent: "#1"
order: 122
---

Source : retours de Georges après les premiers essais du lot 1 (2026-10-11).

## User story

En tant qu'enfant, je veux une belle récompense visuelle quand j'ai tout juste afin d'être fier de ma série.

## Comportement attendu

- Quand une série d'exercices se termine **sans aucune erreur** (10/10), un feu d'artifice s'affiche sur l'écran de récapitulatif.
- Le même composant sera utilisé pour un **examen** réussi sans faute, quand le mode examen arrivera (SUB001414, lot 2).
- Animation courte (2 à 3 secondes), aux couleurs du thème, qui ne bloque pas les boutons du récapitulatif (l'enfant peut continuer pendant l'animation).
- Un court son de fête accompagne l'animation ; il respecte le futur réglage « couper les sons ».
- Si le système demande de réduire les animations (`prefers-reduced-motion`), on affiche une version fixe (étoiles et message) à la place.
- Message lu par Piper, par exemple « Bravo, tout est juste ! ».

## Critères d'acceptation

- [ ] Feu d'artifice affiché après une série 10/10, et seulement dans ce cas.
- [ ] Composant réutilisable (`Fireworks` dans `web/src/components/child/`), mentionné dans la fiche du mode examen.
- [ ] Version sans mouvement quand `prefers-reduced-motion` est actif.
- [ ] Test Vitest (affiché à 10/10, absent à 9/10) et parcours Playwright mis à jour.

## Notes techniques

- Canvas ou Framer Motion, sans dépendance réseau ; pas de bibliothèque lourde.
