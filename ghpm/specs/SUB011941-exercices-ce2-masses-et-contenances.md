---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoWE
url: https://github.com/georgesp/Educator/issues/298
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011941 — Exercices CE2 — Masses et contenances
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1243
---

Source : décision de Georges du 2026-10-16 — ajouter les grandeurs et mesures au lot 4 (CE2).

## User story

En tant qu'enfant de CE2, je veux m'entraîner à lire, comparer et convertir des masses et des contenances afin de suivre
le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2, masses : unités g, kg et t et leurs relations (1 kg = 1 000 g ; 1 t = 1 000 kg ;
  5 462 g = 5 kg 462 g) ; choisir l'unité adaptée ; comparer ; masses de référence ; estimer.
- Contenances : comparer ; unités L, dL et cL (1 L = 10 dL = 100 cL ; 780 cL = 7 L 80 cL).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-mass` | 4 | Les masses | Relations g, kg, t : conversions (3 kg = … g), décomposition (5 462 g = … kg … g), comparaison (1 kg 200 g … 1 500 g) ; lecture d'une balance (masses marquées de 1 g à 1 kg, aiguille graduée) ; choix de l'unité et estimation (banque d'objets de référence) | `measureRead`, `convert`, `compare`, `estimate` |
| `ce2-capacity` | 5 | Les contenances | Relations L, dL, cL : conversions (2 L = … cL), décomposition (780 cL = … L … cL), comparaison ; lecture d'un récipient gradué ; choix de l'unité et estimation | `measureRead`, `convert`, `compare`, `estimate` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- La balance est en équilibre avec 500 g + 200 g + 50 g. Combien pèse le melon ? → 750 g.
- 5 462 g = … kg … g → 5 kg 462 g.
- Une pomme pèse : 150 g, 150 kg ou 15 t ? → 150 g.
- 780 cL = … L … cL → 7 L 80 cL.
- Un verre d'eau contient : 20 cL, 20 L ou 2 L ? → 20 cL.

## Évolutions du moteur et de l'écran de série

- Type « mesure », formats et voix des unités : fiche « Exercices CE2 — Longueurs ».
- Composants `scale` (balance) et `measuringJug` (récipient gradué), partagés avec la leçon (fiche « Extensions des
  animations et du quiz des leçons du CE2 »).

## Distracteurs (QCM)

1 kg = 100 g ; décomposition décalée (5 kg 46 g) ; masses marquées comptées sans leur valeur ; 1 L = 10 cL.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes, nombres et unités lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [ ] Les compétences sont liées à la fiche « Leçon CE2 — Masses et contenances » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Remarques et décisions

- Attendus repris de résumés du programme par des enseignants : à vérifier sur le PDF officiel en rédigeant les fichiers JSON.
