---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXvMY
url: https://github.com/georgesp/Educator/issues/323
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012044 — Leçon CM1 — Lire l'heure et calculer des durées
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 5200
---

## User story

En tant qu'enfant de CM1, je veux une leçon courte et animée qui m'explique comment lire l'heure et calculer une durée, avant de m'entraîner.

## Leçon

- **Classe / période** : CM1, période 1.
- **Compétences liées** (`skillIds`) : `cm1-time-read`, `cm1-time-duration` — exercices : la fiche « Exercices CM1 — Lire l'heure et calculer des durées ».
- **Objectif** : lire l'heure et placer les aiguilles ; calculer une durée en faisant des bonds jusqu'à l'heure ronde.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « heure », « minute », « durée », « horaire », « aiguille ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Lire l'heure et calculer des durées » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Deux aiguilles** — La petite aiguille montre les heures, la grande les minutes. | `clockFace` |
| 3 | **Les minutes** — Le tour du cadran fait 60 minutes : de 5 en 5, 5, 10, 15… | `clockFace` (graduations) |
| 4 | **Lire 3 h 40** — La petite aiguille a dépassé le 3, la grande est sur le 8 : 3 h 40. | `clockFace` |
| 5 | **À toi** — Interaction : place les aiguilles sur 7 h 25. | `clockFace` (interactif) |
| 6 | **Matin ou après-midi** — 3 h 40 de l'après-midi s'écrit 15 h 40 : la journée sur une piste de 0 h à 24 h. | `numberLine` (heures) |
| 7 | **Une durée** — De 9 h 45 à 11 h 20 : un bond de 15 min jusqu'à 10 h, un bond de 1 h jusqu'à 11 h, un bond de 20 min. | `numberLine` (bonds) |
| 8 | **Ajouter les bonds** — 15 min + 1 h + 20 min = 1 h 35 min. | `calcChain` |
| 9 | **À toi** — Interaction : le Grand Prix part à 14 h 50 et dure 1 h 45 min ; à quelle heure finit-il ? | `numberLine` (interactif) |
| 10 | **Je retiens** — « La petite aiguille donne l'heure, la grande les minutes. Pour une durée, je fais des bonds jusqu'à l'heure ronde. 1 h = 60 min. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `calcChain`, `clockFace`, `numberLine`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » ; `clockFace` est ajoutée par la fiche « Animations des leçons CM1 — heures, mesures, probabilités et données ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Quelle heure indique l'horloge ? (QCM : 3 h 40, 8 h 15, 4 h 40)
2. Place les aiguilles sur 7 h 25.
3. De 9 h 45 à 11 h 20 : quelle durée ?
4. Combien de minutes dans 1 h 15 min ?

Chaque question a une explication courte, affichée et lue après la réponse.

## Sources

- [Programme de mathématiques du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025 ; CM1 à la rentrée 2025, CM2 à la rentrée 2026)](https://www.education.gouv.fr/sites/default/files/programme-de-math-matiques-pour-le-cycle-3-439827.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cm1-lire-l-heure-durees.json`, validée par `lesson.schema.json`, `skillIds` = `cm1-time-read`, `cm1-time-duration`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris.
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 3, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » et fiche « Animations des leçons CM1 — heures, mesures, probabilités et données » (`clockFace`).
- Fiche « Exercices CM1 — Lire l'heure et calculer des durées » : composant horloge partagé.
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
