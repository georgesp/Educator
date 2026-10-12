---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACaY8
url: https://github.com/georgesp/Educator/issues/196
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012108 — Exercices CM2 — Les grands nombres jusqu'au milliard
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#121"
order: 2110
---

## User story

En tant qu'enfant de CM2, je veux m'entraîner à lire, écrire, décomposer et comparer les nombres jusqu'à 999 999 999 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM2 : trois nouveaux rangs (unités, dizaines et centaines de millions) ; attendus sur des nombres d'au plus neuf chiffres.
- Pendant les deux premières périodes, nombres d'au plus six chiffres (ceux du CM1).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm2-num-read-write` | 3 | Lire et écrire les nombres jusqu'au milliard | Nombre de 1 000 000 à 999 999 999 dicté → chiffres ; décomposition par classes | `writeNumber`, `readNumber`, `decompose` |
| `cm2-num-compare` | 3 | Comparer, ranger et encadrer les grands nombres | Nombres de 7 à 9 chiffres proches ; encadrer au million, à la dizaine de millions | `compare`, `order`, `frame` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Écris en chiffres : « vingt-trois millions quatre cent mille cinq » → 23 400 005.
- Combien de millions dans 452 380 100 ? → 452.
- Encadre 23 400 005 entre deux millions.

## Évolutions du moteur et de l'écran de série

- Conversion nombre → français écrit jusqu'au milliard, orthographe traditionnelle (« millions » au pluriel, « mille » invariable) ; graphie rectifiée de 1990 acceptée en saisie.

## Distracteurs (QCM)

Classe vide oubliée (2 340 005), confusion millions / milliers.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM2 — Les grands nombres jusqu'au milliard » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM2 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026-livret-accompagnement-mathematiques-cm2-128556.pdf)
