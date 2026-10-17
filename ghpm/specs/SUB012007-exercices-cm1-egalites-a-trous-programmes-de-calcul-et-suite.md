---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZtA
rev: 2026-10-11T18:50:42Z
url: https://github.com/georgesp/Educator/issues/161
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012007 — Exercices CM1 — Égalités à trous, programmes de calcul et suites"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1560
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à trouver un nombre inconnu, suivre un programme de calcul et continuer une suite afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1, algèbre : trouver le nombre manquant dans une égalité à trous ; déterminer un nombre inconnu à l'aide d'un symbole ; exécuter un programme de calcul ; identifier une règle pour poursuivre une suite ; poursuivre une suite de motifs évolutive.
- Livret d'accompagnement CM1 (2026) : séquence sur les problèmes algébriques (égalités à trou, programmes de calcul).
- Livret d'accompagnement CM1 (2026) : l'algèbre et les probabilités doivent être abordées « sans attendre la fin de l'année scolaire ».

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-algebra-missing` | 2 | Égalités à trous | … + 37 = 92, 6 × … = 54, 120 − … = 75, … ÷ 4 = 9 ; un symbole (★) peut remplacer la case ; petites situations de balance (2 boîtes identiques + 3 = 11) | `missingOperand`, `balance` |
| `cm1-calc-program` | 3 | Programmes de calcul et suites | Programme en 2 ou 3 étapes (« choisis 7, multiplie par 3, ajoute 5 ») ; suite arithmétique ou de motifs, trouver le terme suivant | `program`, `sequence` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- … + 37 = 92 → 55.
- Choisis 7, multiplie par 3, ajoute 5 → 26.
- 4, 7, 10, 13, … → 16.
- 2 boîtes identiques + 3 = 11 : combien vaut une boîte ? → 4.

## Évolutions du moteur et de l'écran de série

- Formats `program` (étapes affichées et lues) et `sequence` (suite de nombres ou de motifs SVG simples) générés par règles.
- Affichage `balance` (SVG : deux plateaux, boîtes identiques et poids connus), comme dans la séquence 2 du livret.

## Distracteurs (QCM)

Opération inverse mal choisie (… + 37 = 92 → 129), étape oubliée dans le programme.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Égalités à trous, programmes de calcul et suites » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
