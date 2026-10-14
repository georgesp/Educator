---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WvQ
rev: 2026-10-14T19:30:44Z
url: https://github.com/georgesp/Educator/issues/43
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001510 — Espace parent — accès
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#15"
order: 620
---

## User story

En tant que parent, je veux un espace réservé facile d'accès pour moi mais pas pour mon enfant.

## Comportement attendu

- Accès depuis Options → **Espace parent**.
- Barrière « adulte » : une multiplication à deux chiffres à résoudre (ou maintenir un bouton 3 secondes). Ce n'est pas un mot de passe, juste un garde-fou.
- Les réglages sensibles (suppression de profil, objectif du jour, chrono) passent dans l'espace parent.

## Critères d'acceptation

- [x] Barrière régénérée à chaque ouverture.
- [x] Retour au menu enfant en un clic.

## Réalisation

- Options → **Espace parent** → multiplication d'un nombre à deux chiffres par un nombre à un chiffre (ex. 27 × 6), tirée et vérifiée par le cœur Rust (`educator-core::parent_gate`, commandes `get_parent_gate` / `check_parent_gate`) ; une mauvaise réponse en tire une autre.
- L'accès se referme au retour au menu enfant ou en revenant sur la barrière ; il n'est jamais mémorisé d'un lancement à l'autre.
- **Carte des parents** sur l'écran « Qui joue ? », après les enfants : nom « Parents » et l'ours par défaut, créée au premier lancement et modifiable dans l'espace parent (nom, personnage). Elle mène à la barrière, puis à l'espace parent ; on revient ensuite au choix du profil. Ce n'est pas un profil enfant (ni classe, ni points, ni statistiques) : elle est stockée dans `AppSetting`.
- La suppression de profil est passée d'Options à l'espace parent. L'objectif du jour et le chrono y viendront avec SUB001507 et SUB001512.
