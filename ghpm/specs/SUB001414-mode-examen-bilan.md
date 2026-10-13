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
status: Done
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

- [x] Algorithme de composition de l'examen testé (répartition des compétences, pas de doublon).
- [x] Examen enregistré (session `exam`).

## Implémentation (2026-10-14)

- Menu Examen → matière ; 15 questions des compétences de la classe jusqu'à la période en cours (`exam.rs`) :
  poids 3 pour la période en cours, 2 pour la précédente, 1 avant ; au plus 3 questions par compétence ;
  modes de réponse mélangés ; jamais deux fois la même question.
- Bilan : score, points forts et notions à retravailler par compétence, avec liens vers la leçon et les exercices ;
  feu d'artifice (`Fireworks`) en cas de sans-faute ; maîtrises mises à jour.
- Tests : `exam.rs`, `content_validation.rs` (800 examens composés), `ExamScreen.test.tsx`, Playwright.
