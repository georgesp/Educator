---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WPU
url: https://github.com/georgesp/Educator/issues/20
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001404 — Audio des mots pré-généré avec Piper
status: Todo
size: S
estimate:
startDate:
endDate:
parent: "#14"
order: 240
---

## User story

En tant qu'enfant, je veux entendre le mot à écrire immédiatement afin de ne pas attendre pendant la dictée.

## Comportement attendu

- Script de build qui génère un WAV (ou OGG) par mot de la banque avec la voix Piper retenue, à vitesse ralentie.
- Les fichiers sont embarqués avec le contenu ; régénération uniquement des mots ajoutés ou modifiés (hash).
- Lecture du mot seul, puis dans une phrase courte si elle est fournie (`example` optionnel dans le format du mot), pour lever les ambiguïtés (« vert », « verre », « ver »).

## Critères d'acceptation

- [ ] Lecture < 200 ms après le clic.
- [ ] Écoute de contrôle d'un échantillon de 50 mots (prononciation correcte).
- [ ] Taille totale de l'audio documentée.
