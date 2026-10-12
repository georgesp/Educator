---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Ru0
rev: 2026-10-11T07:56:33Z
url: https://github.com/georgesp/Educator/issues/6
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000105 — Charte graphique et composants enfant
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 60
---

## User story

En tant qu'enfant de 6 ans, je veux une interface avec de gros boutons, peu de texte et des réactions encourageantes afin de l'utiliser seul.

## Direction retenue — « S1 · Grand Prix » (thème voiture de sport)

Choisie le 2026-10-11 parmi les maquettes du canevas « Educator — Thèmes lot 1 » (https://claude.ai/artifact/6Yf1zMWsCm1ba4uJBaDXFz, écrans S1).

- Thème clair : fond blanc `#FFFFFF`, surfaces gris clair `#F3F3F5`, texte graphite `#1A1A1F`, rouge course `#D7262B` (texte rouge : `#B71C21`).
- Typographie : **Lexend** (texte, conçue pour la lisibilité) et **Barlow Condensed** italique (titres, boutons principaux), embarquées via Fontsource, sans chargement réseau.
- Motifs : bandeau damier en haut d'écran, boutons en parallélogramme, progression des séries sous forme de piste avec une voiture qui avance jusqu'au drapeau à damier.
- Couleurs et polices passent par des variables CSS regroupées dans un bloc `[data-theme="sport"]` (thème par défaut), pour la personnalisation du thème prévue au lot 3.
- Déjà en place : variables, polices et style des écrans du lot 0 (`web/src/styles.css`).

## Contenu

- Thème : palette (alignée sur celle des personnages), typographie très lisible, tailles minimales (boutons ≥ 64 px de haut, texte ≥ 20 px).
- Composants React maison :
  - `BigButton` (icône + libellé, états survol / appui / désactivé) ;
  - `NumberPad` (pavé numérique à l'écran 0–9, effacer, valider) ;
  - `ChoiceGrid` (2 à 4 réponses en grosses tuiles) ;
  - `SpeakButton` (lit un texte via Piper) ;
  - `Feedback` (animation + son : bonne réponse / à retenir), sans élément punitif ;
  - `PointsBadge`.
- Animations avec Framer Motion, discrètes et courtes (< 600 ms).

## Critères d'acceptation

- [x] Page de démonstration des composants (route de dev uniquement).
- [x] Contraste conforme WCAG AA sur tous les textes.
- [x] Les sons peuvent être coupés (réglage stocké localement, en attendant les Options).
- [x] Tests Vitest des composants interactifs (`NumberPad`, `ChoiceGrid`).

## Notes techniques

- Pas de Mantine : composants légers dédiés au public enfant.
- Sons courts libres de droits (licence CC0), listés avec leur source dans `content/sounds/LICENSES.md`.

## Implémentation (2026-10-11)

- Composants dans `web/src/components/child/` : `BigButton`, `NumberPad`, `ChoiceGrid`, `SpeakButton`, `Feedback`, `PointsBadge`, plus `CharacterImage` et `RaceTrack` (la piste de progression).
- Page de démonstration : route `#/dev/composants`, disponible seulement avec `npm run dev`. Elle est absente du build.
- Contraste vérifié par le calcul : toutes les paires texte / fond dépassent 4,5:1 (la plus juste, le texte grisé sur gris clair, est à 4,76:1).
- Sons : ils sont **synthétisés** avec la Web Audio API (deux notes montantes pour une bonne réponse, deux notes douces pour une erreur, un arpège en fin de série). Il n'y a donc aucun fichier audio à embarquer ni de licence à suivre, et `content/sounds/LICENSES.md` n'est pas nécessaire. Un bouton haut-parleur en haut à droite coupe les sons ; le réglage est gardé dans `localStorage` en attendant les Options.
- Animations Motion de moins de 600 ms. Elles sont désactivées si le système demande de réduire les animations.
- Tests : `NumberPad.test.tsx`, `ChoiceGrid.test.tsx`.
