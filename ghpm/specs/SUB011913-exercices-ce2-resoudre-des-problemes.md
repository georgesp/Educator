---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACZK0
rev: 2026-10-11T18:49:56Z
url: https://github.com/georgesp/Educator/issues/135
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011913 — Exercices CE2 — Résoudre des problèmes
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 1230
---

## User story

En tant qu'enfant de CE2, je veux m'entraîner à résoudre des petits problèmes avec les quatre opérations afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2 : problèmes additifs en une étape (parties-tout, comparaison) et en deux étapes, multiplicatifs en une étape, mixtes en deux ou trois étapes, comparaison multiplicative, produits cartésiens.
- Résolution de problèmes au cœur des apprentissages (au moins dix problèmes par semaine au cycle 2).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-problems-additive` | 1 | Problèmes additifs | Gabarits parties-tout, transformation et comparaison, en une ou deux étapes ; nombres ≤ 10 000 (programme : le champ s'étend au-delà de 1 000) | `problem` |
| `ce2-problems-multiplicative` | 3 | Problèmes multiplicatifs | Gabarits de groupements, partages (6 dictionnaires pour 72 €, prix d'un ?), « fois plus » / « fois moins », produits cartésiens (3 pantalons × 7 tee-shirts) ; facteurs ≤ 10 ou nombre ≤ 100, résultats ≤ 1 000 | `problem` |
| `ce2-problems-mixed` | 5 | Problèmes en plusieurs étapes | Gabarits mixtes en deux ou trois étapes (4 tables de 6 et 7 tables de 4) ; nombres ≤ 1 000, prix en euros et centimes | `problem` |
| `ce2-problems-fractions` | 5 | Problèmes avec des fractions | Problèmes additifs avec des fractions de même dénominateur (gâteau : 1/10, 3/10 et 2/10 mangés, quelle part reste ?) ; réponse fraction, toute fraction égale acceptée | `problem` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Léo a 125 billes, il en gagne 38 puis en perd 20. Combien en a-t-il ?
- Une trottinette coûte 4 fois le prix d'un casque à 32 €. Combien coûte la trottinette ?
- Trois amis mangent 1/10, 3/10 et 2/10 d'un gâteau. Quelle part du gâteau reste-t-il ? → 4/10.
- 3 casques et 4 combinaisons : combien de tenues différentes ?

## Évolutions du moteur et de l'écran de série

- Format `problem` et gabarits créés au lot 1 (fiche « Exercices CE1 — Résoudre des problèmes ») ; nouveaux gabarits : comparaison multiplicative, produits cartésiens, problèmes en trois étapes, prix en euros et centimes (calcul en centimes, comme `ce2-money-add-sub`), fractions de même dénominateur (réponse fraction) ; réponse avec unité affichée.
- Nouveau type de problème `fraction` dans `problems.kinds` (schéma) et gabarits dans `content/problems/`.
- Les problèmes de mesures (longueurs, masses, durées, monnaie) sont portés par les fiches de grandeurs et mesures du lot.

## Distracteurs (QCM)

Mauvaise opération (addition à la place de soustraction), une seule étape faite, résultat intermédiaire donné comme réponse.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [ ] La compétence est liée à la fiche « Leçon CE2 — Résoudre des problèmes » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Fiches existantes reprises

SUB001606 (Nouveaux types d'exercices) : gabarits de problèmes du CE2.

## Remarques et décisions

- Prénoms variés et non stéréotypés, contextes du quotidien de l'enfant ; liste relue par Georges.
