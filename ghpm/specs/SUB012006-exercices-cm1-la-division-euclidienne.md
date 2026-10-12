---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZr0
url: https://github.com/georgesp/Educator/issues/160
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012006 — Exercices CM1 — La division euclidienne
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#120"
order: 1870
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à calculer un quotient et un reste et poser une division afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : diviser euclidiennement par un diviseur à un chiffre (division posée).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-div-remainder` | 3 | Quotient et reste | a ÷ b avec b de 2 à 9, a ≤ 99 (avec les tables), reste < b ; problèmes de partage avec reste | `quotientRemainder` |
| `cm1-div-column` | 4 | Division posée par un nombre à un chiffre | Dividende de 3 ou 4 chiffres, diviseur de 2 à 9, reste quelconque | `quotientRemainder`, affichage posé |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 38 ÷ 5 : q = 7, r = 3.
- 847 ÷ 6 : q = 141, r = 1.
- 57 élèves, voitures de 4 places : combien de voitures ?

## Évolutions du moteur et de l'écran de série

- Opération `division` avec reste (SUB001604) : saisie du quotient et du reste dans deux champs ; propriété testée : dividende = diviseur × quotient + reste, reste < diviseur, jamais de division par 0.
- Composant d'opération posée étendu à la division (potence, soustractions successives).

## Distracteurs (QCM)

Reste ≥ diviseur, quotient d'une table voisine, zéro intermédiaire oublié au quotient (847 ÷ 4 → 21 au lieu de 211).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — La division euclidienne » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)

## Fiches existantes reprises

SUB001604 (Division) : division avec reste et division posée.
