---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACY9o
rev: 2026-10-11T18:49:46Z
url: https://github.com/georgesp/Educator/issues/127
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011905 — Exercices CE2 — Comprendre la division
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 1170
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à trouver combien de parts ou combien dans chaque part, et écrire une division afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2 : comprendre le sens de la division et utiliser le symbole ÷ ; division comme opération inverse de la multiplication (ex. 7 × 13 = 91, donc 91 ÷ 7 = 13).
- Problèmes de partage et de groupement (déjà au CP : 24 élèves en équipes de 4).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-div-tables` | 4 | Divisions exactes avec les tables | a ÷ b avec a = b × q, b et q de 2 à 10 | `result`, `phrase` (partage / groupement) |
| `ce2-div-inverse` | 5 | Division, inverse de la multiplication | Une multiplication donnée (7 × 13 = 91) puis la division associée (91 ÷ 7) ; q ≤ 99 | `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 42 ÷ 6 = ?
- 24 élèves en équipes de 4 : combien d'équipes ?
- 7 × 13 = 91, donc 91 ÷ 7 = ?

## Évolutions du moteur et de l'écran de série

- Opération `division` (exacte seulement au CE2 : a = b × q, jamais de division par 0) — reprise de SUB001604 ; symbole « ÷ » lu « divisé par ».
- Format `phrase` avec petits énoncés de partage et de groupement (gabarits, voir la fiche « Exercices CE2 — Résoudre des problèmes »).

## Distracteurs (QCM)

Produit au lieu du quotient, quotient de la table voisine, diviseur et quotient inversés dans l'énoncé.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Comprendre la division » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001604 (Division) : partie CE2 (sens, division exacte).
