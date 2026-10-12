---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_9gVw
rev: 2026-10-11T15:22:37Z
url: https://github.com/georgesp/Educator/issues/71
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001513 — Personnalisation du thème
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#15"
order: 640
---

## User story

En tant qu'enfant, je veux choisir l'ambiance de mon application (voitures de sport, espace…) afin qu'elle me ressemble.

## Contexte

Le thème « voiture de sport » (direction « S1 · Grand Prix », voir SUB000105) est le thème par défaut depuis le lot 1.
Ses couleurs et polices sont des variables CSS regroupées dans un bloc `[data-theme="sport"]` de `web/src/styles.css` :
un nouveau thème redéfinit les mêmes variables dans son propre bloc.

## Comportement attendu

- Dans **Options** (SUB001512), une rubrique **Thème** affiche chaque thème sous forme de vignette (aperçu du menu).
- Le choix est **propre à chaque profil** et s'applique immédiatement, sans recharger l'application.
- Au moins un **deuxième thème** livré avec la fiche (piste : « Cosmos » clair, à partir des maquettes du canevas).
- Les motifs propres à un thème (damier, piste et voiture de la progression) sont fournis par le thème, pas codés en dur dans les écrans.

## Critères d'acceptation

- [ ] Thème choisi stocké dans le profil et restauré à l'ouverture du profil.
- [ ] Changement de thème visible tout de suite sur tous les écrans.
- [ ] Contraste WCAG AA vérifié pour chaque thème (test automatisé sur les paires de variables texte / fond).
- [ ] Polices de chaque thème embarquées localement, licences consignées dans `THIRD-PARTY-NOTICES.md`.

## Notes techniques

- Le front applique `data-theme` sur l'élément racine ; aucune couleur en dur dans les composants.
- Les thèmes disponibles sont du contenu (liste validée), pas des valeurs codées dans l'application.
