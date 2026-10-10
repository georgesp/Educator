---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Ru0
url: https://github.com/georgesp/Educator/issues/6
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000105 — Charte graphique et composants enfant
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 60
---

## User story

En tant qu'enfant de 6 ans, je veux une interface avec de gros boutons, peu de texte et des réactions encourageantes afin de l'utiliser seul.

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

- [ ] Page de démonstration des composants (route de dev uniquement).
- [ ] Contraste conforme WCAG AA sur tous les textes.
- [ ] Les sons peuvent être coupés (réglage stocké localement, en attendant les Options).
- [ ] Tests Vitest des composants interactifs (`NumberPad`, `ChoiceGrid`).

## Notes techniques

- Pas de Mantine : composants légers dédiés au public enfant.
- Sons courts libres de droits (licence CC0), listés avec leur source dans `content/sounds/LICENSES.md`.
