---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZNY
rev: 2026-10-11T18:49:25Z
url: https://github.com/georgesp/Educator/issues/137
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011915 — Exercices CE2 — Tables de multiplication de 2, 3, 4, 5 et 10"
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 1030
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à connaître les tables de 2, 3, 4, 5 et 10 dans les deux sens afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2 : tables de multiplication connues dans les deux sens (12 égalités en une minute en fin d'année, ex. 7 × … = 42).
- Vocabulaire : facteur, produit, multiple ; commutativité.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-mul-tables-2-5` | 1 | Tables de 2, 3, 4, 5 et 10 | a × b avec a ∈ {2, 3, 4, 5, 10}, b de 1 à 10, ordre a × b ou b × a tiré au hasard | `result`, `missingOperand` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 4 × 7 = ?
- … × 5 = 35
- 30 = 3 × …

## Évolutions du moteur et de l'écran de série

- Aucun : opération `multiplication`, facteur manquant et écran **Mes tables** sont créés au lot 1 (fiche « Exercices CE1 — Le signe × et les tables de multiplication »). Au CE2, les tables sont révisées et consolidées.

## Distracteurs (QCM)

Table voisine (4 × 6 au lieu de 4 × 7), ± le multiplicande (24, 32), somme au lieu du produit (11).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Tables de multiplication de 2, 3, 4, 5 et 10 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001602 (Tables de multiplication) : révision au CE2 (l'essentiel est repris au lot 1, CE1).
