---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsEKY
rev: 2026-10-15T13:35:20Z
url: https://github.com/georgesp/Educator/issues/252
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023813 — Icônes, écran de lancement et installation sur la tablette de test"
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#238"
order: 3130
---

## User story

En tant que Georges, je veux installer Educator sur la tablette de mon enfant, avec une vraie icône, et la mettre à jour facilement.

## Dépend de

Fiches 01 et 02.

## Comportement attendu

- Icônes iOS (jeu complet généré depuis l'icône existante, `tauri icon`), écran de lancement sobre (couleur de la charte, mascotte), nom affiché « Educator ».
- iPad : installation sur appareil avec l'Apple ID gratuit (« Personal Team », profil valable 7 jours, voir `docs/tablette.md`) ; TestFlight seulement si un compte Apple Developer est pris.
- Procédure de mise à jour documentée dans `docs/tablette.md` (nouvelle version par-dessus, données conservées).
- `THIRD-PARTY-NOTICES.md` : rien à ajouter si seule la voix du système est utilisée ; à vérifier.

## Critères d'acceptation

- [ ] Installation réussie sur la tablette de test à partir d'un clone propre en suivant `docs/tablette.md`.
- [ ] Une mise à jour conserve les profils et les résultats.
- [ ] Hors périmètre rappelé : publication sur les stores.

## Implémentation (2026-10-15)

- Icône : `gen/apple` contenait encore l'icône par défaut de Tauri. Source `web/src-tauri/icons/icon-source.svg` (1 024 px, redessinée d'après `icon.png`) ; génération par `npm run tauri -- icon src-tauri/icons/icon-source.svg --ios-color "#FF7A45"` (`docs/tablette.md`, étape 9) : **à lancer par Georges** puis commiter.
- Écran de lancement blanc explicite (fond du thème), aussi en mode sombre ; nom affiché « Educator ».
- `docs/tablette.md` : version autonome (`npm run tauri ios build`, installation du `.ipa` par Xcode) et **mise à jour** par-dessus en conservant les données (étape 10).
- `THIRD-PARTY-NOTICES.md` : rien à ajouter (voix du système, aucune voix ni moteur embarqué ; crates Rust sous licences permissives).
- Hors périmètre : publication sur l'App Store (lot packaging).

## Tests à faire (revue)

1. `cd web && npm run tauri -- icon src-tauri/icons/icon-source.svg --ios-color "#FF7A45"` → `src-tauri/icons/` et `gen/apple/Assets.xcassets/AppIcon.appiconset/` régénérés ; l'icône du bureau reste la même (orange au « + »), l'icône de l'iPad aussi, sans coins blancs ni noirs. Commiter les deux dossiers.
2. `npm run tauri ios build` puis installation du `.ipa` par Xcode (`docs/tablette.md`, étape 10) → Educator s'ouvre sans le Mac, avec son icône ; écran de lancement blanc (iPad en mode sombre aussi).
3. Mise à jour : créer un profil, faire une série, réinstaller une nouvelle version par-dessus → profil et résultats toujours là.
4. Clone propre : suivre `docs/tablette.md` depuis le début dans un autre dossier → installation réussie (noter les écarts éventuels dans la doc).
