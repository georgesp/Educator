---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5k6M
rev: 2026-10-11T15:22:48Z
url: https://github.com/georgesp/Educator/issues/69
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "US0069 — Lot packaging — Installeurs, signature Apple et tests Windows"
status: Todo
size: L
estimate:
startDate:
endDate:
parent:
order: 650
---

Lot sorti du lot 0 (socle technique) le 2026-10-10 : le packaging, la signature et la validation sur Windows
ne bloquent pas le développement des lots 1 à 4, qui se fait avec `npm run tauri dev` sur macOS.

- **Prérequis** : lot 0 terminé (US0060).
- **À trancher avant de démarrer** : compte Apple Developer (signature + notarisation) ou distribution d'un `.dmg`
  non signé ; machine Windows disponible pour les tests, ou CI GitHub Actions uniquement.

## Objectif

Qu'un parent puisse installer Educator comme n'importe quel logiciel, sur macOS et Windows, sans Node ni Rust,
et sans alerte de sécurité bloquante.

## Contenu

- Installeurs `.dmg` et `.msi` / NSIS, Piper et la voix embarqués (fiche SUB006008, rattachée à ce lot).
- Contenu pédagogique embarqué : le dossier `content/` est une ressource Tauri (`bundle.resources` de
  `tauri.conf.json`, lot 1bis) ; `find_content_directory` (`educator-content`) le cherche dans les ressources de
  l'application (ou dans `EDUCATOR_CONTENT_DIR`). À vérifier dans les installeurs. Piper reste à ajouter aux ressources.
- Signature du code macOS et notarisation Apple (certificat Developer ID, secrets dans GitHub Actions).
- Signature Windows : à évaluer (coût d'un certificat vs alerte SmartScreen pour une application gratuite).
- Validation complète sur Windows : lancement, SQLite, Piper, parcours du lot 1.
- Mise à jour de l'application : à évaluer (`tauri-plugin-updater` ou téléchargement manuel).

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] Installation et premier lancement réussis sur macOS (arm64 et x64) et Windows x64, sur des machines sans SDK.
- [ ] Sur macOS, pas d'alerte Gatekeeper bloquante (si signature retenue) ; sinon procédure d'ouverture documentée
      dans le README.
- [ ] Procédure de publication d'une version documentée (`docs/release.md`).
- [ ] L'application installée charge tout le contenu : aucun « Invalid content skipped » dans les logs, compétences
      et personnages présents.
