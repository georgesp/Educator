---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACX9o
rev: 2026-10-11T18:48:52Z
url: https://github.com/georgesp/Educator/issues/91
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000123 — Exercices CE1 — Les premières fractions
status: In progress
size: L
estimate:
startDate:
endDate:
parent: "#1"
order: 1220
---

## User story

En tant qu'enfant de CE1, je veux m'entraîner à lire, écrire, comparer et ajouter des fractions simples afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE1 : fractions 1/2, 1/3, 1/4, 1/5, 1/6, 1/8 et 1/10, puis fractions non unitaires inférieures ou égales à 1 ; mots « numérateur » et « dénominateur ». Le travail « commence dès la période 2 » avec les fractions unitaires.
- Comparer des fractions de même dénominateur et des fractions de numérateur 1 (cas simples « dès la période 4 ») ; additionner et soustraire des fractions de même dénominateur.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce1-frac-unit` | 2 | Les fractions unitaires | 1/b avec b ∈ {2, 3, 4, 5, 6, 8, 10} : bande ou disque partagé → écriture chiffrée ; mots (un tiers) ↔ chiffres | `fractionPicture`, `fractionWords` |
| `ce1-frac-read-write` | 3 | Lire et écrire des fractions jusqu'à 1 | a/b ≤ 1, mêmes dénominateurs : partie coloriée → fraction ; « trois quarts » → 3/4 | `fractionPicture`, `fractionWords` |
| `ce1-frac-compare-add` | 4 | Comparer et ajouter des fractions | Comparer à même dénominateur ou de numérateur 1 (1/3 et 1/5) ; a/b + c/b et a/b − c/b, résultat ≤ 1 | `compare`, `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Quelle fraction de la bande est coloriée ? → 1/4.
- Écris en chiffres : trois quarts → 3/4.
- 1/3 … 1/5 → >.
- 2/8 + 3/8 = ? → 5/8.

## Évolutions du moteur et de l'écran de série

- Type de nombre **fraction** dans le cœur (numérateur, dénominateur, égalité par produit en croix, sans flottants) — repris de SUB001605 ; étendu ensuite au CE2 et au cycle 3.
- Saisie d'une fraction (deux champs superposés), image de bande ou de disque dans l'exercice (réutilise `fractionBar`), lecture Piper (« un tiers », « trois quarts »).
- Réponse fraction : **toute fraction égale est acceptée** (décision de Georges du 2026-10-11) ; le retour montre la forme attendue.

## Distracteurs (QCM)

Numérateur et dénominateur inversés, « 1/5 > 1/3 parce que 5 > 3 », dénominateurs additionnés (2/8 + 3/8 = 5/16).

## Critères d'acceptation

- [x] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [x] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [x] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE1 — Les premières fractions » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001605 (Nombres décimaux et fractions simples) : type fraction et premières fractions.

## Réalisation (2026-10-11)

- Saisie d'une fraction : numérateur et dénominateur l'un au-dessus de l'autre ; en QCM, les fractions sont écrites de la même façon.
- Restent ouverts : relecture des échantillons (`docs/specs/echantillons-ce1.md`), lecture par Piper (voix coupée dans les maths pour l'instant), lien avec la leçon (format des leçons au lot 2).
