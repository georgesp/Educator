---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsEfg
url: https://github.com/georgesp/Educator/issues/257
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023904 — Contrat de reconnaissance (trait Rust, commande Tauri, types générés)"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#239"
order: 4040
---

## User story

En tant que développeur, je veux un seul point d'entrée « tracé → candidats de texte », identique quelle que soit la plateforme et la voie de reconnaissance retenues.

## Dépend de

Fiche 01 (voie retenue) et fiche 02 (modèle de tracé). À supprimer si seule la voie 1 est retenue.

## Comportement attendu

- Trait synchrone `HandwritingRecognizer` dans `educator-core` (comme `SpeechSynthesizer`) : `status()`, `recognize(strokes, language) -> Vec<Candidate>` (texte, confiance, classés). Une implémentation « indisponible » par défaut, une implémentation de test.
- Commande Tauri `recognize_handwriting` (contrats dans `contracts.rs`, types TypeScript générés par `npm run api:generate`), exécutée sur un thread de travail ; erreurs en `CommandError` (503 si la reconnaissance est indisponible).
- Contexte transmis pour améliorer la reconnaissance : langue (français), longueur attendue, liste fermée possible (mots de la banque) — attention : la dictée ne doit jamais révéler le mot attendu au front.
- Normalisation des candidats avant validation (espaces, casse tolérée, accents conservés) : dans le Rust, avec les règles de SUB001405.

## Critères d'acceptation

- [ ] Tests d'intégration du trait avec un faux moteur ; test d'architecture inchangé (`educator-core` sans nouvelle dépendance d'infrastructure).
- [ ] Contrats générés à jour (`bindings::types_ts_is_up_to_date` vert).
- [ ] Sur le bureau, le synthétiseur « indisponible » est utilisé sans erreur.
