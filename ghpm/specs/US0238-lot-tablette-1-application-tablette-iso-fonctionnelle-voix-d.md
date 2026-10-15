---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsC4w
rev: 2026-10-15T19:44:26Z
url: https://github.com/georgesp/Educator/issues/238
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "US0238 — Lot tablette 1 — Application tablette (iso-fonctionnelle, voix de la tablette)"
status: In progress
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
- Mac : MacBook Pro M1 Pro, macOS Golden Gate 27.0.1, Xcode 27.0.
- Compte : **Apple ID gratuit** pour commencer (installation valable 7 jours, pas de TestFlight) ; compte Apple Developer payant à décider avant un usage quotidien par les enfants.

## Avancement (2026-10-15)

- Terminées : SUB023801 (étude, ADR 0006), SUB023802 (projet iOS), SUB023803 (données, contenu, taille 4,9 Mo), SUB023804 (étude de la voix : contrat WAV, ADR 0005).
- En revue : SUB023805 (plugin de voix), SUB023806 (voix iOS), SUB023808 (écrans, zones sûres), SUB023809 (tactile), SUB023810 (saisie sans clavier du système), SUB023811 (cycle de vie), SUB023812 (CI iOS), SUB023813 (icônes, installation, mise à jour), SUB023814 (recette : checklist prête dans `docs/tablette.md`, à dérouler sur l'iPad). Chaque fiche a sa section « Tests à faire (revue) ».
- Mise de côté : SUB023807 (voix Android).

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
