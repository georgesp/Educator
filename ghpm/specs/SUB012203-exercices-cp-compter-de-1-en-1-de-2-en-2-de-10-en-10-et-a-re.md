---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACauw
rev: 2026-10-12T19:26:23Z
url: https://github.com/georgesp/Educator/issues/215
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012203 — Exercices CP — Compter de 1 en 1, de 2 en 2, de 10 en 10 et à rebours"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 1960
---

## User story

En tant qu'enfant de CP, je veux m'entraîner à compter en avançant et en reculant, de 1 en 1, de 2 en 2 et de 10 en 10 afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CP : compter de 1 en 1, de 2 en 2 et de 10 en 10 à partir de n'importe quel nombre ; compter à rebours de 1 en 1 ; suites répétitives.
- Ordinaux jusqu'à « vingtième » : repérer un rang dans une file, relier le rang au nombre d'éléments qui précèdent (ex. le quatrième coureur, le sixième gobelet).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cp-count-1-2` | 1 | Compter de 1 en 1 et de 2 en 2 | Suite de 4 à 6 nombres ≤ 60, un ou deux nombres cachés ; avant / après | `sequence` |
| `cp-count-10-back` | 2 | Compter de 10 en 10 et à rebours | Suite de 10 en 10 depuis n'importe quel nombre ≤ 100 ; à rebours de 1 en 1 | `sequence` |
| `cp-ordinals` | 2 | Les nombres ordinaux | Rang d'une voiture dans une file de 5 à 20 (« Quelle est la 4e voiture ? », « À quel rang est la voiture rouge ? ») ; premier à vingtième, lus par Piper ; lien entre rang et nombre d'éléments qui précèdent | `choice` (clic sur l'image), `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 12, 14, 16, …, 20 → 18.
- 7, 17, 27, …, 47 → 37.
- 31, 30, 29, … → 28.
- Clique sur la sixième voiture.
- La voiture bleue est à quel rang ? → 3e (2 voitures devant elle).

## Évolutions du moteur et de l'écran de série

- Format `sequence` (lot 5) disponible au CP : suites courtes, cases vides au milieu ou à la fin.
- Format `choice` sur image : une file de voitures dessinées, l'enfant clique sur celle demandée ; ordinaux lus par Piper (« sixième »).

## Distracteurs (QCM)

Pas de 1 au lieu de 2, sens inversé pour le compte à rebours.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CP — Compter de 1 en 1, de 2 en 2, de 10 en 10 et à rebours » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
