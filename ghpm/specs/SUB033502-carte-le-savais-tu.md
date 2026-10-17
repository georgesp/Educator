---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdqMA
url: https://github.com/georgesp/Educator/issues/337
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB033502 — Carte « Le savais-tu ? »
status: Backlog
size: S
estimate:
startDate:
endDate:
parent: "#335"
order: 652
---

## User story

En tant qu'enfant, je veux découvrir une petite histoire vraie sur ce que j'apprends, puis revenir à ma leçon d'un clic.

## Comportement attendu

- Composant `DidYouKnowCard` dans la charte du thème actif (Grand Prix par défaut, SUB000105 et SUB001513) : bandeau « Le savais-tu ? », titre, texte, petite icône (ampoule) ; respecte la taille du texte et la police dyslexie des réglages (SUB001512).
- Bouton **Écouter** : lit `speech` à la vitesse des consignes (0,9, ajustée par le réglage de vitesse de la voix). Aucune lecture automatique. Voix système sur la tablette.
- Bouton **Continuer** (et touche Entrée / Échap) pour fermer. Pas de compte à rebours, pas de fermeture automatique.
- La mascotte (si elle n'est pas cachée) présente la carte d'une courte phrase affichée, sans voix.
- Signale au cœur que l'anecdote a été **vue** (ouverte au moins 2 secondes), pour le carnet.

## Critères d'acceptation

- [ ] Rendu vérifié sur bureau (macOS) et iPad paysage, en taille de texte « très grande ».
- [ ] Test Playwright : la carte s'ouvre, la voix ne part qu'au clic, Continuer la ferme.
