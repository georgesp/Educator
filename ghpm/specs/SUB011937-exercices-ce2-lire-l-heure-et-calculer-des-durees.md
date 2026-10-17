---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoLg
url: https://github.com/georgesp/Educator/issues/294
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011937 — Exercices CE2 — Lire l'heure et calculer des durées
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 1245
---

Source : décision de Georges du 2026-10-16 — ajouter les grandeurs et mesures au lot 4 (CE2).

## User story

En tant qu'enfant de CE2, je veux m'entraîner à lire l'heure sur une horloge à aiguilles, à placer les aiguilles et à
calculer des durées afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE2, temps : lire l'heure sur une horloge à aiguilles ; placer les aiguilles en heures
  entières ou en heures et minutes.
- Comparer et mesurer des durées dans une même journée (2 h 20 min = 140 min) ; résoudre des problèmes de durées en une
  ou deux étapes.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce2-clock-read` | 1 | Lire l'heure | Horloge à aiguilles : heures entières, demies et quarts, puis minutes de 5 en 5, puis à la minute (`progression: easyFirst`) ; réponse « … h … min » ; matin ou après-midi (14 h 30 ou 2 h 30 de l'après-midi) quand l'énoncé le précise | `measureRead`, `clockMatch` |
| `ce2-clock-set` | 2 | Placer les aiguilles | Heure affichée en chiffres (8 h 45) ou dite par la voix au clic : l'enfant place les deux aiguilles (pas de 5 min, puis de 1 min) | `clockSet` |
| `ce2-durations` | 4 | Calculer des durées | Conversions h ↔ min (1 h = 60 min, 2 h 20 min = 140 min, 90 min = 1 h 30 min) ; durée entre deux heures d'une même journée (9 h 45 → 11 h 10) ; heure de fin (14 h 30 + 1 h 45) ; problèmes en une ou deux étapes | `convert`, `result`, `problem` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Quelle heure indique l'horloge ? → 4 h 35 min.
- Place les aiguilles sur 8 h 45.
- 2 h 20 min = … min → 140.
- Le film commence à 14 h 30 et dure 1 h 45. À quelle heure finit-il ? → 16 h 15.
- Le car part à 9 h 45 et arrive à 11 h 10. Combien de temps dure le trajet ? → 1 h 25 min.

## Évolutions du moteur et de l'écran de série

- Type « mesure » (grandeurs `time` pour une heure de la journée, `duration` pour une durée), réponse en plusieurs cases et
  voix des unités : fiche « Exercices CE2 — Longueurs ». Une heure va de 0 h 00 à 23 h 59 ; une durée reste dans la journée.
- Composant `clock` partagé avec la leçon (fiche « Extensions des animations et du quiz des leçons du CE2 ») : horloge à
  aiguilles lisible (chiffres de 1 à 12, graduations des minutes), petite aiguille placée entre deux heures selon les minutes.
- Format `clockSet` : les deux aiguilles se déplacent au doigt ou à la souris, la petite suit la grande ; la réponse est
  juste si l'heure lue correspond (la petite aiguille est tolérée entre les deux heures voisines). Réponse au clavier
  possible pour l'accessibilité.
- Format `clockMatch` : relier une horloge à son affichage numérique (QCM).
- Gabarits de problèmes de durées dans `content/problems/` (journée de l'enfant, course, trajet, cuisson), même règle de
  prénoms que les autres problèmes.

## Distracteurs (QCM)

Grande et petite aiguilles inversées ; minutes lues comme le chiffre montré (4 h 7 au lieu de 4 h 35) ; 1 h = 100 min ;
durée calculée comme une soustraction de nombres décimaux (11,10 − 9,45).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur (2 000 graines par compétence) ; tests des heures et des durées (passage de l'heure, 60 minutes, jamais au-delà de 23 h 59).
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce2.md`, relu par Georges.
- [ ] Heures et durées lues correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement (« 4 heures 35 », « 1 heure 25 minutes »).
- [ ] Horloge utilisable au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [ ] Les compétences sont liées à la fiche « Leçon CE2 — Lire l'heure et calculer des durées » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Remarques et décisions

- Attendus repris de résumés du programme par des enseignants : à vérifier sur le PDF officiel en rédigeant les fichiers JSON.
- Les heures s'écrivent « 4 h 35 » (espace insécable de chaque côté de « h »), jamais « 4:35 ».
