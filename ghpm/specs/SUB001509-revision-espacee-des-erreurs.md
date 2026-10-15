---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wts
rev: 2026-10-10T19:39:33Z
url: https://github.com/georgesp/Educator/issues/42
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001509 — Révision espacée des erreurs
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#15"
order: 619
---

## User story

En tant qu'enfant, je veux revoir plus tard les calculs et les mots que j'ai ratés afin de ne plus me tromper.

## Comportement attendu

- Chaque erreur alimente une file de révision par profil (exercice ou mot), avec une prochaine date de révision (J+1, J+3, J+7, J+14).
- Série **« Mes révisions »** proposée quand des éléments sont à revoir ; les éléments réussis s'espacent, les éléments ratés reviennent plus tôt.
- Pour les maths, on revoit la même opération ou une opération proche (même paramètres).

## Critères d'acceptation

- [x] Table de révision + migration.
- [x] Algorithme testé avec un `TimeProvider` simulé.

## Réalisation

- Intervalles dans `content/progress/reviews.json` : J+1, J+3, J+7, J+14 ; 10 exercices au plus par série.
- Chaque erreur (séries, quiz, bilans, défis ; pas l'évaluation de niveau) ajoute l'élément à la file du profil (table `ReviewItem`, migration `0004_motivation`) ou le remet à J+1 s'il y est déjà : une même erreur n'est qu'un élément (clé de l'exercice).
- **Mes révisions** (`/revisions/maths`, `/revisions/orthographe`) : proposé sur l'écran Exercices quand des éléments sont dus (« 4 calculs ou mots à revoir aujourd'hui »). Réussi, l'élément passe à l'intervalle suivant, puis quitte la file après le dernier ; raté, il revient le lendemain.
- Maths : la même opération quand elle peut être reconstruite (même compétence, même graine, même place dans la série) ; sinon une opération de la même compétence (mêmes paramètres). Orthographe : le même mot. Sans voix, un mot n'est jamais redemandé en dictée.
- Séance enregistrée avec le type `Review` (points comme une série, pas d'étoiles) ; nombre d'éléments dus dans `get_profile_overview`.
- Tests avec l'horloge simulée (`ManualClock`) : `crates/educator-core/tests/reviews.rs`, et de bout en bout avec les mots d'orthographe dans `services.rs`.
