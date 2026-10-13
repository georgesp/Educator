---
type: Issue
title: Cache de la voix plafonné en taille
status: Todo
size: S
parent: specs/lot-2bis-voix-en-direct.md
order: 504
---

## User story

En tant que parent, je veux que l'application ne remplisse pas le disque, même après des mois d'utilisation.

## Comportement attendu

- Le cache disque de la voix (`SpeechCache`, dossier de données de l'application) est plafonné en **taille** (30 Mo par défaut) et non plus en nombre de fichiers (2 000 aujourd'hui, ≈ 100 Mo).
- Au-delà, les fichiers les moins récemment **lus** sont supprimés (aujourd'hui : les moins récemment écrits).
- Le cache reste jetable : un problème de lecture ou d'écriture se rabat sur la synthèse.
- La clé reste voix + vitesse + texte : changer de voix ou de prononciation ne sert jamais un ancien fichier.

## Critères d'acceptation

- [ ] Taille du cache jamais au-dessus du plafond (+ un fichier) ; test avec un petit plafond.
- [ ] Un fichier relu récemment est gardé plus longtemps qu'un fichier jamais relu (test).
