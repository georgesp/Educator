# ADR 0005 — Voix sur tablette (étude)

- **Statut** : proposée — partie documentaire faite, **mesures sur iPad à faire** (lot tablette)
- **Date** : 2026-10-15
- **Fiche** : SUB026703 (lot 2bis), suite dans le lot tablette 1 (SUB023804 à SUB023806)

## Contexte

Sur le bureau, la voix est le moteur Piper lancé comme **processus annexe** (ADR 0002, ADR 0004). Sur iOS, une
application ne peut pas lancer de processus annexe : avoir la même voix sur la tablette demande un moteur
**intégré à l'application**. Un moteur intégré supprimerait aussi le besoin de Rosetta 2 sur les Mac Apple Silicon
(Piper 2023.11.14 n'existe qu'en binaire Intel pour macOS).

Le lot tablette 1 a déjà retenu la **voix du système** (`AVSpeechSynthesizer` sur iOS, `TextToSpeech` sur Android)
pour une première version iso-fonctionnelle. Cette étude dit si, et comment, la voix Piper peut suivre.

## Options étudiées

| Option | Principe | Licence | État |
|---|---|---|---|
| A. Voix du système | `AVSpeechSynthesizer` / `TextToSpeech` via un plugin Tauri | Aucune contrainte | Retenue pour le lot tablette 1 |
| B. sherpa-onnx | Bibliothèque C/C++ (API C) qui lit les voix Piper converties (métadonnées + `tokens.txt`) ; ONNX Runtime ; phonétisation espeak-ng | sherpa-onnx : Apache-2.0 ; ONNX Runtime : MIT ; **espeak-ng : GPL-3.0** | iOS arm64, Android, macOS, Windows, Linux pris en charge |
| C. Piper réimplémenté en Rust | crate `ort` (ONNX Runtime) + phonétisation | `ort` : MIT / Apache-2.0 ; phonétisation : espeak-ng (GPL-3.0) ou autre | Tout à écrire (phonèmes → identifiants, inférence, WAV) |
| D. `libpiper` de `piper1-gpl` | API C du successeur officiel de Piper, espeak-ng embarqué | **GPL-3.0** pour tout le moteur | API C apparue en 2025 (v1.5 à v1.8) ; pas de cible iOS publiée |

Les options B, C et D partagent la même question : la **phonétisation** (texte → phonèmes) que les voix Piper
attendent est celle d'**espeak-ng**, sous **GPL-3.0**.

## Licence espeak-ng et App Store

- Sur le bureau, Educator **exécute** Piper comme programme séparé (échange par entrée / sortie standard) : le reste
  de l'application n'est pas soumis à la GPL (ADR 0002).
- Sur iOS, espeak-ng serait **lié** dans l'application : l'application entière devrait être distribuée sous
  GPL-3.0 (sources fournies, mêmes droits pour les utilisateurs).
- La FSF considère que les conditions d'utilisation de l'App Store (usage limité à certains appareils, pas de
  redistribution ni de modification) ajoutent des restrictions que la GPL interdit ; en 2011, VLC a été retiré de
  l'App Store à la demande d'un de ses auteurs pour cette raison. La question n'a jamais été tranchée en justice,
  mais un auteur d'espeak-ng peut obtenir le retrait d'une application qui l'embarque.
- La GPL-3.0 exige aussi, pour un appareil grand public, les « informations d'installation » permettant
  d'installer une version modifiée : difficile à honorer sur iOS hors comptes développeurs.
- **Installation personnelle** (Xcode ou compte développeur, sur les tablettes de la famille) : ce n'est pas une
  distribution par l'App Store ; la question ne se pose pas, Georges étant l'auteur de l'application et
  respectant la GPL pour espeak-ng.

Contournements possibles si une publication sur l'App Store devient un objectif :

