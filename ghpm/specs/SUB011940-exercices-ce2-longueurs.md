---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoTk
url: https://github.com/georgesp/Educator/issues/297
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011940 — Exercices CE2 — Longueurs
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 1241
---

Source : décision de Georges du 2026-10-16 — ajouter les grandeurs et mesures au lot 4 (CE2). Cette fiche porte aussi les
évolutions du moteur communes à toutes les mesures (type « mesure », famille de couleur, voix des unités).

## User story

En tant qu'enfant de CE2, je veux m'entraîner à mesurer, à comparer et à convertir des longueurs, et à calculer un
périmètre, afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2, grandeurs et mesures : connaître et utiliser les unités m, dm, cm, mm et km et leurs
  relations (1 cm = 10 mm ; 6 km = 6 000 m ; 215 cm = 2 m 1 dm 5 cm ; 5 km + 750 m = 5 750 m).
- Choisir l'unité adaptée ; comparer des longueurs ; mesurer et tracer un segment à la règle graduée.
- Longueurs de référence, estimer une longueur ou une distance (une trousse mesure-t-elle 2 cm, 20 cm ou 2 m ?).
- Savoir ce qu'est le périmètre d'une figure plane ; déterminer un périmètre à la règle graduée.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-length-measure` | 1 | Mesurer avec la règle | Segment posé sur une règle graduée (début à 0 puis ailleurs) : longueur en cm, puis en cm et mm (7 cm 4 mm) ; segments de 1 à 15 cm | `measureRead` |
| `ce2-length-convert` | 2 | Unités de longueur | Relations m, dm, cm, mm, km : conversions simples (6 km = … m), décomposition (215 cm = … m … dm … cm), comparaison (1 m 5 cm … 150 cm), choix de l'unité et estimation (banque d'objets de référence) | `convert`, `compare`, `estimate` |
| `ce2-perimeter` | 4 | Le périmètre | Polygone (triangle, carré, rectangle, figure à 4 à 6 côtés) avec les longueurs écrites ou à mesurer sur la règle ; périmètre en cm (≤ 100) ou en m | `result` (avec figure), `measureRead` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Quelle est la longueur du segment ? → 7 cm 4 mm (ou 74 mm).
- 6 km = … m → 6 000.
- 215 cm = … m … dm … cm → 2 m 1 dm 5 cm.
- 1 m 5 cm … 150 cm → <.
- Une trousse mesure : 2 cm, 20 cm ou 2 m ? → 20 cm.
- Un rectangle a des côtés de 7 cm et 3 cm. Quel est son périmètre ? → 20 cm.

## Évolutions du moteur et de l'écran de série (communes aux mesures)

- Nouvelle opération `measure` (schéma, `educator-core`) avec un champ `measure` : grandeur (`length`, `mass`, `capacity`,
  `duration`, `time`, `money`), unités utilisées, plage, sortes de questions.
- **Type « mesure »** dans `educator-core` : une valeur entière dans la plus petite unité (mm, g, cL, min, centimes) ;
  conversion, comparaison et affichage en unités composées (2 m 1 dm 5 cm) ; aucune virgule de nombre décimal au CE2
  (sauf les prix, déjà gérés en centimes).
- **Réponse en plusieurs cases** (un nombre par unité, l'unité écrite à côté de chaque case), validée par l'égalité des
  valeurs : 74 mm et 7 cm 4 mm sont justes quand la consigne n'impose pas l'unité ; une unité imposée (« en mm ») doit être
  respectée.
- Nouveaux formats : `measureRead` (lire une mesure sur un instrument : règle, balance, récipient, horloge), `convert`
  (conversion et décomposition), `estimate` (choisir la mesure vraisemblable parmi trois) ; `compare` étendu aux mesures.
- Banque d'objets de référence pour estimer : `content/measures/references.json` (objet, grandeur, mesure, dessin ou mot ;
  schéma `references.schema.json`), relue par Georges.
- Composant `ruler` partagé avec les leçons (fiche « Extensions des animations et du quiz des leçons du CE2 ») ; figures
  de périmètre dessinées à l'échelle, longueurs écrites en clair.
- **Code couleur** (SUB001514) : nouvelle famille « Grandeurs et mesures » (symbole `cm`) pour l'opération `measure` ;
  sixième couleur ajoutée à chaque thème (`web/src/styles.css`, contraste AA vérifié par `web/src/themes.test.ts`).
- **Voix** : unités lues en entier (« 7 centimètres 4 millimètres », « 2 heures 20 minutes », « 5 kilogrammes »,
  « 3 euros 68 ») par une normalisation du texte avant Piper et avant la voix du système de l'iPad.

## Distracteurs (QCM)

Lecture de la règle à partir de 1 au lieu de 0 ; 10 au lieu de 100 ou 1 000 dans une conversion (6 km = 600 m) ;
comparaison des seuls nombres sans les unités (1 m 5 cm < 150 cm faux) ; périmètre confondu avec la somme de deux côtés.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur (2 000 graines par compétence), y compris les règles de variété de SUB000112 ; tests du type « mesure » (conversions aller-retour, comparaison, réponses équivalentes acceptées, unité imposée respectée).
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Consignes, nombres et unités lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [ ] Famille « Grandeurs et mesures » affichée sur les cartes et dans le filtre des écrans Exercices et Leçons.
- [ ] Les compétences sont liées à la fiche « Leçon CE2 — Longueurs » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Remarques et décisions

- Les attendus de grandeurs et mesures ci-dessus viennent de reprises du programme par des enseignants : les exemples
  et les éventuelles périodes sont à vérifier sur le PDF officiel au moment de rédiger les fichiers JSON.
- Géométrie (solides, figures, symétrie) et organisation et gestion de données (tableaux, diagrammes) restent hors du
  lot 4 : voir la fiche du lot.
