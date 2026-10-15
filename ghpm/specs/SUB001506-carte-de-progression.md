---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WpY
rev: 2026-10-10T19:39:29Z
url: https://github.com/georgesp/Educator/issues/39
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001506 — Carte de progression
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#15"
order: 616
---

## User story

En tant qu'enfant, je veux voir mon parcours comme une carte de jeu vidéo afin de savoir où j'en suis et ce qui m'attend.

## Comportement attendu

- Carte par matière et par classe : chaque compétence est une étape (avec ses étoiles), reliées dans l'ordre des périodes.
- Étape de la période en cours mise en avant ; les étapes futures sont visibles mais marquées « À venir », pas bloquées.
- Clic sur une étape → leçon ou exercices.

## Critères d'acceptation

- [x] Carte générée à partir des compétences (pas de dessin manuel par classe).
- [x] Lisible en 1024 × 700.

## Réalisation

- Écran **Ma carte** (`/carte/maths`, `/carte/orthographe`), depuis le menu.
- Le parcours est construit à partir des compétences de la classe (`get_skills`, révisions exclues) : 6 étapes par ligne, une ligne sur deux de droite à gauche, reliées par une route en pointillés ; une pastille « Période N » au début de chaque période.
- Chaque étape : la couleur et le symbole de sa famille (SUB001514), son titre, ses étoiles. Étapes de la période en cours entourées ; étapes futures atténuées et marquées « À venir », mais cliquables.
- Clic sur une étape : son titre, ses étoiles, sa maîtrise, et les boutons **La leçon** (quand une leçon a cette compétence pour principale : `LessonSummaryResponse.skillId`) et **Les exercices**.
- Mise en page vérifiée en 1024 × 700 (parcours CP de 13 étapes).
