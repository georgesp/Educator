---
type: Issue
title: Une voix Piper prête pour chaque vitesse
status: Todo
size: M
parent: specs/lot-2bis-voix-en-direct.md
order: 502
---

## User story

En tant qu'enfant, je veux entendre la consigne puis le mot à écrire sans attendre, même si le mot est lu plus lentement.

## Comportement attendu

- `PiperSpeechSynthesizer` (`educator-content::speech`) garde **un processus Piper par vitesse** au lieu d'un seul : passer de la consigne (vitesse normale) au mot dicté (ralenti) ne redémarre plus Piper.
- Vitesses utilisées : consignes (1), mots dictés (0,75), phrases d'exemple (0,9), écoute lente (0,5). Les vitesses voisines peuvent être regroupées si l'écoute le permet (à décider avec Georges : par exemple 0,9 lu à 1).
- Au démarrage : préchauffage des processus « consignes » et « mots dictés ». Les autres sont lancés à la première demande.
- Mémoire : ≈ 110 Mo par processus. Au plus 2 processus restent chauds en permanence ; un processus inutilisé depuis 2 minutes s'arrête (sauf les deux préchauffés).
- Un processus planté ou bloqué est relancé, comme aujourd'hui ; les demandes à une même vitesse restent servies une à la fois.

## Critères d'acceptation

- [ ] Plus aucun redémarrage de Piper lors du passage consigne → mot → consigne (vérifié dans les logs).
- [ ] Mémoire totale mesurée et notée dans la fiche (macOS, Rosetta).
- [ ] Tests : plusieurs vitesses servies sans redémarrage ; arrêt après inactivité (horloge injectée) ; relance après un plantage.
