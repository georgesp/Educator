---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhA4_Lo
url: https://github.com/georgesp/Educator/issues/284
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB027608 — Moteur de fusion dans educator-core (union des événements, conflits, recalcul)"
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#276"
order: 5030
---

## User story

En tant que parent, je veux que la fusion des données de la tablette et de l'ordinateur ne perde et ne double jamais rien, même si on synchronise plusieurs fois ou après un travail hors ligne des deux côtés.

## Travail

- Module `educator-core::sync` : **code pur**, sans réseau ni SQL, qui prend l'état local et un lot de changements reçus et produit les écritures à appliquer.
- Règles de l'ADR 0006 : union des séances et réponses par identifiant ; profils fusionnés champ par champ ; pierres tombales ; progression de leçon ; compétences inconnues conservées mais ignorées des statistiques.
- **Recalcul après fusion** de `SkillMastery` (20 dernières réponses par compétence) et des points, avec les fonctions existantes de `educator-core::statistics` et `sessions` (pas de seconde implémentation).
- Application des écritures dans une **seule transaction** (`educator-data`) : tout ou rien.

## Critères d'acceptation

- [ ] **Idempotence** : appliquer deux fois le même lot ne change rien.
- [ ] **Commutativité** : fusionner A puis B, ou B puis A, donne le même état (tests sur des jeux de données générés, avec `rand_chacha` à graine fixe).
- [ ] Cas testés : même séance reçue deux fois, profil renommé des deux côtés, profil supprimé d'un côté et utilisé de l'autre, deux profils de même nom, réponse avec compétence inconnue, séance non terminée, évaluation de niveau déjà faite sur l'autre appareil.
- [ ] Après fusion, les statistiques de l'espace parent sont identiques à celles qu'on obtiendrait si tout avait été fait sur un seul appareil.
- [ ] Une erreur en cours de route ne laisse pas la base à moitié fusionnée.
- [ ] `educator-core` n'ajoute aucune dépendance d'infrastructure (`tests/architecture.rs` reste vert).
