---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsD5E
rev: 2026-10-14T11:29:09Z
url: https://github.com/georgesp/Educator/issues/248
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023809 — Interactions tactiles (cibles, survol, sélection, zoom, retours)"
status: In Review
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

## Implémentation (2026-10-15)

- Survol : les 21 règles `:hover` de `styles.css` et `lesson.css` ne s'appliquent qu'avec une souris (`@media (hover: hover)`) ; au doigt, le même aspect pendant l'appui (`:active` sous `@media (hover: none)`).
- Écran tactile (`@media (pointer: coarse)`) : pas de sélection de texte, de menu contextuel, de surbrillance au toucher ni de rebond ; `touch-action: manipulation` (pas de zoom au double-tap) ; champs de saisie toujours sélectionnables. Zoom par pincement désactivé (`viewport`).
- Droite graduée : l'aperçu au survol ne suit que la souris (`NumberLine.tsx`). Glisser-déposer des lettres : Framer Motion (Pointer Events), utilisable au doigt.
- Cibles : boutons de 48 px et plus (pavé 64 px, lettres 52 px) ; seule la réduction des touches de lettres à 40 px vaut sous 760 px de large (jamais sur l'iPad, 1 366 px).
- Pas de vibration ajoutée.

## Corrections après relecture (2026-10-15)

- `main.tsx` : écouteur `touchstart` vide, sans lequel Safari n'applique pas `:active` (retour visuel à l'appui).
- Audit des cibles de `lesson.css` : toutes ≥ 44 points sauf la bande « couper ici » de la grille de voitures (`ArrayGrid.tsx`), passée de 16 à 38 unités (≈ 44 points affichée).
- Import de `isTouchScreen` remis dans l'ordre dans `NumberLine.tsx`.
- À vérifier sur l'iPad : un bouton de leçon change d'aspect sous le doigt ; leçon « tableaux » (grille de voitures) : la coupure se fait facilement au doigt entre deux rangées.

## Tests à faire (revue)

1. iPad, au doigt : une série en QCM, une série au pavé numérique, une leçon avec animation interactive (droite graduée, lettres à glisser), un examen.
2. Pour chaque bouton touché : il réagit du premier coup, change d'aspect pendant l'appui et ne reste pas « en survol » après.
3. Gestes parasites : double-tap rapide sur un bouton → pas de zoom ; pincer → pas de zoom ; appui long sur un texte ou un bouton → ni sélection ni menu ; tirer la page vers le bas → pas de rebond.
4. Droite graduée : toucher une graduation ne laisse pas de bulle d'aperçu affichée.
5. Bureau, à la souris : le survol des boutons, tuiles et cartes de profil s'affiche comme avant ; `npm test` vert (`NumberLine.test.tsx` inchangé).
