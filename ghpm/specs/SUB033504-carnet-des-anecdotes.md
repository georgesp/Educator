---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdqPU
url: https://github.com/georgesp/Educator/issues/339
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB033504 — Carnet des anecdotes
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#335"
order: 654
---

## User story

En tant qu'enfant, je veux retrouver les anecdotes que j'ai déjà découvertes, pour les relire ou les raconter.

## Comportement attendu

- Table `SeenAnecdote` (profil, identifiant, date de première vue), migration SQLite versionnée ; incluse dans la sauvegarde / restauration (SUB001512) et dans la synchronisation tablette ↔ bureau (lot tablette, union des anecdotes vues).
- Écran **Mon carnet** (depuis le profil ou le menu, à côté de « Mes badges ») : anecdotes vues regroupées par matière puis par notion, avec la date ; chacune s'ouvre dans la carte « Le savais-tu ? ».
- Les anecdotes pas encore vues n'apparaissent pas (pas de cases grises à remplir) : le carnet garde la surprise et ne crée pas d'obligation de collection.
- Pas de points ni d'étoiles pour la lecture. Un éventuel badge « Curieux » (conditions `anecdotesSeen`, ex. 10 anecdotes découvertes) est à valider par Georges avant d'être ajouté à `badges.json`.
- Espace parent : nombre d'anecdotes découvertes par l'enfant (simple information).

## Critères d'acceptation

- [ ] Une anecdote vue apparaît dans le carnet du bon profil seulement.
- [ ] Aller-retour sauvegarde / restauration sans perte des anecdotes vues.
- [ ] Une anecdote retirée du corpus disparaît du carnet sans erreur.
