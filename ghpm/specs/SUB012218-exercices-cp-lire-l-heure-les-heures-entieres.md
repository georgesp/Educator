---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdp8A
url: https://github.com/georgesp/Educator/issues/328
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012218 — Exercices CP — Lire l'heure : les heures entières"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2077
---

Source : décision de Georges du 2026-10-16 (grandeurs et mesures), étendue au CP par la revue du 2026-10-17.

## User story

En tant qu'enfant de CP, je veux m'entraîner à lire l'heure sur une horloge à aiguilles et à placer les aiguilles, afin
de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CP, repérage dans le temps : lire sur une horloge à aiguilles une heure donnée en heures
  entières ; positionner les aiguilles d'une horloge pour une heure donnée (uniquement des heures entières inférieures ou
  égales à douze) ; associer une heure à un moment de la journée (en lien avec « Questionner le monde »).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cp-clock-read` | 3 | Lire l'heure | Horloge à aiguilles sur une heure entière de 1 h à 12 h : quelle heure est-il ? Relier une horloge à l'heure écrite (« 3 h ») ; associer une heure à un moment de la journée (« On va à l'école à 8 h du matin ou à 8 h du soir ? », avec image) | `measureRead`, `clockMatch`, `multiChoice` |
| `cp-clock-set` | 4 | Placer les aiguilles | Heure entière de 1 h à 12 h écrite (« 5 h ») ou lue au clic : l'enfant place la petite aiguille, la grande est sur le 12 (puis l'enfant place les deux) | `clockSet` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Quelle heure est-il ? (petite aiguille sur le 3, grande sur le 12) → 3 h.
- Place les aiguilles sur 9 h.
- On se couche à 8 h du soir ou à 8 h du matin ? → du soir.

## Évolutions du moteur et de l'écran de série

- Composant `clock`, formats `clockSet` et `clockMatch` (fiche « Exercices CE2 — Lire l'heure et calculer des durées »,
  SUB011937) dans une version CP : heures entières seulement, petite et grande aiguilles bien distinctes (longueur,
  épaisseur, couleur du thème) ; la grande aiguille peut être fixée sur le 12.
- Voix au clic : « 3 heures », jamais « 3 h 00 ».
- Images des moments de la journée (lever, école, repas, coucher) : dessins originaux dans la charte.
- Si le lot 7 passe avant le lot 4, cette fiche porte ces évolutions (voir « Ordre de réalisation » dans la fiche du lot).

## Distracteurs (QCM)

Petite et grande aiguilles inversées (12 h au lieu de 3 h) ; heure voisine (2 h ou 4 h) ; matin et soir confondus.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md` (nouveau fichier ; les 11 compétences du lot 1 restent dans `echantillons-lot1.md`), relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement (décision du 2026-10-15).
- [ ] Exercices jouables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [ ] Les compétences sont liées à la fiche « Leçon CP — Lire l'heure : les heures entières » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Résumé par niveau du programme de mathématiques du cycle 2 (2024) — attendus de grandeurs et mesures du CP, à vérifier sur le PDF officiel](https://maitresselunicole.fr/wp-content/uploads/2025/03/resume-par-niveaux-programme-de-mathematiques-cycle-2-2024-1.pdf)

## Remarques et décisions

- Demi-heures, quarts et minutes hors du CP : le programme se limite aux heures entières.
- Attendus à vérifier sur le PDF officiel au moment de rédiger les fichiers JSON.
