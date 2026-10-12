---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAvpk
rev: 2026-10-12T19:32:54Z
url: https://github.com/georgesp/Educator/issues/72
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: US0072 — Lot 1bis — Migration du backend .NET vers Rust
status: Done
size: XL
estimate:
startDate:
endDate:
parent:
order: 100
---

Source : étude de faisabilité d'une version tablette (2026-10-11).

- **Lot** : 1bis — Migration du backend .NET vers Rust
- **Prérequis** : fiche « Lot 1 — Socle mathématiques ».
- **Lot suivant** : fiche « Lot 2 — Orthographe, leçons et examens ».

## Contexte

Une version tablette (iPad, Android) est envisagée dans un lot ultérieur. Sur iOS, une application ne peut pas lancer de processus annexe : le host .NET lancé par Tauri (`educator-host`) ne peut donc pas être embarqué tel quel. Toute la logique passe dans le cœur Rust de Tauri, ce qui rend l'application autonome sur toutes les plateformes, sans serveur ni processus annexe.

La migration est faite maintenant, tant que le code se limite aux lots 0 et 1 (~4 600 lignes de C#, ~2 500 lignes de tests). Chaque fiche du lot 2 écrite en C# l'alourdirait.

## Ce qui ne change pas

- Le front React + TypeScript et ses écrans : seule l'implémentation de `UiApi` (`web/src/api/ui.ts`) change.
- Le contenu pédagogique (`content/`) et ses schémas JSON.
- Le comportement vu par l'enfant : mêmes écrans, mêmes exercices, mêmes points, même voix.

## Architecture cible

| Crate | Rôle | Dépend de |
|---|---|---|
| `educator-core` | Domaine et moteur d'exercices, code pur et testable | rien (hors `serde`, `rand`) |
| `educator-content` | Chargement du contenu (règles, personnages, calendrier, points), Piper sur bureau | `core` |
| `educator-data` | SQLite (`rusqlite`), migrations, profils et réglages | `core` |
| Application Tauri | Commandes Tauri, état partagé, démarrage, logs | tous |

Même règle de dépendance que l'ADR 0001 : `educator-core` ne dépend d'aucune crate d'infrastructure.

## Hors périmètre

- La version tablette elle-même (adaptation tactile, build iOS/Android, voix sur mobile) : lot dédié ultérieur.
- Toute nouvelle fonctionnalité.

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] `npm run tauri dev` lance l'application sur macOS sans .NET installé (vérifié sous Linux/Xvfb ; reste macOS).
- [ ] Le parcours du lot 1 (création de profil, série d'addition clavier et QCM, points, voix) fonctionne comme avant (vérifié sous Linux sans la voix ; reste macOS avec Piper).
- [x] Plus aucun projet .NET dans le dépôt ; pipeline CI vert sur macOS et Windows (`cargo test` + front).

## Notes de réalisation

2026-10-11 — fiches 01 à 05 et 07 réalisées, 06 aussi (code .NET supprimé), sauf la CI à voir verte sur GitHub. Vérifié dans l'environnement Linux de Claude : `cargo fmt --check`, `cargo clippy --workspace --all-targets
-D warnings` et `cargo test --workspace` verts (92 tests), Vitest (15) et Playwright (2) verts, application Tauri
lancée sous Xvfb : création de profil, choix du profil, liste des compétences, série au clavier, base conservée entre
deux lancements. Décision sur la base existante : reprise (ADR 0003).
