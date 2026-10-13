---
type: Issue
title: Corriger la prononciation d'un mot dans la banque
status: Todo
size: S
parent: specs/lot-2bis-voix-en-direct.md
order: 506
---

## User story

En tant que concepteur du contenu, je veux corriger un mot mal prononcé par la voix sans enregistrer de fichier.

## Comportement attendu

- Champ facultatif `pronunciation` dans le format des mots (`content/words/*.json`, `words.schema.json`) : le texte donné à Piper à la place du mot (« fils » → « fisse », « os » au pluriel → « ô »). Le mot affiché et attendu ne change pas.
- Le champ `audio` du format (nom d'un fichier pré-généré) est retiré.
- Script d'écoute de contrôle `scripts/listen-words.sh` / `.ps1` : génère dans un dossier temporaire (jamais embarqué) les WAV d'un échantillon de mots (`--sample 50`) ou d'une compétence (`--skill ce1-son-o-au-eau`), avec la liste à écouter.
- Reprend l'écoute de contrôle des 50 mots de SUB001404.

## Critères d'acceptation

- [ ] Un mot avec `pronunciation` est lu avec ce texte ; le mot attendu reste le mot (test).
- [ ] Schéma documenté ; un `pronunciation` vide est refusé.
- [ ] Écoute de contrôle de 50 mots faite par Georges ; les corrections trouvées sont ajoutées à la banque.
