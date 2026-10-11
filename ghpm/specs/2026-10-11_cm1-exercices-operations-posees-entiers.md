---
type: Issue
title: Exercices CM1 — Opérations posées sur les entiers
status: Todo
size: L
parent: "specs/2026-10-11_lot-1-2-cm1.md"
order: 1850
---
## User story

En tant qu'enfant de CM1, je veux m'entraîner à poser des additions, des soustractions et des multiplications de grands nombres afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : additions, soustractions et multiplications posées utilisées dès le début de l'année selon les nombres en jeu ; multiplier deux entiers ; estimer un ordre de grandeur.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-add-sub-column` | 1 | Additions et soustractions posées | Nombres ≤ 99 999 puis ≤ 999 999 (période 3), plusieurs retenues ou cassages | `result`, affichage posé |
| `cm1-mul-column` | 2 | Multiplications posées | Nombre de 2 à 4 chiffres × nombre de 2 ou 3 chiffres, résultat ≤ 999 999 | `result`, affichage posé |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 45 708 + 9 386 = ?
- 60 004 − 7 568 = ?
- 326 × 47 = ? → 15 322.

## Évolutions du moteur et de l'écran de série

- Composant d'opération posée (lot 1.1) : multiplication à trois lignes de produits partiels ; soustraction selon la technique du profil (cassage ou compensation).

## Distracteurs (QCM)

Décalage oublié, retenue mal reportée, cassage en cascade raté sur les zéros.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Opérations posées sur les entiers » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
