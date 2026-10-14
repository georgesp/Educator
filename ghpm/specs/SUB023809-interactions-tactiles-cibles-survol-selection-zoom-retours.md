---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsD5E
url: https://github.com/georgesp/Educator/issues/248
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023809 — Interactions tactiles (cibles, survol, sélection, zoom, retours)"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#238"
order: 3090
---

## User story

En tant qu'enfant, je veux que chaque bouton réponde du premier coup au doigt, sans effet bizarre (zoom, sélection de texte, menu surprise).

## Dépend de

Fiche 02 ; peut avancer en parallèle de la fiche 08.

## Comportement attendu

- Cibles tactiles : au moins 44 × 44 pt pour tout ce qui se touche ; revoir les `min-width` (48 px, 88 px…) et l'espacement entre boutons de réponse (QCM, pavé numérique).
- États `:hover` (boutons, tuiles de choix, cartes de profil, cibles à cocher) : l'état « pressé » (`:active`) doit être visible au toucher et le survol ne doit pas rester « collé » après un tap (`@media (hover: hover)`).
- Désactiver le zoom par pincement et le double-tap, la sélection de texte et le menu contextuel sur les éléments de l'interface (pas sur les champs de saisie) ; pas de rebond de défilement de la page.
- Glisser-déposer et tout composant à la souris (animations de leçon interactives, `lessonDemos`) : utilisables au doigt (Pointer Events).
- Retour sonore / visuel inchangé ; vibration légère optionnelle : à ne pas ajouter sans validation.

## Critères d'acceptation

- [ ] Parcours d'une série, d'une leçon avec animation interactive et d'un examen faits au doigt, sans geste parasite.
- [ ] Aucune cible tactile de moins de 44 pt (relevé par test ou revue des composants enfant).
- [ ] Tests Vitest ; le comportement à la souris sur le bureau est inchangé.
