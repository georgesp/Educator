---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACa0E
rev: 2026-10-11T18:51:49Z
url: https://github.com/georgesp/Educator/issues/219
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012207 — Exercices CP — Partager et faire des groupes
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2040
---

## User story

En tant qu'enfant de CP, je veux m'entraîner à partager équitablement et faire des groupes égaux afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CP : partages équitables (ex. 24 élèves en équipes de 4 donnent 6 équipes ; 18 images pour 3 enfants donnent 6 chacun), nombres ≤ 30.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cp-share-group` | 5 | Partager et grouper | Partage de n ≤ 30 objets entre 2 à 5 enfants (sans reste) ; groupes de 2 à 5 dans n ≤ 30 (sans reste) | `phrase`, `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 18 images pour 3 enfants : combien chacun ? → 6.
- 24 élèves en équipes de 4 : combien d'équipes ? → 6.

## Évolutions du moteur et de l'écran de série

- Format `phrase` avec gabarits de partage et de groupement, illustrés par `groupObjects` (pas de signe ÷ au CP).

## Distracteurs (QCM)

Soustraction au lieu du partage (18 − 3 = 15), confusion nombre de parts / taille des parts.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CP — Partager et faire des groupes » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
