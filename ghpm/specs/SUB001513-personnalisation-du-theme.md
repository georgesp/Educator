---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_9gVw
rev: 2026-10-16T19:31:00Z
url: https://github.com/georgesp/Educator/issues/71
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001513 — Personnalisation du thème
status: Done
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

- [x] Thème choisi stocké dans le profil et restauré à l'ouverture du profil.
- [x] Changement de thème visible tout de suite sur tous les écrans.
- [x] Contraste WCAG AA vérifié pour chaque thème (test automatisé sur les paires de variables texte / fond).
- [x] Polices de chaque thème embarquées localement, licences consignées dans `THIRD-PARTY-NOTICES.md`.

## Notes techniques

- Le front applique `data-theme` sur l'élément racine ; aucune couleur en dur dans les composants.
- Les thèmes disponibles sont du contenu (liste validée), pas des valeurs codées dans l'application.

## Réalisation

- Liste des thèmes en contenu : `content/themes/themes.json` (schéma `themes.schema.json`, `sport` obligatoire) ; un thème inconnu est refusé par le cœur.
- Deuxième thème **Cosmos** : ciel clair lavande, violet, bandeau étoilé, fusée sur la piste de progression, boutons arrondis, police Fredoka (titres droits, sans majuscules forcées). Bloc `[data-theme="cosmos"]` de `web/src/styles.css`.
- Les formes et motifs propres à un thème sont des variables : `--strip` (bandeau du haut), `--finish` (arrivée de la piste), `--track*`, `--button-shape`, `--tag-shape`, `--display-style`, `--display-case`… ; le véhicule de la piste est fourni par le thème (`.theme-vehicle[data-for]`). Les couleurs en dur de la piste ont été remplacées par des variables.
- Options → **Le thème** : une vignette par thème (menu miniature dessiné avec les variables du thème), choix enregistré dans les réglages de l'enfant et appliqué tout de suite (`data-theme` sur la racine) ; restauré à l'ouverture du profil (avant le premier choix d'un profil, l'application est en Grand Prix).
- Test `web/src/themes.test.ts` : chaque thème du contenu a son bloc, définit toutes les variables du thème sport, et atteint le contraste AA (4,5:1) sur 15 paires texte / fond.
- Police Fredoka (SIL OFL 1.1) embarquée via `@fontsource/fredoka`, consignée dans `THIRD-PARTY-NOTICES.md`.
