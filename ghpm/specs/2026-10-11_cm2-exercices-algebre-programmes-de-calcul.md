---
type: Issue
title: Exercices CM2 — Égalités à trous, programmes de calcul et suites
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-3-cm2.md"
order: 2290
---
## User story

En tant qu'enfant de CM2, je veux m'entraîner à trouver des nombres inconnus, produire des programmes de calcul et prolonger des suites afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM2, algèbre : nombre manquant dans une égalité à trous ; problèmes algébriques ; exécuter ou produire un programme de calcul ; formuler une règle pour une suite ; poursuivre une suite de motifs et trouver le nombre d'éléments à une étape donnée.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm2-algebra-missing` | 2 | Égalités à trous et problèmes algébriques | Égalités à deux opérations (3 × … + 5 = 26), problèmes « un nombre caché » à une étape | `missingOperand`, `problem` |
| `cm2-sequences` | 4 | Programmes de calcul et suites | Programme de 3 étapes, programme inverse (trouver le nombre de départ), étape n d'une suite de motifs (n ≤ 20) | `program`, `sequence` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 3 × … + 5 = 26 → 7.
- Je pense à un nombre, je le multiplie par 4, j'enlève 3 : j'obtiens 25. Mon nombre ? → 7.
- Motif : 1, 4, 7, 10 drapeaux… combien à l'étape 10 ? → 28.

## Évolutions du moteur et de l'écran de série

- Format `program` étendu au programme inverse ; `sequence` avec question à l'étape n.

## Distracteurs (QCM)

Inverse des opérations dans le mauvais ordre, terme suivant donné au lieu de l'étape n.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM2 — Égalités à trous, programmes de calcul et suites » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
