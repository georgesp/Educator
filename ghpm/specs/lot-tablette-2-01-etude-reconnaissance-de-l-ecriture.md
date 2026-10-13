---
type: Issue
title: Étude — reconnaissance de l'écriture au stylet (échantillons, comparaison, go / no-go)
status: Todo
size: M
parent: specs/lot-tablette-2-stylet-et-capture-de-l-ecriture.md
order: 4010
---

## User story

En tant que Georges, je veux savoir, mesures à l'appui, si la reconnaissance de l'écriture d'un enfant de 6 à 10 ans est assez fiable pour corriger une dictée, et par quelle voie.

## Questions à trancher

- Voie 1 — écriture manuscrite du système : fonctionne-t-elle dans les champs de la webview (WKWebView, WebView Android) ? Versions minimales de système, réglage à activer par l'utilisateur, stylets compatibles, comportement avec un champ sans correction automatique.
- Voie 2 — zone d'écriture maison + reconnaissance native : ML Kit Digital Ink sur Android (modèle français, taille, téléchargement hors ligne) ; iOS : options réelles (Vision sur image du tracé, modèle embarqué), qualité attendue.
- Voie 3 — modèle embarqué en Rust (`ort` / `tract`) : modèles disponibles, licences, poids, performance sur tablette.
- Accents et diacritiques : taux d'erreur sur é, è, ê, à, ç, ï, ô, ù…
- Écriture cursive ou script ? (programmes du cycle 2 : écriture cursive) ; lettres capitales (tolérées « pour l'instant », CLAUDE.md).

## Protocole

- Jeu d'échantillons d'écriture à constituer : mots de la banque CP–CE1, enfants de plusieurs âges, avec l'accord des parents ; tracés bruts (points, temps, pression) pour pouvoir rejouer toutes les voies sur les mêmes données.
- Mesures : taux de mots exactement reconnus, taux d'erreurs d'accents, temps de reconnaissance, taille ajoutée, besoin de réseau.

## Livrable

- ADR 0006 « Reconnaissance de l'écriture » : mesures, voie retenue (ou combinaison par plateforme), seuil minimum pour la mise en production, décision **go / no-go** par Georges. Les fiches 04 à 07 sont ajustées en conséquence (certaines peuvent être supprimées).

## Critères d'acceptation

- [ ] Au moins une voie essayée sur chaque plateforme retenue, avec les mêmes échantillons.
- [ ] Décision go / no-go et seuil de fiabilité écrits dans l'ADR.
- [ ] Règles de conservation des tracés d'enfants (aucun envoi, suppression avec le profil) écrites dans l'ADR.
