---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAzCAc
rev: 2026-10-14T19:30:54Z
url: https://github.com/georgesp/Educator/issues/268
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB026701 — Cache de la voix plafonné en taille
status: Done
size: S
estimate:
startDate:
endDate:
parent: "#267"
order: 540
---

## User story

En tant que parent, je veux que l'application ne remplisse pas le disque, même après des mois d'utilisation.

## Comportement attendu

- Le cache disque de la voix (`SpeechCache`, dossier de données de l'application) est plafonné en **taille** (30 Mo par défaut) et non plus en nombre de fichiers (2 000 aujourd'hui, ≈ 100 Mo).
- Au-delà, les fichiers les moins récemment **lus** sont supprimés (aujourd'hui : les moins récemment écrits).
- Le cache reste jetable : un problème de lecture ou d'écriture se rabat sur la synthèse.
- La clé reste voix + vitesse + texte : changer de voix ou de prononciation ne sert jamais un ancien fichier.

## Critères d'acceptation

- [x] Taille du cache jamais au-dessus du plafond (+ un fichier) ; test avec un petit plafond.
- [x] Un fichier relu récemment est gardé plus longtemps qu'un fichier jamais relu (test).

## Implémentation (2026-10-15)

- `SpeechCache` plafonné à 30 Mo (`MAX_BYTES`) ; taille comptée à la première écriture (fichiers des lancements
  précédents compris), puis tenue à jour ; au-delà du plafond, suppression des fichiers les moins récemment lus
  jusqu'à 90 % du plafond. Une lecture depuis le cache remet la date du fichier à l'heure (horloge injectée :
  `SpeechCache::with_limit`).
- `contains` et `forget` (refaire une lecture ratée, SUB026706).
- Tests : `the_cache_never_grows_above_its_cap`, `a_file_read_again_stays_longer_than_a_file_never_read`,
  `the_cache_counts_what_earlier_runs_left`, `a_forgotten_reading_is_synthesized_again`.
