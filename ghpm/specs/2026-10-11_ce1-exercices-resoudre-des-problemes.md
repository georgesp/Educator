---
type: Issue
title: Exercices CE1 — Résoudre des problèmes
status: Todo
size: L
parent: "#1"
order: 1260
---
## User story

En tant qu'enfant de CE1, je veux m'entraîner à résoudre des problèmes additifs, multiplicatifs et en deux étapes afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE1 : au moins dix problèmes par semaine, champ numérique jusqu'à 1 000.
- Parties-tout en une étape (transformations incluses) ; comparaison additive en une étape ; additifs en deux étapes ; multiplicatifs en une étape (valeur du tout, partage équitable : nombre de parts ou valeur d'une part) ; mixtes en deux étapes (une additive, une multiplicative).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce1-problems-additive` | 1 | Problèmes additifs | Gabarits parties-tout, transformation et comparaison (« de plus », « de moins »), une étape, nombres ≤ 100 puis ≤ 1 000 | `problem` |
| `ce1-problems-two-steps` | 3 | Problèmes en deux étapes | Gabarits additifs en deux étapes, nombres ≤ 1 000 | `problem` |
| `ce1-problems-multiplicative` | 4 | Problèmes multiplicatifs et mixtes | Valeur du tout (4 boîtes de 6), partage équitable (24 pour 4), mixtes en deux étapes ; nombres ≤ 100 | `problem` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Tom a 35 billes, Léa en a 12 de plus. Combien Léa a-t-elle de billes ? → 47.
- 4 boîtes de 6 pneus : combien de pneus ? → 24.
- Un garage a 3 rangées de 8 voitures ; 5 voitures partent. Combien en reste-t-il ? → 19.

## Évolutions du moteur et de l'écran de série

- Format `problem` : énoncés générés à partir de **gabarits** (`content/problems/*.json`, validés par schéma), prénoms, objets et contextes variés, lus par Piper, image de la situation — repris de SUB001606 ; réutilisé par tous les niveaux.
- Aide « schéma » après une erreur : `barModel` rempli avec les données de l'énoncé.

## Distracteurs (QCM)

Mauvaise opération (« de plus » → addition alors qu'on cherche la plus petite quantité), une seule étape faite, nombre de l'énoncé recopié.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE1 — Résoudre des problèmes » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001606 (Nouveaux types d'exercices) : « petits problèmes illustrés » et gabarits.

## Remarques et décisions

- Prénoms variés et non stéréotypés, contextes du quotidien de l'enfant ; liste relue par Georges.
