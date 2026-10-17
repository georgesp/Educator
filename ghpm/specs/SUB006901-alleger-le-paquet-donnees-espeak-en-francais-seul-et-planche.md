---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoFA
url: https://github.com/georgesp/Educator/issues/291
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB006901 — Alléger le paquet : données espeak en français seul et planche des accessoires hors du paquet"
status: Backlog
size: S
estimate:
startDate:
endDate:
parent: "#69"
order: 665
---

## User story

En tant que parent, je veux une application aussi légère que possible à télécharger et à installer, afin qu'elle
ne prenne pas de place inutile sur l'ordinateur ou sur l'iPad.

## Contexte

Mesures du 2026-10-16 (CP + CE1) :

- IPA iPad (release) : 12 Mo installée, 5,3 Mo compressée ; binaire 9,5 Mo, contenu 3 Mo.
- `content/accessories/planche.html` (1,3 Mo) est une page de contrôle pour le développement (accessoires sur
  tous les personnages, SVG en ligne). Elle est embarquée avec le reste de `content/` (`bundle.resources` de
  `tauri.conf.json`), sur le bureau comme sur l'iPad : plus de 10 % de l'IPA.
- Piper sur le bureau : environ 107 Mo, dont `espeak-ng-data` = 19 Mo. Les dictionnaires des 113 langues
  pèsent 17 Mo (russe 8,2 Mo, mandarin 1,5 Mo…) alors que `fr_dict` fait 64 Ko. Seul le français est utilisé.

Ajouter les niveaux CE2 → CM2 ne représente que quelques Mo de contenu : ces deux postes sont les gains faciles.
Le changement de voix (`siwis-medium`, 63 Mo) est écarté : la qualité de la dictée passe avant.

## Contenu

### 1. Données espeak limitées au français

- `scripts/fetch-piper.sh` et `scripts/fetch-piper.ps1` ne gardent dans `espeak-ng-data` que les fichiers
  nécessaires au français (fichiers communs `phontab`, `phonindex`, `phondata`, `phondata-manifest`,
  `intonations`, `fr_dict`, définition de langue `lang/roa/fr`… liste exacte à établir en testant), dans
  une liste blanche commentée dans le script.
- Le filtrage se fait au téléchargement, pas seulement au packaging : le développement et l'application
  installée utilisent les mêmes fichiers, donc ce qui est testé est ce qui est livré.
- Une installation existante (`vendor/piper` déjà présent) est nettoyée à la relance du script (ou option
  `--force` documentée).

### 2. Planche des accessoires hors du paquet

- Déplacer la planche hors de `content/` (par exemple `docs/accessoires/planche.html`) : elle n'est plus copiée
  dans les ressources Tauri.
- Adapter `crates/educator-content/tests/accessories.rs` (chemin d'écriture et de comparaison), ainsi que la
  structure et la commande `EDUCATOR_WRITE_PLANCHE` dans `CLAUDE.md` / `AGENTS.md`.
- Aucun autre fichier non utilisé par l'application ne doit rester dans `content/` (vérification au passage).

## Critères d'acceptation

- [ ] `vendor/piper/<plateforme>/espeak-ng-data` pèse moins de 1 Mo sur macOS et Windows.
- [ ] La voix fonctionne à l'identique : `scripts/listen-words.sh --sample 50` (et `.ps1` sur Windows) lit tous les
      mots sans erreur, rendu comparé à celui d'avant le filtrage (mêmes WAV si Piper est déterministe, sinon
      écoute de contrôle).
- [ ] Consignes, dictée, « Plus lentement » et « Choisis la bonne écriture » fonctionnent dans `npm run tauri dev`.
- [ ] `planche.html` n'est plus dans le paquet : absente de `Educator.app/assets/content` (iPad) et des ressources
      de l'application de bureau.
- [ ] `cargo test --workspace` passe, y compris le test qui vérifie que la planche est à jour.
- [ ] `THIRD-PARTY-NOTICES.md` à jour (espeak-ng reste embarqué, sa licence aussi).
- [ ] Tailles avant / après notées dans cette fiche (IPA, dossier Piper).
