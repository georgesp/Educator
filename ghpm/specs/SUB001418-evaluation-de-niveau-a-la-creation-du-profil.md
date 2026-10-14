---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAzB4Q
url: https://github.com/georgesp/Educator/issues/266
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001418 — Évaluation de niveau à la création du profil
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#14"
order: 550
---

Source : demande de Georges du 2026-10-14 (évaluation d'entrée par matière, 20 s par question, chrono réglable par profil).

## User story

En tant que parent, je veux que l'enfant passe une évaluation dans chaque matière à la création de son profil, afin que l'application connaisse son vrai niveau (statistiques, défi chrono, leçons à revoir) dès le départ.

## Comportement attendu

- Après la création d'un profil, l'application propose l'**évaluation de niveau** : une par matière (maths, puis orthographe), au niveau de la classe du profil et **jusqu'à la période en cours**.
- Elle est aussi proposée dans le menu **Examen** tant qu'elle n'est pas **terminée** pour une matière.
- **2 ou 3 questions par compétence** de la classe, **30 questions au maximum** par matière. Quand la classe a plus de compétences que le plafond permet (ex. CE1 en maths), les compétences sont tirées au sort, les périodes récentes pesant davantage (même pondération que l'examen).
- Un **chrono par question** : 20 secondes par défaut. Quand le temps est écoulé, la question compte comme fausse, la bonne réponse s'affiche brièvement, puis la question suivante arrive (comme dans le défi chrono, sans pénalité de points). En orthographe, le chrono ne démarre qu'une fois le mot lu.
- **Valide seulement si terminée** : une évaluation abandonnée, ou interrompue par la fermeture de l'application, ne compte pas — ses réponses sont exclues des statistiques et de la maîtrise — et elle est reproposée en entier. Une fois terminée pour une matière, elle ne peut plus être refaite.
- **Alimente les statistiques** : chaque réponse est enregistrée (séance de type `Placement`) et met à jour la maîtrise (`SkillMastery`) de chaque compétence interrogée ; l'évaluation apparaît dans la liste des examens de l'espace parent.
- Bilan de fin identique à l'examen : score, points forts, notions à retravailler avec liens vers la leçon et les exercices.
- Pas de points gagnés (l'évaluation ne doit pas inciter à deviner).
- Si la classe n'a aucune compétence dans la matière (contenu CE2 à CM2 pas encore écrit), l'évaluation de cette matière n'est pas proposée.

## Réglage du chrono (options du profil)

Dans la fiche « Modifier le profil » :

- **Chrono de l'évaluation de niveau** : 15 / 20 (défaut) / 30 / 45 / 60 secondes par question.
- **Chrono de l'examen** : sans chrono (défaut) / 20 / 30 / 45 / 60 secondes par question. Même mécanique que l'évaluation quand il est activé.
- Réglages stockés dans la table `Profile` (migration `0003`), supprimés avec le profil.

## Critères d'acceptation

- [ ] Composition testée : 2 à 3 questions par compétence, 30 au maximum, aucun doublon, toutes les combinaisons classe × période × matière (`content_validation.rs`).
- [ ] Séance `Placement` enregistrée avec ses réponses ; maîtrise mise à jour à la fin.
- [ ] Une évaluation abandonnée n'apparaît ni dans les statistiques ni dans la maîtrise, et reste à refaire.
- [ ] Une évaluation terminée n'est plus proposée pour la matière.
- [ ] Chrono par question : temps écoulé = réponse fausse, question suivante automatique ; durée lue dans les réglages du profil.
- [ ] Réglages du chrono modifiables dans « Modifier le profil » et pris en compte à l'évaluation et à l'examen.
- [ ] Textes de l'enfant dans `fr.ts`, types TypeScript régénérés, tests Rust, Vitest et Playwright.
