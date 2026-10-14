---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsC0I
rev: 2026-10-14T11:25:41Z
url: https://github.com/georgesp/Educator/issues/237
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: US0237 — Lot 2bis — Voix en direct pour les mots d'orthographe
status: Todo
size:
estimate:
startDate:
endDate:
parent:
order: 4130
---

Source : comparaison « fichiers audio / voix en direct » du 2026-10-14, décision de Georges : passer les mots d'orthographe en voix en direct.

- **Lot** : 2bis — Voix en direct
- **Prérequis** : fiche « Lot 2 — Orthographe, leçons et examens » (SUB001404 : audio des mots pré-généré).
- **Lot suivant** : fiche « Lot 3 — Motivation et espace parent ».

## Contexte

Au lot 2, chaque mot de la banque et sa phrase d'exemple sont pré-générés en WAV avec Piper (SUB001404). Mesures du 2026-10-14 (voix `fr_FR-siwis-medium`) :

| | Fichiers WAV | Voix en direct |
|---|---|---|
| Espace disque CP–CE1 (959 mots + phrases) | 129 Mo | 0 Mo de plus (moteur et voix déjà embarqués pour les consignes) |
| Espace disque projeté CP → CM2 (≈ 2 500 mots, ≈ 1 000 phrases) | ≈ 420 Mo, ≈ 600 Mo avec une version à 0,5 | 0 Mo de plus, cache plafonné |
| Latence | ≈ 10 à 30 ms | mot ≈ 70 à 100 ms, phrase ≈ 130 à 260 ms (Linux x86) ; ≈ 10 à 30 ms une fois préparé à l'avance |
| Ralentir à 0,5 | 2e jeu de fichiers, ou déformation du son | natif (`--length_scale`), sans perte de qualité |
| Simplicité | script de 20 min à relancer, deux chemins (fichiers + secours en direct) | un seul chemin, déjà utilisé pour les consignes |

Contraintes vérifiées :
- Piper fixe la vitesse **pour tout le processus** (`--length_scale`) : un champ de vitesse par demande JSON est ignoré (testé). Changer de vitesse redémarre Piper (≈ 0,4 à 0,5 s).
- Un processus Piper chaud occupe ≈ 110 Mo de mémoire.

## Objectif

Lire les mots d'orthographe avec la voix en direct, sans fichiers pré-générés, avec un délai imperceptible pour l'enfant (< 200 ms après le clic), et proposer une écoute plus lente (vitesse 0,5).

## Hors périmètre

- La voix sur tablette (moteur intégré à l'application) : étudiée ici (dernière sous-fiche), réalisée dans le lot tablette.
- Le changement de voix.

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] Aucun fichier audio de mot n'est embarqué ; la taille de l'application ne dépend plus du nombre de mots.
- [ ] Sur le Mac de Georges (Rosetta), un mot de dictée est entendu moins de 200 ms après l'affichage ou le clic, phrase comprise une fois préparée.
- [ ] ADR 0004 « Voix en direct pour les mots » rédigé (remplace la partie pré-génération de SUB001404).
