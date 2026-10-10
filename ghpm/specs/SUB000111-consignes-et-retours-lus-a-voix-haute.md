---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5R4A
url: https://github.com/georgesp/Educator/issues/12
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000111 — Consignes et retours lus à voix haute
status: Todo
size: S
estimate:
startDate:
endDate:
parent: "#1"
order: 120
---

## User story

En tant qu'enfant de CP qui ne lit pas encore bien, je veux entendre les consignes et les corrections afin de pouvoir utiliser l'application seul.

## Comportement attendu

- Le front demande la lecture d'un texte au host : `POST /api/ui/speech` `{ text }` → audio WAV (ou URL d'un fichier en cache).
- Textes lus au lot 1 : consignes des écrans (choix du profil, menu, choix du mode), consigne de l'exercice, calcul (« sept plus cinq »), correction (« sept plus cinq égale douze »), encouragements.
- Les nombres et symboles sont convertis en texte français avant la synthèse (`+` → « plus », `-` → « moins », `?` → « combien »).
- Cache disque des fichiers audio (clé : hash du texte + voix + vitesse) dans le dossier de données de l'application.
- Une seule lecture à la fois : une nouvelle lecture interrompt la précédente.

## Critères d'acceptation

- [ ] Temps entre le clic et le début du son < 300 ms pour un texte en cache, < 1,5 s sinon.
- [ ] Tests unitaires de la conversion texte (nombres jusqu'à 1 000, symboles).
- [ ] Fonctionne hors ligne sur macOS et Windows.

## Notes techniques

- Repose sur l'intégration Piper du lot 0 (binaire + voix française embarqués).
- Vitesse de lecture légèrement ralentie par défaut ; réglage exposé plus tard dans les Options.
