---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZ20
rev: 2026-10-11T18:50:33Z
url: https://github.com/georgesp/Educator/issues/168
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012014 — Exercices CM1 — Les nombres décimaux : dixièmes et centièmes"
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#120"
order: 1500
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à lire, écrire et décomposer des nombres décimaux afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : nombres décimaux introduits par les fractions décimales puis l'écriture à virgule, jusqu'aux centièmes.
- Période 1 : décimaux rencontrés dans des problèmes de monnaie ; période 2 : étude générale. Lire, écrire, passer d'une écriture à l'autre, donner la partie entière.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-dec-money` | 1 | Euros et centimes | Prix en euros et centimes (3,45 € = 3 € et 45 c) ; écrire un prix avec une virgule | `decimalWrite`, `result` |
| `cm1-dec-fractions` | 2 | Fractions décimales et écriture à virgule | Passer de 345/100 ou 3 + 4/10 + 5/100 à 3,45 et inversement ; partie entière, chiffre des dixièmes / centièmes | `decompose`, `recompose`, `placeValue` |
| `cm1-dec-usual-fractions` | 3 | Fractions usuelles et nombres décimaux | Faits à connaître : 1/2 = 0,5 ; 1/4 = 0,25 ; 3/4 = 0,75 ; 1/10 = 0,1 ; 1/100 = 0,01 ; 1/2 = 2/4 = 5/10 ; dans les deux sens | `result`, `choice` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 3 + 4/10 + 5/100 = ? → 3,45.
- Écris 207/100 avec une virgule → 2,07.
- Quel est le chiffre des dixièmes de 12,38 ? → 3.
- 3/4 = ? (écriture décimale) → 0,75.

## Évolutions du moteur et de l'écran de série

- Type **décimal** exact dans le cœur (entier + nombre de décimales, pas de flottant) — repris de SUB001605.
- Saisie avec la virgule française ; le point est accepté et converti ; zéros inutiles tolérés (3,50 = 3,5).
- Lecture Piper des décimaux (« trois virgule quarante-cinq », ou « 3 euros 45 » dans un contexte de prix).

## Distracteurs (QCM)

Confusion dixièmes / centièmes (3,045), 2,7 pour 207/100, virgule mal placée.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Les nombres décimaux : dixièmes et centièmes » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)

## Fiches existantes reprises

SUB001605 (Nombres décimaux et fractions simples) : type décimal, saisie, lecture.
