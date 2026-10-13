---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WPU
rev: 2026-10-10T19:39:07Z
url: https://github.com/georgesp/Educator/issues/20
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001404 — Audio des mots pré-généré avec Piper
status: In progress
size: S
estimate:
startDate:
endDate:
parent: "#14"
order: 370
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
- [x] Taille totale de l'audio documentée.

## Implémentation (2026-10-14)

- Script `scripts/generate-word-audio.sh` / `.ps1` (exemple Rust `generate_word_audio`) : un WAV par mot et un par
  phrase d'exemple, voix Piper retenue, débit ralenti, dans `content/audio/words/` (non commité, embarqué au build).
  Un manifeste garde le texte, la voix et le débit de chaque fichier : seuls les mots ajoutés ou modifiés sont
  régénérés. `--sample 50` écrit la liste d'écoute `ecoute.txt`.
- Lecture : le fichier pré-généré s'il existe, sinon la voix Piper en direct (repli).
- Dictée : le mot est lu seul, puis dans sa phrase quand elle existe ; boutons pour réécouter l'un ou l'autre.
- Taille mesurée le 2026-10-14 : **1 918 fichiers, 128,8 Mo** (WAV 22 050 Hz mono 16 bits, 1,5 s en moyenne
  pour un mot seul).
- Reste : écoute de contrôle des 50 mots de `ecoute.txt` et mesure du délai de lecture sur le Mac (Georges).
- Décision du 2026-10-14 : la pré-génération sera remplacée par la voix en direct (lot 2bis, « Voix en direct pour
  les mots d'orthographe ») ; l'écoute de contrôle des 50 mots y est reprise (fiche « Corriger la prononciation
  d'un mot dans la banque »).
