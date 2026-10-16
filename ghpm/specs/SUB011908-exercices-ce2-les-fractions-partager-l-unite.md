---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZC4
rev: 2026-10-11T18:49:49Z
url: https://github.com/georgesp/Educator/issues/130
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011908 — Exercices CE2 — Les fractions : partager l'unité"
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 1190
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à lire, écrire et représenter des fractions afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2 : fractions inférieures ou égales à 1, dénominateurs ≤ 12 ; égalités de fractions (5/10 = 1/2, six huitièmes = trois quarts).
- À partir de la période 3 : graduer une bande-unité en fractions, mesurer des longueurs non entières (1 unité + 3/10).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-frac-read-write` | 3 | Lire, écrire et représenter des fractions | Fraction a/b, b ≤ 12, a ≤ b : bande coloriée → écriture chiffrée ; mots (trois quarts) ↔ chiffres | `fractionPicture`, `fractionWords` |
| `ce2-frac-equal` | 3 | Fractions égales | Égalités avec dénominateurs multiples (1/2 = …/10, 3/4 = 6/…), dénominateurs ≤ 12 | `missingOperand` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Quelle fraction de la bande est coloriée ? → 3/8.
- Écris en chiffres : cinq sixièmes → 5/6.
- 1/2 = …/10 → 5.

## Évolutions du moteur et de l'écran de série

- Type **fraction**, saisie et lecture Piper créés au lot 1 (fiche « Exercices CE1 — Les premières fractions ») ; étendus aux dénominateurs jusqu'à 12 (septièmes, neuvièmes, onzièmes, douzièmes) et aux égalités de fractions.
- Réponse fraction : **toute fraction égale est acceptée** (2/4 pour 1/2) ; le retour montre la forme attendue.

## Distracteurs (QCM)

Numérateur et dénominateur inversés, parts non coloriées comptées, nombre de traits compté au lieu du nombre de parts.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Les fractions : partager l'unité » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001605 (Nombres décimaux et fractions simples) : fractions du CE2.
