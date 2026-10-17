---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdqDU
url: https://github.com/georgesp/Educator/issues/332
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012222 — Leçon CP — Lire l'heure : les heures entières"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2078
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'explique comment lire l'heure sur une horloge à aiguilles, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 3.
- **Compétences liées** (`skillIds`) : `cp-clock-read`, `cp-clock-set` — exercices : la fiche « Exercices CP — Lire l'heure : les heures entières ».
- **Objectif** : lire une heure entière sur une horloge à aiguilles, placer les aiguilles, associer une heure à un moment de la journée.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « heure », « horloge », « petite aiguille », « grande aiguille », « matin », « après-midi », « soir ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Lire l'heure » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **L'horloge** — 12 nombres et deux aiguilles : la petite montre l'heure, la grande montre les minutes. | `clock` |
| 3 | **L'heure pile** — La grande aiguille est sur le 12 ; la petite est sur le 3 : il est 3 heures. | `clock` |
| 4 | **À toi** — Interaction : quelle heure est-il ? (8 h) | `clock` (pick) |
| 5 | **La petite aiguille avance** — 1 h, 2 h, 3 h : la petite aiguille passe d'un nombre au suivant. | `clock` |
| 6 | **À toi** — Interaction : place la petite aiguille sur 5 h. | `clock` (interactif) |
| 7 | **Matin ou soir ?** — 8 h du matin pour aller à l'école, 8 h du soir pour se coucher : la même horloge. | `clock` |
| 8 | **Ma journée** — Lever à 7 h, école à 9 h, repas à 12 h : les moments se suivent. | `sequencePattern` (frise) |
| 9 | **À toi** — Interaction : relie chaque moment de la journée à son horloge. | `clock` (match) |
| 10 | **Je retiens** — « Quand la grande aiguille est sur le 12, la petite aiguille me dit l'heure. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `clock`, `recapCard`, `sequencePattern`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) » ; `clock` : fiche « Extensions des animations et du quiz des leçons du CE2 » (SUB011946).
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Quelle heure est-il ? (horloge sur 3 h)
2. Place les aiguilles sur 9 h.
3. Quelle aiguille montre l'heure ? (QCM : la petite, la grande)
4. On va à l'école à 8 h du matin ou à 8 h du soir ? (QCM)

Chaque question a une explication courte, affichée après la réponse et lisible au clic (rien n'est lu tout seul, décision du 2026-10-15).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Résumé par niveau du programme de mathématiques du cycle 2 (2024) — attendus de grandeurs et mesures du CP, à vérifier sur le PDF officiel](https://maitresselunicole.fr/wp-content/uploads/2025/03/resume-par-niveaux-programme-de-mathematiques-cycle-2-2024-1.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [ ] Leçon `content/lessons/math/cp-lire-l-heure.json`, validée par `lesson.schema.json`, `skillIds` = `cp-clock-read`, `cp-clock-set`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [ ] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [ ] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Fiche « Extensions des animations et du quiz des leçons du CE2 » (SUB011946) : `clock`.
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.
