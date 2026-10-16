---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5RnI
rev: 2026-10-16T19:38:08Z
url: https://github.com/georgesp/Educator/issues/1
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: US0001 — Lot 1 — Socle mathématiques
status: In progress
size: XL
estimate:
startDate:
endDate:
parent:
order: 13
---

Source : `2026-10-10_notes-application-educative.docx` (cadrage, version 4), section 13.

- **Lot** : 1 — Socle mathématiques
- **Prérequis** : fiche « Lot 0 — Socle technique » (application Tauri 2 au cœur Rust, SQLite, intégration Piper, CI).
- **Lot suivant** : fiche « Lot 2 — Orthographe, leçons et examens ».

## Objectif

Première version utilisable par un enfant de CP ou CE1 : il choisit ou crée son profil, arrive sur le menu, choisit une notion d'addition ou de soustraction adaptée à sa classe et enchaîne une série d'exercices générés en direct, en tapant la réponse ou en choisissant parmi plusieurs propositions. Il gagne des points.

## Périmètre

- Personnages SVG des profils (lion, ours, lapin, renard, cochon, enfants stylisés).
- Profils multiples, accès libre, image attribuée au hasard.
- Écran de choix du profil à l'ouverture, menu d'accueil.
- Format des règles de génération + moteur de génération + validation des réponses.
- Règles CP–CE1 pour l'addition et la soustraction, alignées sur les repères annuels de progression.
- Écran de série d'exercices (clavier et QCM), consignes lues par Piper.
- Points.

## Hors périmètre (lots suivants)

- Orthographe, leçons, examens, chrono, enregistrement détaillé des statistiques (lot 2).
- Niveaux, étoiles, badges, accessoires d'avatar, difficulté adaptative, espace parent (lot 3).
- Multiplication, division, CE2 → CM2 (lots 4, 5 et 6, un par niveau).

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] Parcours complet testé sur macOS et Windows : création de profil → menu → notion → série de 10 exercices → points affichés.
- [ ] Un enfant de CP peut réaliser le parcours sans aide (consignes lues, gros boutons, pavé numérique).

## Implémentation (2026-10-11)

Toutes les sous-fiches sont implémentées et testées dans l'environnement Linux de Claude :

- version Rust (lot 1bis), avec les retours d'essai (SUB000112 à SUB000116) : 107 tests Rust, `clippy -D warnings`
  propre, 27 tests Vitest et 2 parcours Playwright verts ;
- `npm run build` sans erreur ;
- parcours lancé dans la vraie application Tauri (Linux, Xvfb) : création de profil, compétences, série au clavier.

Reste à faire par Georges : le parcours complet sur macOS avec la vraie voix (`npm run tauri dev`), les délais de lecture (SUB000109, SUB000111), la relecture des personnages et des échantillons d'exercices régénérés (SUB000112), puis Windows plus tard.

## Historique

Le lot 1 a d'abord été réalisé en .NET (263 tests), puis porté en Rust par le lot 1bis (`specs/lot-migration-rust.md`,
ADR 0003). Les noms de types cités dans les sous-fiches sont ceux de la version Rust.
