---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZFc
rev: 2026-10-11T18:49:43Z
url: https://github.com/georgesp/Educator/issues/131
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011909 — Exercices CE2 — Multiplication posée
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 1150
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à poser une multiplication par un nombre à un ou deux chiffres afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2 : poser et effectuer des multiplications d'un nombre à deux ou trois chiffres par un nombre à un ou deux chiffres (ex. 16 × 548) ; algorithme introduit au plus tard en période 4.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-mul-column-1-digit` | 4 | Multiplication posée par un nombre à un chiffre | Nombre de 2 ou 3 chiffres × nombre de 2 à 9, avec retenues | `result`, affichage posé |
| `ce2-mul-column-2-digits` | 5 | Multiplication posée par un nombre à deux chiffres | Nombre de 2 ou 3 chiffres × nombre de 11 à 99, résultat ≤ 10 000 (programme : nombres ≤ 10 000) | `result`, affichage posé |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 247 × 3 = ?
- 38 × 24 = ?
- 16 × 548 → posée avec 548 en haut = 8 768.

## Évolutions du moteur et de l'écran de série

- Composant d'opération posée étendu à la multiplication : lignes de produits partiels, zéro (ou point) de décalage, retenues, somme finale ; saisie chiffre par chiffre et correction ciblée (SUB001603).

## Distracteurs (QCM)

Retenue oubliée ou ajoutée avant de multiplier, oubli du décalage de la deuxième ligne, erreur de table.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Multiplication posée » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001603 (Multiplication posée et calcul mental multiplicatif) : partie CE2.
