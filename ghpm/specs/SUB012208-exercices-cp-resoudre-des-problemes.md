---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACa1g
rev: 2026-10-11T18:51:52Z
url: https://github.com/georgesp/Educator/issues/220
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012208 — Exercices CP — Résoudre des problèmes
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#122"
order: 2060
---

## User story

En tant qu'enfant de CP, je veux m'entraîner à résoudre des petits problèmes lus à voix haute afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CP : au moins dix problèmes par semaine, données de 0 à 100.
- Problèmes parties-tout et transformations en une étape (43 cerises moins 18 ; 36 oiseaux dont 21 restent) ; additifs en deux étapes (nombres ≤ 30 : bus à 29 enfants, 12 descendent, 7 montent) ; multiplicatifs en une étape (≤ 30) ; partages équitables.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cp-problems-one-step` | 2 | Problèmes en une étape | Gabarits parties-tout et transformation (gain, perte), nombres ≤ 50 puis ≤ 100 | `problem` |
| `cp-problems-two-steps` | 5 | Problèmes en deux étapes | Gabarits additifs en deux étapes, nombres ≤ 30 | `problem` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Dans le bus, il y a 29 enfants. 12 descendent, 7 montent. Combien d'enfants dans le bus ? → 24.
- Léa a 43 cerises, elle en mange 18. Combien en reste-t-il ? → 25.

## Évolutions du moteur et de l'écran de série

- Format `problem` (lot 1, CE1) adapté au CP : énoncé **lu** systématiquement, phrases courtes, image de la situation, réponse au pavé numérique.

## Distracteurs (QCM)

Mauvaise opération, une seule étape faite (17 ou 41), nombre de l'énoncé recopié.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CP — Résoudre des problèmes » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Remarques et décisions

- Au CP, l'enfant ne lit pas toujours seul : l'énoncé est lu automatiquement et peut être réécouté ; les mots écrits restent courts.