1. **Phonèmes pré-calculés au build** : tous les textes lus sont connus d'avance (mots, phrases d'exemple,
   consignes, phrases de la mascotte, leçons). espeak-ng tournerait seulement sur la machine de build et l'application
   n'embarquerait que les phonèmes (la sortie d'un programme GPL n'est pas couverte par la GPL). Les textes
   dynamiques (prénom d'un profil, nombres) passeraient par la voix du système, ou par un petit phonétiseur maison
   (nombres en lettres déjà écrits en Rust : `speech::number_to_words`).
2. Voix du système uniquement sur tablette (option A), Piper sur le bureau.

## Prototype mesuré (environnement de développement de Claude)

sherpa-onnx 1.13.8 (Python, Linux x86-64, 2 cœurs), voix `fr_FR-siwis-medium` convertie (métadonnées ajoutées,
`tokens.txt` tiré de `phoneme_id_map`), vitesse des mots (`length_scale` 1,18), comparé au Piper actuel sur la même
machine et la même voix :

| Texte | sherpa-onnx | Piper (processus annexe) |
|---|---|---|
| « un stylo. » | 74 à 233 ms | 166 à 198 ms |
| « J'écris avec un stylo. » | 125 à 346 ms | 264 à 323 ms |
| « Écoute bien le mot, puis écris-le. » | 208 à 488 ms | 424 à 482 ms |
| Chargement de la voix | 1,6 s | ≈ 0,5 s (premier mot) |
| Mémoire (RSS) | 181 à 221 Mo (avec l'interpréteur Python) | ≈ 105 Mo par processus |

- Même voix, même rendu : sherpa-onnx lit les voix Piper sans perte, aux mêmes vitesses.
- Taille ajoutée : voix 63 Mo (`siwis-medium`) ; données espeak-ng réduites au **français : 0,8 Mo** (19 Mo pour
  toutes les langues), testées sans différence ; bibliothèques sherpa-onnx + ONNX Runtime : 37 Mo sur Linux x86-64
  (la taille d'un framework iOS arm64 statique reste à mesurer).
- Sur iPad, la préparation des mots à l'avance (ADR 0004) masque le temps de synthèse comme sur le bureau.

## Recommandation

1. **Lot tablette 1** : garder la voix du système (option A), comme décidé ; aucune contrainte de licence ni de
   taille. Le trait `SpeechSynthesizer` et la préparation des mots (`SpeechPreparer`) s'appliquent tels quels.
2. **Même voix que l'ordinateur** (lot tablette ultérieur, ou lot packaging pour retirer Rosetta) : **sherpa-onnx**
   (option B) derrière `SpeechSynthesizer`, voix Piper convertie, données espeak-ng du français seulement.
   À n'envisager pour l'App Store qu'avec les phonèmes pré-calculés (contournement 1) ; en installation personnelle,
   espeak-ng peut être embarqué en respectant la GPL.
3. **Voix plus légère** `fr_FR-siwis-low` : à télécharger (`scripts/fetch-piper.sh fr_FR-siwis-low`) et à écouter
   par Georges avec `scripts/compare-voices.sh` avant de la retenir pour la tablette.

## Reste à faire (lot tablette)

- Prototype sur iPad : temps de synthèse d'un mot et d'une phrase, mémoire, taille ajoutée à l'application (fiche
  à créer dans le lot tablette, à côté de SUB023804).
- Écoute de `fr_FR-siwis-low`.

## Sources

- [The GPL, the App Store, and you — Engadget (2011)](https://www.engadget.com/2011-01-09-the-gpl-the-app-store-and-you.html)
- [FSF position on GPLv2 & current App Store terms — vlc-devel (2010)](https://mailman.videolan.org/pipermail/vlc-devel/2010-November/077027.html)
- [App Store & GPL-2.0 License — Apple Developer Forums](https://developer.apple.com/forums/thread/722101)
- [OHF-Voice/piper1-gpl — releases (GPL-3.0, `libpiper`, espeak-ng embarqué)](https://github.com/OHF-Voice/piper1-gpl/releases)
- [sherpa-onnx — voix Piper](https://github.com/k2-fsa/sherpa/blob/master/docs/source/onnx/tts/piper.rst)
- [sherpa_onnx — plateformes prises en charge (dont iOS arm64)](https://pub.dev/documentation/sherpa_onnx/latest/)
- [ONNX Runtime pour iOS (Objective-C)](https://onnxruntime.ai/docs/get-started/with-obj-c.html)
