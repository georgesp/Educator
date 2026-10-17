---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACats
rev: 2026-10-17T13:46:37Z
url: https://github.com/georgesp/Educator/issues/214
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012202 — Exercices CP — Comparer, ranger et placer les nombres jusqu'à 100"
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 1940
---

## User story

En tant qu'enfant de CP, je veux m'entraîner à comparer et ranger les nombres et les placer sur une ligne graduée afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CP : comparer avec =, < et > (ex. 49 et 53), ordonner jusqu'à cinq nombres, placer des nombres sur une demi-droite graduée de 1 en 1.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cp-num-compare` | 3 | Comparer et ranger les nombres | Deux nombres ≤ 100 → `<`, `>`, `=` ; ranger 3 à 5 nombres | `compare`, `order` |
| `cp-num-line` | 4 | Placer les nombres sur une ligne graduée | Ligne graduée de 1 en 1 (morceau de 10 à 20 cases, repères aux dizaines) : quel nombre ? où est … ? | `numberLinePoint` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 49 … 53 → <.
- Range du plus petit au plus grand : 38, 83, 35.
- Quel nombre est sous la flèche ? (entre 40 et 50)

## Évolutions du moteur et de l'écran de série

- Formats `compare`, `order` et `numberLinePoint` (lot 1, CE1), plages CP, tailles de boutons pour le CP.

## Distracteurs (QCM)

Signe inversé, comparaison sur le chiffre des unités (49 > 53).

## Critères d'acceptation

- [x] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [x] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [x] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md` (nouveau fichier ; les 11 compétences du lot 1 restent dans `echantillons-lot1.md`), relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement (décision du 2026-10-15).
- [ ] Exercices jouables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [x] La compétence est liée à la fiche « Leçon CP — Comparer, ranger et placer les nombres jusqu'à 100 » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Remarques et décisions

- Implémentation (2026-10-17) : paramètre `lineSteps` des compétences de numération (`[1]` au CP) ; les propositions restent dans les nombres travaillés (jamais 130 sur une ligne du CP) ; « = » limité à une comparaison sur dix.
