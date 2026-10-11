---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Rp0
url: https://github.com/georgesp/Educator/issues/3
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB000102 — Profils — modèle, stockage et API"
status: In progress
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

Entité `Profile` (EF Core, table `Profile`) :

| Champ | Type | Remarque |
|---|---|---|
| `Id` | Guid v7 | clé |
| `Name` | texte | 1 à 20 caractères, unique (insensible à la casse) |
| `Grade` | enum en texte | `CP`, `CE1`, `CE2`, `CM1`, `CM2` |
| `CharacterId` | texte | id de `characters.json` ; deviendra la configuration d'avatar au lot 3 |
| `Points` | entier | ≥ 0 |
| `CreatedAt`, `UpdatedAt`, `LastUsedAt` | ms Unix UTC | via intercepteur + `TimeProvider` |

## API locale

- `GET /api/ui/profiles` (trié par `LastUsedAt` décroissant)
- `POST /api/ui/profiles` `{ name, grade }` → personnage aléatoire
- `PATCH /api/ui/profiles/{id}` `{ name?, grade?, characterId? }`
- `POST /api/ui/profiles/{id}/reroll-character` → nouveau personnage aléatoire, différent du précédent
- `POST /api/ui/profiles/{id}/select` → met à jour `LastUsedAt`
- `DELETE /api/ui/profiles/{id}`

## Critères d'acceptation

- [x] Migration EF Core `AddProfile` appliquée au démarrage.
- [x] Validation côté host : nom vide, trop long ou en double → `400` avec message en français.
- [x] Tests unitaires du service (création, tirage aléatoire, unicité du nom) et tests d'intégration de l'API.
- [x] Contrat OpenAPI à jour, client TypeScript régénéré.

## Notes techniques

- Le tirage aléatoire passe par une abstraction (`IRandomSource`) injectable pour les tests.
- Mêmes règles de stockage que l'ADR 0001 de GitHubProjectManager (dates en ms UTC, enums en texte, `IDbContextFactory`).

## Implémentation (2026-10-11)

- Core : `Profile`, `IProfileStore`, `ProfileService` (validation, tirage via `IRandomSource`, nouveau tirage toujours différent).
- Data : table `Profile`, migration `AddProfile`, index unique sur `NormalizedName` (prénom en majuscules). Les points ne changent que par `AddPointsAsync`, une mise à jour atomique.
- Messages d'erreur : « Écris ton prénom. », « Ton prénom est trop long : 20 lettres au maximum. », « Ce prénom est déjà pris. Choisis-en un autre. ». Ils arrivent dans `detail` d'un problème 400, avec `field`.
- `GET /api/ui/characters/draw?except=` tire le personnage montré à l'écran de création. `POST /api/ui/profiles` accepte ce `characterId`, ce qui garde le tirage côté host.
- Tests : `ProfileServiceTests` (unitaires), `SqliteProfileStoreTests` (SQLite réelle), `ProfileEndpointsTests` (API).
