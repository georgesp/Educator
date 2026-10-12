---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZyI
url: https://github.com/georgesp/Educator/issues/165
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012011 — Exercices CM1 — Multiples, diviseurs et critères de divisibilité"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1750
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à reconnaître les multiples et les diviseurs et utiliser les critères de divisibilité par 2, 5 et 10 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : reconnaître les multiples et les diviseurs (inférieurs ou égaux à 10) ; critères de divisibilité uniquement par 2, 5 et 10, les autres cas s'appuyant sur les tables ou des divisions.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-divisibility` | 2 | Critères de divisibilité par 2, 5 et 10 | Nombre ≤ 9 999 : divisible par 2 ? par 5 ? par 10 ? (oui / non) ; trouver les nombres divisibles dans une liste | `yesNo`, `choice` |
| `cm1-multiples-divisors` | 3 | Multiples et diviseurs | « 42 est-il un multiple de 6 ? », « Donne un diviseur de 35 » (diviseurs ≤ 10), multiples dans une liste | `yesNo`, `choice` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 135 est-il divisible par 5 ? → oui.
- Lequel est un multiple de 7 : 48, 49, 50 ? → 49.
- Est-ce que 4 est un diviseur de 30 ? → non.

## Évolutions du moteur et de l'écran de série

- Formats `yesNo` et `choice` à plusieurs bonnes réponses créés au lot 1 (fiche « Exercices CE1 — Pair, impair, doubles et moitiés »).

## Distracteurs (QCM)

Critère du dernier chiffre mal appliqué (divisible par 5 si finit par 5 seulement), confusion multiple / diviseur.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Multiples, diviseurs et critères de divisibilité » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
