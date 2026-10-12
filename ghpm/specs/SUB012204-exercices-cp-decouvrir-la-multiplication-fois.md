---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACawY
rev: 2026-10-11T18:51:46Z
url: https://github.com/georgesp/Educator/issues/216
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012204 — Exercices CP — Découvrir la multiplication : « fois »"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2020
---

## User story

En tant qu'enfant de CP, je veux m'entraîner à comprendre « 3 fois 20 » comme une addition répétée afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CP : sens de la multiplication — « fois » comme addition itérée (ex. trois fois vingt = 20 + 20 + 20) ; problèmes multiplicatifs en une étape, nombres ≤ 30 (3 paquets de 7 biscuits).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cp-times-as-repeated-add` | 4 | « Fois » : additions répétées | « a fois b » avec a de 2 à 5 et b ≤ 10, ou b ∈ {10, 20} ; écrit en mots et en addition répétée, résultat ≤ 60 | `phrase`, `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 3 fois 20 = ? → 60.
- 4 fois 5 = 5 + 5 + 5 + 5 = ? → 20.
- 2 paquets de 7 biscuits : combien de biscuits ? → 14.

## Évolutions du moteur et de l'écran de série

- Format `phrase` : « 3 fois 20 » affiché en mots et en addition répétée, sans le signe × (introduit plus tard).
- Image de paquets d'objets dans l'exercice (réutilise `groupObjects`).

## Distracteurs (QCM)

Addition des deux nombres (3 fois 20 → 23), une répétition en trop ou en moins.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CP — Découvrir la multiplication : « fois » » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
