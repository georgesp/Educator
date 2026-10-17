---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACY_I
rev: 2026-10-11T18:49:31Z
url: https://github.com/georgesp/Educator/issues/128
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011906 — Exercices CE2 — Doubles, moitiés et faits multiplicatifs usuels"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1070
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à connaître les doubles, les moitiés et les multiples de 25 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2, faits multiplicatifs usuels :
  - doubles des nombres de 1 à 20 et de 20, 25, 30, 35, 40, 45, 50, 60, 75, 100, 150, 200, 250, 300, 400, 500, 600 ;
  - moitiés des nombres pairs de 2 à 40 et de 40, 50, 60, 70, 80, 90, 100, 120, 150, 200, 300, 400, 500, 600, 800, 1 000, 1 200 ;
  - multiples de 25 (1 × 25 à 4 × 25) ; décompositions de 60 (1 × 60, 2 × 30, 3 × 20, 4 × 15, 5 × 12, 6 × 10).
- Fluence attendue en fin de CE2 : 12 égalités en une minute (ex. 2 × … = 70, 1 000 = 2 × …).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-double-half` | 2 | Doubles et moitiés | Doubles (`doubles`) et moitiés (`halves`) : exactement les deux listes du programme ci-dessus (jamais la moitié de 25, 35, 45 ou 75) ; formats « le double de … », « la moitié de … » et 2 × … = 70 | `phrase` (« le double de … »), `result` |
| `ce2-mul-facts-25-60` | 3 | Multiples de 25 et décompositions de 60 | Multiples de 25 (`timesTwentyFive` : 1 à 4) ; décompositions de 60 (nouveau champ `products: [60]`) : 60 = 4 × …, 5 × … = 60 | `result`, `missingOperand` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Le double de 17 → 34.
- La moitié de 36 → 18.
- 3 × 25 = ? → 75.
- 60 = 4 × … → 15.

## Évolutions du moteur et de l'écran de série

- Format `phrase` (lot 1, CE1) : consigne en mots générée (« Quel est le double de 17 ? »).
- Listes du programme dans les champs `doubles`, `halves` et `timesTwentyFive` de la compétence (existants depuis le CE1, `ce1-double-half-facts`).
- Nouveau champ `products` (opération `doubleHalf`) : produits dont l'enfant cherche un facteur (60 = 4 × …) ; décompositions tirées parmi celles en facteurs ≥ 1 ; validé par le schéma.
- La compétence est aussi proposée au défi chrono (12 égalités en une minute).

## Distracteurs (QCM)

Double et moitié confondus, + 2 au lieu de × 2, moitié d'un nombre à deux chiffres prise chiffre par chiffre (36 → 13).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Doubles, moitiés et faits multiplicatifs usuels » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
