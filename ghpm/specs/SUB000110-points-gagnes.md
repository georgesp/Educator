---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5R2o
rev: 2026-10-10T19:29:07Z
url: https://github.com/georgesp/Educator/issues/11
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000110 — Points gagnés
status: In progress
size: S
estimate:
startDate:
endDate:
parent: "#1"
order: 110
---

## User story

En tant qu'enfant, je veux gagner des points quand je réussis afin d'avoir envie de continuer.

## Règles (lot 1)

- **+10 points** par bonne réponse.
- **+20 points** de bonus pour une série terminée, quel que soit le score (récompense l'effort).
- **+20 points** supplémentaires pour une série sans faute.
- Jamais de points retirés.
- Les points sont stockés dans le profil (`Profile.Points`) à la fin de chaque série.

## Critères d'acceptation

- [x] Barème défini dans un fichier de contenu (`content/rewards/points.json`), pas en dur dans le code.
- [x] Les points gagnés s'affichent avec une petite animation au récapitulatif de fin de série, puis dans le menu.
- [x] Une série abandonnée rapporte les points des bonnes réponses déjà données, sans bonus.
- [ ] Tests unitaires du calcul des points.

## Notes techniques

- Les niveaux, étoiles et badges basés sur ces points arrivent au lot 3.

## Implémentation (2026-10-11)

- Barème : `content/rewards/points.json`, lu par `SmallContentLoader`. Le calcul est dans `PointsRules` (Core).
- Les points sont ajoutés au profil de façon atomique, une seule fois par série.
- Le récapitulatif affiche « +N points » avec une animation. Le badge du menu compte jusqu'au nouveau total.
- Tests : `PointsRulesTests`, `SeriesServiceTests`, `SeriesEndpointsTests`.
