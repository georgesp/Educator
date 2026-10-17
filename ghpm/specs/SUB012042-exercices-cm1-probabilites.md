---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXvI4
url: https://github.com/georgesp/Educator/issues/321
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012042 — Exercices CM1 — Probabilités
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 5180
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à dire si un événement est certain, possible ou impossible, et s'il est plus ou moins probable, afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1 : distinguer les événements certains, impossibles et ceux dont on ne peut pas affirmer a priori qu'ils se réaliseront ; estimer une probabilité sur une échelle allant de « impossible » à « certain » ; distinguer « peu probable » et « probable ».
- Livret d'accompagnement CM1 (2026) : séquence 3 « Enseigner les probabilités » (vocabulaire « impossible », « certain », « une chance sur deux », équiprobabilité) ; à aborder « sans attendre la fin de l'année scolaire ».

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-proba-events` | 2 | Certain, possible ou impossible | Sac de billes de couleurs, dé à 6 faces, roue à secteurs : l'événement est-il certain, possible ou impossible ? | `choice` |
| `cm1-proba-scale` | 4 | Plus ou moins probable | Placer un événement sur l'échelle impossible → peu probable → probable → certain ; « une chance sur deux » ; comparer deux sacs dans des cas évidents (même nombre de billes, ou une couleur absente d'un sac) | `choice`, `probabilityScale` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Dans un sac de 5 billes rouges, tirer une bille rouge → certain.
- Avec un dé à 6 faces, obtenir 7 → impossible.
- Dans un sac de 3 billes rouges et 1 bleue, tirer la bleue → peu probable.
- Pile ou face : obtenir pile → une chance sur deux.
- Dans quel sac as-tu le plus de chances de tirer une rouge : 3 rouges et 1 bleue, ou 1 rouge et 3 bleues ?

## Évolutions du moteur et de l'écran de série

- Illustrations générées à partir de la compétence : sac de billes (couleurs et nombres), dé, roue à secteurs, dans les couleurs du thème et avec un motif en plus de la couleur (lisible par un enfant daltonien).
- Format `probabilityScale` : glisser l'événement sur une échelle à quatre zones (impossible, peu probable, probable, certain).
- Aucun tirage aléatoire pendant l'exercice : la réponse porte sur l'événement, jamais sur un résultat tiré.

## Distracteurs (QCM)

« Possible » confondu avec « probable », « peu probable » confondu avec « impossible », vouloir prédire le résultat d'un tirage, sac choisi d'après le nombre de billes de la couleur sans regarder les autres.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Probabilités » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Livret d'accompagnement de mathématiques CM1 (eduscol, 2026)](https://eduscol.education.gouv.fr/sites/default/files/document/2026livretaccompagnementmathematiquescm1-129268.pdf)
