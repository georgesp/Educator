---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Rp0
rev: 2026-10-11T07:56:30Z
url: https://github.com/georgesp/Educator/issues/3
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB000102 — Profils — modèle, stockage et API"
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 30
---

## User story

En tant que parent, je veux que plusieurs enfants aient chacun leur profil sur le même ordinateur afin que leurs progrès ne se mélangent pas.

## Comportement attendu

- Un profil a : un nom, une classe (CP, CE1, CE2, CM1, CM2), un personnage, des points.
- Plusieurs profils par ordinateur, **sans mot de passe** : tous les profils sont accessibles à tous.
- À la création, le personnage est **tiré au hasard** parmi `characters.json` ; il peut être tiré à nouveau.
- Modification du nom, de la classe et du personnage ; suppression d'un profil (avec confirmation côté interface, lot Options).

## Modèle

Table `Profile` (`educator-data`) :

| Champ | Type | Remarque |
|---|---|---|
| `Id` | Guid v7 | clé |
| `Name` | texte | 1 à 20 caractères, unique (insensible à la casse) |
| `Grade` | enum en texte | `CP`, `CE1`, `CE2`, `CM1`, `CM2` |
| `CharacterId` | texte | id de `characters.json` ; deviendra la configuration d'avatar au lot 3 |
| `Points` | entier | ≥ 0 |
| `CreatedAt`, `UpdatedAt`, `LastUsedAt` | ms Unix UTC | renseignés à partir de l'horloge injectée (`Clock`) |

## Commandes Tauri

- `list_profiles` (trié par `LastUsedAt` décroissant)
- `create_profile` `{ name, grade, characterId? }` → personnage aléatoire si absent
- `update_profile` `{ id, changes: { name?, grade?, characterId? } }`
- `reroll_character` → nouveau personnage aléatoire, différent du précédent
- `select_profile` → met à jour `LastUsedAt`
- `delete_profile`

## Critères d'acceptation

- [x] Migration créant la table `Profile` appliquée au démarrage.
- [x] Validation dans le cœur : nom vide, trop long ou en double → erreur `400` avec message en français.
- [x] Tests unitaires du service (création, tirage aléatoire, unicité du nom) et tests des commandes.
- [x] Contrats à jour, types TypeScript régénérés.

## Notes techniques

- Le tirage aléatoire passe par un trait (`RandomSource`) injectable pour les tests.
- Règles de stockage de l'ADR 0003 (dates en ms UTC, enums en texte, Guid v7).

## Implémentation (2026-10-11)

- `educator-core` : `Profile`, `ProfileStore`, `ProfileService` (validation, tirage via `RandomSource`, nouveau tirage toujours différent).
- `educator-data` : table `Profile`, index unique sur `NormalizedName` (prénom en majuscules). Les points ne changent que par `add_points`, une mise à jour atomique.
- Messages d'erreur : « Écris ton prénom. », « Ton prénom est trop long : 20 lettres au maximum. », « Ce prénom est déjà pris. Choisis-en un autre. ». Ils arrivent dans `detail` d'une erreur 400, avec `field`.
- `draw_character { except }` tire le personnage montré à l'écran de création. `create_profile` accepte ce `characterId`, ce qui garde le tirage dans le cœur.
- Tests : `educator-core/tests/profiles.rs` (unitaires), `educator-data/tests/database.rs` (SQLite réelle), tests de `services.rs` (commandes).
