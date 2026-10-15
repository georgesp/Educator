---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5WkA
rev: 2026-10-10T19:39:24Z
url: https://github.com/georgesp/Educator/issues/34
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001501 — Niveaux du profil
status: In Review
size: S
estimate:
startDate:
endDate:
parent: "#15"
order: 611
---

## User story

En tant qu'enfant, je veux monter de niveau en gagnant des points afin de voir que je progresse.

## Comportement attendu

- Niveau calculé à partir des points cumulés, barème progressif défini dans `content/rewards/levels.json` (ex. niveau 2 à 100 points, niveau 3 à 250…).
- Jauge de progression vers le niveau suivant dans le menu.
- Animation de montée de niveau, avec annonce des nouveautés débloquées (accessoires).

## Critères d'acceptation

- [x] Barème en fichier de contenu.
- [x] Niveau recalculé correctement pour les profils existants à la migration.

## Réalisation

- Barème dans `content/rewards/levels.json` (schéma `levels.schema.json`) : niveau 2 à 100 points, 3 à 250, 4 à 450, 5 à 700, 6 à 1 000… jusqu'au niveau 20 à 21 000 points, puis un niveau tous les 3 000 points (le barème ne s'arrête jamais). Un fichier invalide est signalé et le barème par défaut s'applique.
- Le niveau **n'est jamais stocké** : `educator-core::rewards::levels::LevelScale` le calcule à partir des points, qui ne baissent jamais. Un profil existant a donc tout de suite le bon niveau, sans migration, et les accessoires de son niveau sont débloqués à sa première ouverture (`RewardsService::overview`).
- Menu : jauge « Niveau N · encore X points » sous les points (commande `get_profile_overview`).
- Fin de série : « Tu passes au niveau N ! » animé, avec les accessoires débloqués (« Nouveau pour ton avatar : … »).
