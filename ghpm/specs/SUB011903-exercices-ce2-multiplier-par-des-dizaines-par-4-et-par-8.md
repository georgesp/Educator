---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACY40
rev: 2026-10-11T18:49:37Z
url: https://github.com/georgesp/Educator/issues/125
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011903 — Exercices CE2 — Multiplier par des dizaines, par 4 et par 8"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1110
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à multiplier de tête par 20, 30… et par 4 ou 8 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2, calcul mental : multiplier un nombre inférieur à 10 par un nombre entier de dizaines ; multiplier un nombre entier par 4 ou par 8.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-mul-by-tens` | 3 | Multiplier par un nombre de dizaines | a × (b × 10) avec a et b de 2 à 9 (7 × 30), ordre au hasard | `result`, `missingOperand` |
| `ce2-mul-by-4-8` | 3 | Multiplier par 4 ou par 8 | n × 4 (n ≤ 250) et n × 8 (n ≤ 125), résultat ≤ 1 000 | `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 7 × 30 = ? → 210.
- 40 × 6 = ? → 240.
- 35 × 4 = ? → 140.
- 25 × 8 = ? → 200.

## Évolutions du moteur et de l'écran de série

- Aucun après l'opération `multiplication`.

## Distracteurs (QCM)

Zéro oublié (21 au lieu de 210), × 4 calculé comme + 4, double au lieu du double du double.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Multiplier par des dizaines, par 4 et par 8 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
