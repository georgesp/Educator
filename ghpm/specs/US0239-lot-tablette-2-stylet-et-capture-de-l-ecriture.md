---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsC8w
rev: 2026-10-15T19:43:23Z
url: https://github.com/georgesp/Educator/issues/239
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: US0239 — Lot tablette 3 — Stylet et capture de l'écriture
status: Backlog
size: XL
estimate:
startDate:
endDate:
parent:
order: 4000
---

Source : demande de Georges du 2026-10-14 ; étude de faisabilité du stylet du même jour.

- **Lot** : tablette 3 — Stylet et capture de l'écriture
- **Prérequis** : lot tablette 1 (application sur tablette, saisie sans correction automatique) ; lot 2 (dictée, mots à trous).
- **Lot suivant** : à définir (selon les résultats de l'étude).

## Contexte

Sur tablette, l'enfant peut écrire les mots à la main avec un stylet (Apple Pencil, S Pen ou autre) au lieu de taper au clavier. Les exercices d'orthographe sont les premiers concernés : dictée de mots, mots à trous.

Faisabilité (2026-10-14) :
- **Capture du stylet** : facile. Les webviews exposent les Pointer Events (`pointerType: "pen"`, pression, inclinaison), ce qui permet une zone d'écriture et le rejet de la paume.
- **Reconnaissance de l'écriture** : c'est le vrai risque. Trois voies : (1) écriture manuscrite du système dans un champ texte (Scribble sur iPad, écriture manuscrite Android récente), sans développement mais dépendante de la tablette et des réglages ; (2) zone d'écriture maison et reconnaissance native (Android : ML Kit Digital Ink, français, hors ligne ; iOS : pas d'API publique de reconnaissance d'encre) ; (3) modèle embarqué (ONNX / TFLite), gros chantier.
- **Accents** : la règle « aucune tolérance sur les accents » rend une erreur de reconnaissance (« é » lu « e ») injuste pour l'enfant. D'où la confirmation de la lecture (fiche 07) : l'enfant valide ce que l'application a compris avant la correction.

## Principes

- Le stylet est **optionnel** : le clavier reste disponible partout, à tout moment.
- Aucune règle métier dans le front : la validation reste celle de SUB001405, dans le Rust. La reconnaissance fournit du texte (et des candidats), jamais un verdict.
- Une erreur de reconnaissance n'est jamais comptée comme une erreur d'orthographe de l'enfant.
- Retours bienveillants, jamais culpabilisants. Tout fonctionne hors ligne ; aucune écriture d'enfant n'est envoyée en ligne ni conservée au-delà de la saisie, sauf décision contraire écrite dans l'ADR.

## Objectif

Permettre à l'enfant d'écrire au stylet les réponses des exercices d'orthographe, avec une reconnaissance assez fiable (accents compris) pour que l'exercice reste juste et agréable, et un repli au clavier quand elle ne l'est pas.

## Hors périmètre

- Mathématiques au stylet (chiffres, opérations posées) : lot ultérieur si le stylet donne satisfaction.
- Dessin libre, coloriage, annotations dans les leçons.
- Appréciation de la qualité de l'écriture (calligraphie) : seul le mode « tracé guidé » facultatif (fiche 11) s'en approche, sans note.
- Reconnaissance en ligne par un service externe (contraire au principe « aucune donnée envoyée »).

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] Sur la tablette de test, une dictée de mots complète est faite au stylet, de bout en bout, avec confirmation de la lecture et repli au clavier.
- [ ] Taux de reconnaissance mesuré sur des écritures d'enfants (fiche 12) et conforme au seuil fixé par l'étude (fiche 01).
- [ ] Le build bureau et la CI existante restent verts ; sans stylet, aucun écran ne change.
