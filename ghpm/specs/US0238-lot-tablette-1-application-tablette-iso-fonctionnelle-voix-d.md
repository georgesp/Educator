---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsC4w
url: https://github.com/georgesp/Educator/issues/238
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "US0238 — Lot tablette 1 — Application tablette (iso-fonctionnelle, voix de la tablette)"
status: Todo
size: XL
estimate:
startDate:
endDate:
parent:
order: 3000
---

Source : demande de Georges du 2026-10-14 — pouvoir déployer Educator sur tablette.

- **Lot** : tablette 1 — Application tablette
- **Prérequis** : lot 1bis (backend Rust, terminé) ; lot 2 (orthographe, leçons, examens) assez avancé pour valider la dictée sur tablette.
- **Lot suivant** : fiche « Lot tablette 2 — Stylet et capture de l'écriture ».

## Contexte

Le lot 1bis a rendu l'application autonome : tout est dans le processus Tauri (Rust), sans serveur ni processus annexe. Ce qui est déjà prêt pour le mobile : `#[cfg_attr(mobile, tauri::mobile_entry_point)]`, `crate-type = ["staticlib", "cdylib", "rlib"]`, SQLite embarqué (`rusqlite` bundled), plugins bureau isolés derrière `cfg(desktop)`, synthétiseur « indisponible » utilisé hors bureau (`UnavailableSpeechSynthesizer`).

Ce qui ne fonctionne pas tel quel sur tablette : Piper (processus annexe interdit sur iOS, binaire Intel), l'interface pensée pour la souris et un écran de 1 024 × 700 minimum, le clavier physique, les dossiers de données du bureau, le cycle de vie d'une application qui peut être suspendue ou tuée à tout moment.

## Décision de principe

La tablette utilise la **voix du système** (`AVSpeechSynthesizer` sur iPad, `TextToSpeech` sur Android), pas Piper. Elle remplace la partie « tablette » de l'étude du lot 2bis (« Étude — moteur de voix intégré pour la tablette »), à clôturer une fois ce lot lancé. Conséquence : pas de question de licence espeak-ng / App Store, pas de modèle à embarquer. Contrepartie : la voix dépend de la tablette (voix française à installer, qualité variable).

## Objectif

Lancer Educator sur une tablette et y retrouver le même parcours qu'à l'ordinateur (profils, mathématiques, orthographe, leçons, examens, défi chrono, mascotte), avec la voix de la tablette, hors ligne, sans le stylet (lot suivant). Saisie au clavier à l'écran et au doigt.

## Décisions de la fiche 01 (2026-10-15)

Détail et raisons : `docs/adr/0006-application-tablette.md` ; préparation du Mac et de l'iPad : `docs/tablette.md`.

- **iPad seulement** (pas d'iPhone) ; Android mis de côté (SUB023807 retirée du lot).
- Tablette de test : **iPad Air 13 pouces (M2)**, **iPadOS 27** ; iPadOS 17 minimum déclaré.
- Orientation : **paysage seul**.
- Mac : MacBook Pro M1 Pro, Xcode 27 (macOS Tahoe 26.6 ou plus récent).
- Compte : **Apple ID gratuit** pour commencer (installation valable 7 jours, pas de TestFlight) ; compte Apple Developer payant à décider avant un usage quotidien par les enfants.

## Hors périmètre

- Stylet et reconnaissance de l'écriture : lot tablette 2.
- Publication sur l'App Store et Google Play (fiche de signature et de publication à créer plus tard, dans le lot packaging US0069 ou un lot dédié).
- Synchronisation des profils et des résultats entre l'ordinateur et la tablette : les données restent locales à chaque appareil.
- Tablettes Android (mises de côté, ADR 0006) ; iPhone.
- Téléphones (écrans de moins de 7 pouces) ; tablettes Windows.

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] Sur la tablette de test : création d'un profil, série de calcul (clavier et QCM), dictée avec la voix de la tablette, leçon, examen et défi chrono fonctionnent de bout en bout, sans réseau.
- [ ] Les profils et les résultats sont conservés après fermeture complète et relance de l'application.
- [ ] Aucune règle métier ajoutée dans le front ni dans `web/src-tauri` : la voix passe par le trait `SpeechSynthesizer` de `educator-core`.
- [ ] Le build bureau (macOS, Windows) et la CI existante restent verts.
