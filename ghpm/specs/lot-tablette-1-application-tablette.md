---
type: Issue
title: Lot tablette 1 — Application tablette (iso-fonctionnelle, voix de la tablette)
status: Todo
size: XL
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

## À trancher avant de démarrer (fiche 01)

- Plateforme(s) visée(s) en premier : iPad, tablette Android, ou les deux (les fiches iOS et Android sont séparées pour pouvoir en laisser une de côté).
- Tablette(s) de test et versions minimales de système.
- iPad : compte Apple Developer payant (installation durable sur appareil, TestFlight) ou installation de 7 jours avec un compte gratuit ; Mac avec Xcode.
- Android : Android Studio, SDK et NDK ; installation d'un APK de test sans compte Google Play.

## Hors périmètre

- Stylet et reconnaissance de l'écriture : lot tablette 2.
- Publication sur l'App Store et Google Play (fiche de signature et de publication à créer plus tard, dans le lot packaging US0069 ou un lot dédié).
- Synchronisation des profils et des résultats entre l'ordinateur et la tablette : les données restent locales à chaque appareil.
- Téléphones (écrans de moins de 7 pouces) ; tablettes Windows.

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] Sur la tablette de test : création d'un profil, série de calcul (clavier et QCM), dictée avec la voix de la tablette, leçon, examen et défi chrono fonctionnent de bout en bout, sans réseau.
- [ ] Les profils et les résultats sont conservés après fermeture complète et relance de l'application.
- [ ] Aucune règle métier ajoutée dans le front ni dans `web/src-tauri` : la voix passe par le trait `SpeechSynthesizer` de `educator-core`.
- [ ] Le build bureau (macOS, Windows) et la CI existante restent verts.
