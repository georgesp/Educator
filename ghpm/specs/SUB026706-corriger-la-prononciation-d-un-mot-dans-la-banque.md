---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAzCY4
rev: 2026-10-14T19:31:03Z
url: https://github.com/georgesp/Educator/issues/273
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB026706 — Article, prononciation et homophones dans la banque de mots"
status: Todo
size: S
estimate:
startDate:
endDate:
parent: "#267"
order: 570
---

## User story

En tant que concepteur du contenu, je veux corriger un mot mal prononcé par la voix sans enregistrer de fichier.

## Comportement attendu

- Champ facultatif `pronunciation` dans le format des mots (`content/words/*.json`, `words.schema.json`) : le texte donné à Piper à la place du mot (« fils » → « fisse », « os » au pluriel → « ô »). Le mot affiché et attendu ne change pas.
- Le champ `audio` du format (nom d'un fichier pré-généré) est retiré.
- **Article des noms** (décision du 2026-10-14) : champ `article` (« un » ou « une », jamais « le » ni « l' » pour éviter l'élision), obligatoire pour la catégorie `nom`. Le nom est lu avec son article (« un stylo. ») ; l'enfant n'écrit que le nom, l'article est affiché devant la case (« un [______] »). Les 642 noms de la banque sont complétés par Claude, puis relus par Georges.
- **Autres mots** : lus suivis d'un point (« très. »), texte construit par le Rust à partir du mot.
- **Homophones** : la phrase d'exemple (`example`) devient obligatoire pour un mot qui a un homophone dans la banque ou dans une liste d'homophones courants (et / est, a / à, ou / où, on / ont, prêt / près, ver / verre / vert…) ; le chargement refuse un homophone sans phrase.
- **Refaire une lecture** : Piper tirant du hasard à chaque lecture, le script d'écoute permet de refaire la lecture d'un mot (`--again stylo`), qui remplace celle du cache.
- Script d'écoute de contrôle `scripts/listen-words.sh` / `.ps1` : génère dans un dossier temporaire (jamais embarqué) les WAV d'un échantillon de mots (`--sample 50`) ou d'une compétence (`--skill ce1-son-o-au-eau`), avec la liste à écouter.
- Reprend l'écoute de contrôle des 50 mots de SUB001404.

## Critères d'acceptation

- [ ] Un mot avec `pronunciation` est lu avec ce texte ; le mot attendu reste le mot (test).
- [ ] Schéma documenté ; un `pronunciation` vide est refusé.
- [ ] `article` obligatoire pour un nom (test : un nom sans article est refusé) ; l'article est affiché devant la case de la dictée.
- [ ] Un homophone sans phrase est refusé (test).
- [ ] Écoute de contrôle de 50 mots faite par Georges ; les corrections trouvées sont ajoutées à la banque.
