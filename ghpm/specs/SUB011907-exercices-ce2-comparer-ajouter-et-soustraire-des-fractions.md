---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZAo
url: https://github.com/georgesp/Educator/issues/129
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011907 — Exercices CE2 — Comparer, ajouter et soustraire des fractions"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1590
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à comparer, ajouter et soustraire des fractions simples afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2 : comparer des fractions de même dénominateur, de même numérateur, ou dont un dénominateur est multiple de l'autre (7/12 et 5/6).
- Additionner et soustraire des fractions de même dénominateur ou de dénominateurs multiples (gâteau entamé de 1/10, 3/10 et 2/10).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-frac-compare` | 4 | Comparer des fractions | Paires de même dénominateur, de même numérateur, ou dénominateurs multiples (≤ 12) → `<`, `>`, `=` | `compare` |
| `ce2-frac-add-sub` | 5 | Ajouter et soustraire des fractions | a/b ± c/b, ou dénominateurs multiples (1/2 + 1/4), résultat ≤ 1 et ≥ 0 | `result` (fraction) |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 3/8 … 5/8 → <.
- 2/3 … 2/5 → >.
- 7/12 … 5/6 → <.
- 1/10 + 3/10 + 2/10 = ? → 6/10.
- 1 − 3/4 = ? → 1/4.

## Évolutions du moteur et de l'écran de série

- Format `compare` étendu aux fractions ; réponse sous forme de fraction : toute fraction égale est acceptée (6/10 ou 3/5), le retour affiche la forme attendue.

## Distracteurs (QCM)

Ajouter les dénominateurs (1/10 + 3/10 = 4/20), comparer seulement les dénominateurs (2/5 > 2/3).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Comparer, ajouter et soustraire des fractions » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
