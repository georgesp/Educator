---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAzB8o
rev: 2026-10-15T09:48:07Z
url: https://github.com/georgesp/Educator/issues/267
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: US0267 — Lot 2bis — Voix en direct pour les mots d'orthographe
status: Done
size: L
estimate:
startDate:
endDate:
parent:
order: 510
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

## Décisions d'écoute (2026-10-14)

Prises par Georges après écoute d'extraits générés avec la voix `fr_FR-siwis-medium` :

- **Même vitesse pour le mot et sa phrase : 0,85.** Avec 0,75 pour le mot et 0,9 pour la phrase, le même mot ne sonnait pas pareil.
- **Les noms sont lus avec leur article** : « un stylo. », « une école. ». Seul au début d'une lecture, un nom perd souvent sa première consonne (« stylo » entendu « sylo » : le « t » est dans la phonétique, `stilˈo`, mais la voix l'avale en début de lecture). L'article règle le problème et donne le genre du nom.
- **Les autres mots sont lus dans « Le mot : jaune. », coupé à la pause du deux-points** : l'enfant n'entend que le mot (décision du 2026-10-15, qui remplace le point du 2026-10-14 puis la virgule).
- **Les homophones ont toujours leur phrase** : « prêt » et « près » lus seuls sont le même son ; seule la phrase permet de choisir.
- Piper tire du hasard à chaque lecture (`noise_scale` 0,667, `noise_w` 0,8 dans la configuration de la voix) : deux lectures du même mot diffèrent. Le cache garde la première ; une lecture ratée doit pouvoir être refaite.

## Objectif

Lire les mots d'orthographe avec la voix en direct, sans fichiers pré-générés, avec un délai imperceptible pour l'enfant (< 200 ms après le clic), et proposer une écoute plus lente (vitesse 0,5).

## Hors périmètre

- La voix sur tablette (moteur intégré à l'application) : étudiée ici (dernière sous-fiche), réalisée dans le lot tablette.
- Le changement de voix.

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [x] Aucun fichier audio de mot n'est embarqué ; la taille de l'application ne dépend plus du nombre de mots.
- [x] Sur le Mac de Georges (Rosetta), un mot de dictée est entendu moins de 200 ms après le clic, phrase comprise une fois préparée.
- [x] ADR 0004 « Voix en direct pour les mots » rédigé (remplace la partie pré-génération de SUB001404).

## Avancement (2026-10-15)

- Done : SUB026701, SUB026702, SUB026704, SUB026705, SUB026706, SUB026707, SUB026708 (essais, logs et mesures
  de Georges du 2026-10-15).
- SUB026703 (étude tablette, partie documentaire faite, ADR 0005) : déplacée dans le lot tablette (#239) pour les mesures sur iPad.
- Décision du 2026-10-15 : les consignes restent lues à 0,9.
- ADR 0004 « Voix en direct pour les mots », ADR 0005 « Voix sur tablette » (étude).
- Décisions du 2026-10-15 (essais de Georges) : plus aucune lecture automatique dans l'application, la voix ne part
  qu'au clic (« Écoute » était coupé à l'ouverture de l'exercice) ; les mots autres que les noms sont lus dans
  « Le mot : jaune. », coupé à la pause du deux-points (seuls, « beau. » était entendu « beu » et « jaune, »
  « jeuné »). Voir ADR 0004.
