---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdqKE
url: https://github.com/georgesp/Educator/issues/336
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB033501 — Format des anecdotes
status: Backlog
size: S
estimate:
startDate:
endDate:
parent: "#335"
order: 651
---

## User story

En tant que concepteur du contenu, je veux décrire les anecdotes dans des fichiers afin d'en ajouter sans toucher au code.

## Format

Fichiers `content/anecdotes/{subject}/{grade}.json` (ex. `content/anecdotes/math/cp.json`), schéma `content/schemas/anecdote.schema.json`. Chaque anecdote :

- `id` : identifiant stable (`^[a-z0-9]+(-[a-z0-9]+)*$`), unique dans tout le corpus.
- `subject` (`math` | `spelling`), `grade` (`CP` … `CM2`), `period` facultative (première période où elle peut apparaître).
- `skillIds` : compétences liées (au moins une, existantes, même matière ; classes égales ou inférieures autorisées pour réutiliser une anecdote).
- `title` : 40 caractères au plus (« Le zéro a mis longtemps à arriver »).
- `text` : affiché, 2 à 3 phrases, 280 caractères au plus.
- `speech` : texte lu (facultatif, `text` par défaut), soumis à la même normalisation que les textes lus des leçons (nombres en lettres, longueur maximale).
- `sources[]` : au moins 2 entrées `{ title, url, kind }` avec `kind` ∈ `institution` | `academic` | `reference` | `children` ; au moins une `institution` ou `academic`. URL en `https://`.
- `checkedOn` : date de la dernière vérification (`YYYY-MM-DD`).

## Chargement

- Chargeur `educator-content::anecdotes`, sur le modèle des leçons : schéma validé au build, puis règles qu'un schéma ne sait pas dire (compétence inconnue, identifiant en double, `text` trop long une fois lu, règle des sources). Une anecdote invalide est signalée et ignorée, sans bloquer l'application.
- Commande Tauri `anecdote_for(skill_ids, profile_id)` : renvoie une anecdote liée, en priorité une que le profil n'a pas encore vue, sinon `null` si toutes ont été vues récemment (voir la fiche du carnet).

## Critères d'acceptation

- [ ] Schéma documenté et validé au build ; deux anecdotes d'exemple (une de maths, une de français).
- [ ] Tests du chargeur : compétence inconnue, identifiant en double, une seule source, aucune source institutionnelle ou académique, texte trop long.
- [ ] Contrats générés en TypeScript (`ts-rs`).
