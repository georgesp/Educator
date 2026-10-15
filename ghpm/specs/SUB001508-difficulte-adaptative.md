---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WsU
rev: 2026-10-10T19:39:32Z
url: https://github.com/georgesp/Educator/issues/41
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001508 — Difficulté adaptative
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#15"
order: 618
---

## User story

En tant qu'enfant, je veux des exercices ni trop faciles ni trop difficiles afin de progresser sans me décourager.

## Comportement attendu

- Niveau de départ : classe + période en cours.
- Ajustement selon la maîtrise (`SkillMastery`) : au-dessus de ~85 % de réussite récente, proposer la compétence suivante ou des paramètres plus difficiles ; en dessous de ~60 %, revenir à la compétence précédente.
- Proposition « Je te conseille… » sur l'écran Exercices, l'enfant garde le choix.
- Seuils en fichier de contenu.

## Critères d'acceptation

- [x] Algorithme testé sur des historiques simulés.
- [x] L'enfant n'est jamais bloqué : toutes les compétences restent accessibles.

## Réalisation

- Seuils dans `content/progress/adaptive.json` : 85 % pour passer à la suite, 60 % pour revenir en arrière, 10 réponses avant de bouger le conseil.
- `educator-core::adaptive::advise` : point de départ = compétences de la classe à la période en cours ; une compétence pas encore assez pratiquée est conseillée (« C'est le moment de découvrir », « Continue ») ; sous 60 %, la compétence d'avant dans la même famille (« Pour bien réussir « X », revois d'abord : ») ; au-dessus de 85 %, la compétence suivante de la période puis la première de la période suivante non maîtrisée (« Bravo, tu es prêt pour la suite »). Rien n'est conseillé quand tout est maîtrisé.
- Écran Exercices : carte « Je te conseille… » au-dessus des filtres (commande `get_advice`) ; l'enfant garde le choix, toutes les compétences restent dans la liste.
- Tests sur des historiques de réponses simulés : `crates/educator-core/tests/adaptive.rs`.
- Les « paramètres plus difficiles » d'une même compétence ne sont pas proposés : le conseil passe d'une compétence à l'autre (les compétences sont déjà graduées par période).
