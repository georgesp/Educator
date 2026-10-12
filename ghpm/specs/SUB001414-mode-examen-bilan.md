---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WfI
rev: 2026-10-11T16:02:55Z
url: https://github.com/georgesp/Educator/issues/30
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001414 — Mode examen (bilan)
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#14"
order: 470
---

## User story

En tant qu'enfant, je veux passer un petit examen afin de voir ce que je sais faire.

## Comportement attendu

- Menu **Examen** → choix de la matière ; le profil actif est utilisé.
- 15 questions mélangeant les compétences de la classe **jusqu'à la période en cours** (pondération plus forte sur les plus récentes), types d'exercices variés.
- Sans chrono.
- Résultat : score, **points forts** et **notions à retravailler** (par compétence), avec un lien direct vers la leçon et les exercices correspondants.
- `SkillMastery` mise à jour avec les résultats.
- Examen réussi sans aucune faute : feu d'artifice, avec le composant `Fireworks` (`web/src/components/child/`) créé par SUB000114.

## Critères d'acceptation

- [ ] Algorithme de composition de l'examen testé (répartition des compétences, pas de doublon).
- [ ] Examen enregistré (session `exam`).
