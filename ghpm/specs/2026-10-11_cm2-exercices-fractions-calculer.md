---
type: Issue
title: Exercices CM2 — Calculer avec les fractions
status: Todo
size: M
parent: "specs/2026-10-11_lot-1-3-cm2.md"
order: 2230
---
## User story

En tant qu'enfant de CM2, je veux m'entraîner à additionner et soustraire des fractions, multiplier une fraction par un entier et prendre une fraction d'une quantité afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM2 : additionner et soustraire des fractions ; calculer le produit d'un entier et d'une fraction ; déterminer une fraction d'une quantité.
- Livret d'accompagnement CM2 : additions et soustractions avec des dénominateurs dont l'un est multiple de l'autre (1/4 + 3/8, 1/2 + 3/10, 3/8 − 1/4) ; la simplification n'est pas attendue.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm2-frac-add-sub` | 2 | Additionner et soustraire des fractions | Dénominateurs multiples l'un de l'autre (deux ou trois fractions), résultat ≥ 0 | `result` (fraction) |
| `cm2-frac-times-int` | 3 | Entier × fraction et fraction d'une quantité | n × a/b (4 × 3/5 = 12/5), a/b de N | `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 1/4 + 3/8 = ? → 5/8.
- 3/8 − 1/4 = ? → 1/8.
- 4 × 3/5 = ? → 12/5.
- 3/4 de 60 min = ? → 45 min.

## Évolutions du moteur et de l'écran de série

- Réponse fraction : toute fraction égale acceptée, sans simplification exigée (comme le précise le livret).

## Distracteurs (QCM)

Additionner numérateurs et dénominateurs, multiplier aussi le dénominateur (4 × 3/5 = 12/20).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM2 — Calculer avec les fractions » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
