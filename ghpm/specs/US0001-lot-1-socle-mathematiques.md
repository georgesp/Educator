---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5RnI
rev: 2026-10-10T19:28:52Z
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
order: 10
---

Source : `2026-10-10_notes-application-educative.docx` (cadrage, version 4), section 13.

- **Lot** : 1 — Socle mathématiques
- **Prérequis** : fiche « Lot 0 — Socle technique » (coque Tauri 2, host .NET 10, API locale `/api/ui`, SQLite / EF Core, intégration Piper, installeurs).
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
- Multiplication, division, CE2 → CM2 (lot 4).

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] Parcours complet testé sur macOS et Windows : création de profil → menu → notion → série de 10 exercices → points affichés.
- [ ] Un enfant de CP peut réaliser le parcours sans aide (consignes lues, gros boutons, pavé numérique).

## Implémentation (2026-10-11)

Toutes les sous-fiches sont implémentées et testées dans l'environnement Linux de Claude :

- 263 tests .NET, 15 tests Vitest et 2 parcours Playwright, tous verts ;
- build .NET Release sans avertissement ;
- `npm run build` sans erreur.

Reste à faire par Georges : le parcours complet sur macOS avec la vraie voix (`npm run tauri dev`), les délais de lecture, la relecture des personnages et des échantillons d'exercices, puis Windows plus tard.
