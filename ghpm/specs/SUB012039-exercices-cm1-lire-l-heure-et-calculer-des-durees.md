---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXvBs
url: https://github.com/georgesp/Educator/issues/318
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012039 — Exercices CM1 — Lire l'heure et calculer des durées
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 5150
---

## User story

En tant qu'enfant de CM1, je veux m'entraîner à lire l'heure sur une horloge à aiguilles et à calculer des durées afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 3 (2025), CM1, durées : lire l'heure sur une horloge à aiguilles ; positionner les aiguilles d'une horloge correspondant à une heure donnée en heure et minute ; comparer et mesurer des durées en heures et minutes ; résoudre des problèmes de durées à une ou deux étapes.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cm1-time-read` | 1 | Lire l'heure et placer les aiguilles | Horloge à aiguilles : heures de 1 à 12, minutes de 5 en 5 puis de 1 en 1 ; matin ou après-midi (3 h 40 de l'après-midi = 15 h 40) | `clockRead`, `clockSet` |
| `cm1-time-duration` | 3 | Calculer des durées | Durée entre deux horaires d'une même journée, horaire de fin ou de début, durées inférieures à 12 h ; 1 h = 60 min (1 h 15 min = 75 min) ; problèmes à une ou deux étapes | `duration`, `problem` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Quelle heure indique l'horloge ? (petite aiguille entre 3 et 4, grande sur 8) → 3 h 40.
- Place les aiguilles sur 7 h 25.
- Départ 9 h 45, arrivée 11 h 20 : combien de temps a duré le trajet ? → 1 h 35 min.
- Le Grand Prix commence à 14 h 50 et dure 1 h 45 min. À quelle heure finit-il ? → 16 h 35.
- 1 h 15 min = … min → 75.

## Évolutions du moteur et de l'écran de série

- Composant horloge à aiguilles SVG (couleurs et formes par variables du thème) : lecture, et aiguilles déplaçables à la souris et au doigt (tablette) ; la petite aiguille avance avec les minutes.
- Type de réponse horaire / durée : deux champs (heures, minutes) ; minutes de 0 à 59, « 1 h 75 » refusé avec un retour bienveillant (« 75 minutes, c'est plus d'une heure ») ; propriété testée : début + durée = fin.
- Lecture Piper : « quinze heures quarante », « une heure trente-cinq minutes » (jamais « h » ni « min » lus lettre par lettre).

## Distracteurs (QCM)

Aiguilles inversées (8 h 15 pour 3 h 40), minutes lues comme le chiffre pointé (3 h 8), heure du chiffre suivant (4 h 40), durée calculée comme un nombre décimal (11 h 20 − 9 h 45 = 1 h 75), passage de l'heure oublié.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cm1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CM1 — Lire l'heure et calculer des durées » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
