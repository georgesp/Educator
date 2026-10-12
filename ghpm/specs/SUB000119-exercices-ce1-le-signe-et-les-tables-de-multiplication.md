---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACX3E
rev: 2026-10-11T18:48:55Z
url: https://github.com/georgesp/Educator/issues/87
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000119 — Exercices CE1 — Le signe × et les tables de multiplication
status: In progress
size: L
estimate:
startDate:
endDate:
parent: "#1"
order: 1230
---

## User story

En tant qu'enfant de CE1, je veux m'entraîner à comprendre la multiplication et apprendre les tables de multiplication afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE1 : utiliser le symbole « × » ; la multiplication est commutative ; tables de multiplication dans les deux sens (A × B = C avec A et B de 0 à 10 : huit égalités en une minute en fin de CE1).
- L'apprentissage des tables « s'étale sur l'année scolaire tout entière, de manière progressive » ; leur mémorisation peut être encore imparfaite en fin de CE1.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce1-mul-meaning` | 2 | Le signe × | Addition répétée ↔ multiplication (5 + 5 + 5 = 3 × 5) ; rangées d'objets → produit ; a de 2 à 5, b ≤ 10 | `result`, `phrase` |
| `ce1-mul-tables-2-5-10` | 3 | Tables de 2, 5 et 10 | a × b avec a ∈ {2, 5, 10}, b de 0 à 10, ordre au hasard | `result`, `missingOperand` |
| `ce1-mul-tables-3-4` | 4 | Tables de 3 et 4 | a × b avec a ∈ {3, 4}, b de 0 à 10, révision des tables de 2, 5 et 10 mêlée | `result`, `missingOperand` |
| `ce1-mul-tables-6-9` | 5 | Tables de 6 à 9 (découverte) | a × b avec a de 6 à 9 et b de 0 à 10, faits les plus simples d'abord (× 2, × 5, × 10 commutés) | `result`, `missingOperand` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 5 + 5 + 5 = … × 5 → 3.
- 2 × 7 = ?
- … × 5 = 40 → 8.
- 4 × 6 = ?

## Évolutions du moteur et de l'écran de série

- Opération `multiplication` dans le moteur et le schéma (reprise de SUB001602) ; « × » lu « fois » — créée ici, réutilisée par le CE2 et le cycle 3.
- Format `missingOperand` aux deux positions et égalité inversée (30 = 3 × …).
- Écran **Mes tables** : affichage d'une table en grand, lue par Piper (SUB001602).

## Distracteurs (QCM)

Somme au lieu du produit (2 × 7 = 9), table voisine, ± un facteur.

## Critères d'acceptation

- [x] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [x] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [x] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE1 — Le signe × et les tables de multiplication » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001602 (Tables de multiplication) : opération, distracteurs, écran Mes tables. Les tables restent révisées au CE2 (lot 4).

## Remarques et décisions

- Le programme ne fixe pas l'ordre des tables au CE1 : l'ordre 2-5-10, puis 3-4, puis 6 à 9 est un choix d'Educator, signalé dans `source`.

## Réalisation (2026-10-11)

- « Mes tables » : écran ouvert depuis les exercices quand la classe a des multiplications ; la table (de 2 à 10) est fournie par le cœur (commande `get_multiplication_table`).
- Tables de 3 et 4 : 7 exercices sur 10 sur les tables de 3 et 4, les autres révisent 2, 5 et 10. Tables de 3-4 et de 6 à 9 : facteur de 1 à 10 (le × 0 reste travaillé avec les tables de 2, 5 et 10) ; `0 × ? = 0` n'est jamais demandé.
- `ce1-mul-meaning` : `phrase` = addition itérée (`5 + 5 + 5 = ? × 5`) ; `result` = produit avec les rangées de points dessinées.
- Restent ouverts : relecture des échantillons (`docs/specs/echantillons-ce1.md`), lecture par Piper (voix coupée dans les maths pour l'instant), lien avec la leçon (format des leçons au lot 2).
