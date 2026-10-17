---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZ38
rev: 2026-10-12T19:26:19Z
url: https://github.com/georgesp/Educator/issues/169
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012015 — Exercices CM1 — Opérations posées sur les entiers
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#120"
order: 1420
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à poser des additions, des soustractions et des multiplications de grands nombres et à estimer leur résultat afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : additions, soustractions et multiplications posées utilisées dès le début de l'année selon les nombres en jeu ; multiplier deux entiers ; estimer un ordre de grandeur.
- Périodes 1 et 2 : nombres d'au plus quatre chiffres ; cinq ou six chiffres à partir de la période 3 ou du début de la période 4.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-add-sub-column` | 1 | Additions et soustractions posées | Nombres ≤ 9 999 en périodes 1 et 2, puis ≤ 999 999 à partir de la période 3, plusieurs retenues ou cassages | `result`, affichage posé |
| `cm1-mul-column` | 2 | Multiplications posées | Nombre de 2 ou 3 chiffres × nombre de 2 chiffres, résultat ≤ 9 999 en période 2 ; jusqu'à 4 chiffres × 3 chiffres, résultat ≤ 999 999, à partir de la période 3 | `result`, affichage posé |
| `cm1-estimate` | 2 | Estimer un ordre de grandeur | Choisir parmi trois l'ordre de grandeur d'une somme, d'une différence ou d'un produit (212 × 38 ≈ 8 000 ; 2 987 + 4 012 ≈ 7 000) ; distracteurs 10 fois plus petit et 10 fois plus grand ; résultat ≤ 9 999 en période 2 | `choice` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 4 578 + 3 986 = ? → 8 564.
- 6 004 − 2 568 = ? → 3 436.
- 212 × 38 = ? → 8 056 (326 × 47 = 15 322 à partir de la période 3).
- Ordre de grandeur de 212 × 38 : 800, 8 000 ou 80 000 ? → 8 000.

## Évolutions du moteur et de l'écran de série

- Composant d'opération posée (lot 4) : multiplication à trois lignes de produits partiels ; soustraction selon la technique du profil (cassage ou compensation).
- Format `choice` d'ordre de grandeur (`cm1-estimate`) ; plage de nombres selon la période (section « Taille des nombres selon la période » de US0120).

## Distracteurs (QCM)

Décalage oublié, retenue mal reportée, cassage en cascade raté sur les zéros, ordre de grandeur décalé d'un rang (800 ou 80 000 pour 212 × 38).

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
