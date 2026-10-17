---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoYI
url: https://github.com/georgesp/Educator/issues/299
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011942 — Exercices CE2 — Payer et rendre la monnaie
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1247
---

Source : décision de Georges du 2026-10-16 — ajouter les grandeurs et mesures au lot 4 (CE2). Les additions et
soustractions posées de prix restent dans la fiche « Exercices CE2 — Additions et soustractions posées jusqu'à 10 000 »
(`ce2-money-add-sub`) ; les problèmes de mesures sont ici.

## User story

En tant qu'enfant de CE2, je veux m'entraîner à composer une somme, à rendre la monnaie et à résoudre des problèmes de
mesures afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2, monnaie : simuler des achats et rendre la monnaie (3,68 € payés avec un billet de 5 €) ;
  poser des additions et des soustractions de montants en euros (4,56 € + 15,30 € ; 74,56 € − 15,30 €).
- Résoudre des problèmes mettant en jeu des longueurs, des masses, des contenances, des durées et des prix.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-money-make` | 2 | Euros et centimes | 1 € = 100 c ; écrire une somme montrée en pièces et billets (3 € 68 c = 3,68 €) ; composer une somme avec des pièces et des billets (glisser), au moins une fois avec le moins de pièces possible | `measureRead`, `coinsMake` |
| `ce2-money-change` | 4 | Rendre la monnaie | Prix ≤ 50 € (centimes compris) payé avec un billet de 5, 10, 20 ou 50 € : monnaie à rendre, en comptant « jusqu'à » (3,68 € → 3,70 € → 4 € → 5 €) ; composer la monnaie avec des pièces | `result`, `coinsMake` |
| `ce2-measure-problems` | 5 | Problèmes de mesures | Gabarits en une ou deux étapes : longueurs (5 km + 750 m), masses, contenances, durées, prix ; réponse avec unité, en plusieurs cases si besoin | `problem` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Combien d'argent dans le porte-monnaie ? (2 billets de 5 €, 3 pièces de 20 c) → 10,60 €.
- Fais 2,35 € avec le moins de pièces possible.
- Léa paie 3,68 € avec un billet de 5 €. Combien lui rend-on ? → 1,32 €.
- Un ruban de 2 m : on coupe 75 cm. Quelle longueur reste-t-il ? → 1 m 25 cm (ou 125 cm).

## Évolutions du moteur et de l'écran de série

- Type « mesure » (grandeur `money` : centimes entiers, comme `ce2-money-add-sub`) et réponse en plusieurs cases : fiche
  « Exercices CE2 — Longueurs ». Saisie d'un prix avec la virgule (3,68) ou en deux cases (3 € 68 c), les deux acceptées.
- Composant `coins` partagé avec la leçon (fiche « Extensions des animations et du quiz des leçons du CE2 ») : pièces de
  1 c à 2 € et billets de 5 € à 50 €, **dessins originaux** (pas de reproduction des pièces et billets réels) ; format
  `coinsMake` : glisser des pièces dans le porte-monnaie, la somme se met à jour.
- Gabarits de problèmes de mesures dans `content/problems/measures.json` (schéma `problem-templates.schema.json` étendu
  aux mesures), même règle de prénoms que les autres problèmes.

## Distracteurs (QCM)

Centimes soustraits sans la retenue (5 − 3,68 = 2,32) ; 1 € = 10 c ; unités mélangées dans un problème (2 m − 75 cm = 73).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges (prénoms et contextes compris).
- [ ] Consignes, prix et unités lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement.
- [ ] Pièces et billets utilisables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [ ] Les compétences sont liées à la fiche « Leçon CE2 — Payer et rendre la monnaie » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Remarques et décisions

- Attendus repris de résumés du programme par des enseignants : à vérifier sur le PDF officiel en rédigeant les fichiers JSON.
