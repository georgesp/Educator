---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAvtg
rev: 2026-10-11T15:22:48Z
url: https://github.com/georgesp/Educator/issues/75
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000114 — Feu d'artifice quand toutes les réponses sont justes
status: Done
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

- [x] Feu d'artifice affiché après une série 10/10, et seulement dans ce cas.
- [x] Composant réutilisable (`Fireworks` dans `web/src/components/child/`), mentionné dans la fiche du mode examen.
- [x] Version sans mouvement quand `prefers-reduced-motion` est actif.
- [x] Test Vitest (affiché à 10/10, absent à 9/10) et parcours Playwright mis à jour.

## Notes techniques

- Canvas ou Framer Motion, sans dépendance réseau ; pas de bibliothèque lourde.

## Implémentation (2026-10-11)

- Composant `Fireworks` (`web/src/components/child/Fireworks.tsx`) en Framer Motion, sans canvas ni dépendance :
  - 5 gerbes d'étincelles aux couleurs du thème (variables `--primary`, `--correct`, `--firework-gold`,
    `--firework-blue`) ;
  - 2,6 s d'animation, dans un calque `pointer-events: none` : les boutons du récapitulatif restent utilisables.
- Affiché sur le récapitulatif seulement quand toutes les réponses sont justes. Le message « Bravo, tout est
  juste ! » est écrit et lu par Piper avant le récapitulatif parlé.
- Son de fête `celebration` (Web Audio, comme les autres sons) : il respecte le bouton qui coupe les sons, en
  attendant le réglage des Options.
- `prefers-reduced-motion` : trois étoiles fixes à la place des étincelles, à côté du message.
- Mentionné dans la fiche du mode examen (SUB001414).
- Tests :
  - `Fireworks.test.tsx` : animation puis disparition, version fixe ;
  - `SeriesScreen.test.tsx` : feu d'artifice à 2/2 et absent à 1/2 ;
  - Playwright : absent à 9/10, présent à 10/10, et le bouton Menu reste cliquable pendant l'animation.
