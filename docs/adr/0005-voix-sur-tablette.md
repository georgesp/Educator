# ADR 0005 — Voix sur tablette (étude)

- **Statut** : proposée — partie documentaire faite ; voix du système mesurée sur iPad et **contrat WAV retenu**
  pour la tablette (2026-10-15, SUB023804, plugin `tauri-plugin-system-voice`)
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

## Voix du système sur iPad — mesures (SUB023804, 2026-10-15)

Banc d'essai : page de développement `#/dev/voix` (`web/src/screens/VoiceLabScreen.tsx`, absente de l'application
construite), dans Educator lancé par `npm run tauri ios dev` sur l'iPad de test (iPad Air 13 pouces M2, iPadOS 27).
La voix y est appelée par l'API Web Speech, que WebKit lit avec `AVSpeechSynthesizer` : même moteur et mêmes voix
de base qu'un plugin natif.

### Voix disponibles

| Voix | Langue | Qualité |
|---|---|---|
| Thomas | fr-FR | `compact` et `super-compact` |
| Amélie | fr-CA (canadienne : à écarter pour la France) | `compact` et `super-compact` |

Aucune voix « améliorée » ou « premium » : aucune n'était téléchargée sur l'iPad. Reste à vérifier si une voix
améliorée téléchargée apparaît dans la vue web ou seulement côté natif (`AVSpeechSynthesisVoice.speechVoices()`).

### Temps mesurés (Thomas `compact`, voix par défaut)

| Texte | Vitesse 1 | Vitesse 0,9 | Vitesse 0,5 |
|---|---|---|---|
| « un stylo. » | premier son 39 ms, fin 1 449 ms (1re lecture) | 6 ms, 905 ms | 15 ms, 1 052 ms |
| « J'écris avec un stylo. » | 5 ms, 1 701 ms | 9 ms, 1 724 ms | 6 ms, 2 026 ms |
| « Écoute bien le mot, puis écris-le. » | 6 ms, 2 583 ms | — | — |
| « un stylo. » lancé 1,5 s après le clic | 4 ms, 824 ms | | |

- **Premier son en 4 à 39 ms** : quasi immédiat (Piper sur le bureau : 166 à 198 ms pour un mot, ADR 0004).
- La vitesse 0,5 de l'API Web n'est **pas** une demi-vitesse (à peine 15 % plus lent que 0,9) : l'écoute lente
  devra régler `AVSpeechUtterance.rate` côté natif et être réécoutée.
- Lecture lancée 1,5 s après le clic : jouée (pas de blocage lié au geste de l'utilisateur).

### Lecture d'un WAV par le front (contrat actuel)

WAV 16 bits mono de 0,4 s joué par `new Audio()` : **joué** tout de suite, après 300 ms, après 1,5 s et après un
appel au cœur (`get_status`). Le risque noté dans l'ADR 0006 (lecture refusée après un appel asynchrone) est écarté
dans la vue web de Tauri.

### Mode silencieux

Avec le mode silencieux de l'iPad, la voix du système **ne sort pas** (constaté). Le plugin natif devra placer la
session audio de l'application en `AVAudioSession.Category.playback` pour que consignes et dictées s'entendent
quand même, comme une application de vidéo.

### Écoute de contrôle (Georges)

- Thomas `compact` : **qualité jugée bonne** pour un enfant.
- Vitesse lente : jugée bonne (à réécouter quand le ralenti sera réglé côté natif).

### Conclusions

- **Pas de lecture par l'API Web dans le front** : le mot d'une dictée devrait être envoyé au front avant la
  réponse (contraire à `speak_series_word`), le mode silencieux coupe la voix et les voix améliorées ne sont pas
  garanties.
- La voix passe par le **plugin natif** derrière `SpeechSynthesizer` (SUB023805, SUB023806). Deux contrats restent
  possibles :
  - **(A) WAV** : `AVSpeechSynthesizer.write(_:toBufferCallback:)` produit les échantillons, le Rust renvoie le WAV
    comme avec Piper. Front, cache (`SpeechCache`) et sécurité de la dictée inchangés. **Préféré**, si `write` est
    fiable (à essayer).
  - **(B) Lecture native** : le Swift lit lui-même ; commandes « lire » / « arrêter » et fin de lecture notifiée
    au front ; plus de cache.
- Dans les deux cas : session audio `.playback`, vitesses réglées côté natif, voix `fr-FR` choisie explicitement
  (jamais `fr-CA`), message au parent si aucune voix française n'est installée.

## Voix native par le plugin — mesures et décision (SUB023804, 2026-10-15)

Plugin local `plugins/tauri-plugin-system-voice/` (Swift + Rust), branché derrière `SpeechSynthesizer` par
`web/src-tauri/src/system_voice.rs` sur iOS seulement ; essais par la section 5 de `#/dev/voix` (commandes `speak`
et `speak_word`, chaîne complète cœur → Swift → WAV → front).

| | Résultat |
|---|---|
| `AVSpeechSynthesizer.write` | **Fiable** : toutes les synthèses réussies, échantillons PCM convertis en WAV 16 bits mono par le Rust |
| Format | 22 050 Hz, 16 bits, mono (37 Ko pour « un stylo. », 0,87 s) |
| Temps jusqu'au WAV reçu par le front | **300 à 335 ms** (Piper sur le bureau : ≈ 170 à 200 ms) ; masqué pour les dictées par la préparation des mots (ADR 0004) |
| Voix choisie | « Thomas » (qualité « default »), seule voix `fr-FR` installée ; la meilleure qualité installée est prise d'office (premium, puis améliorée) |
| Mode silencieux | Voix **entendue** grâce à la session audio `.playback` (le WAV joué par la vue web suit cette session) |
| Vitesse lente (0,5, `AVSpeechUtteranceDefaultSpeechRate` × vitesse) | Jugée bonne par Georges |
| Mot seul (« Le mot : jaune. » coupé) | Thomas ne marque presque pas de pause après les deux-points : la coupe du cœur (pause ≥ 150 ms) ne la trouvait pas. Le plugin lit le texte en **SSML avec une pause de 400 ms après chaque deux-points** (`COLON_PAUSE_MS`), repli sur le texte brut si le SSML est refusé : **coupe entendue correcte** |

Défaut trouvé au passage : la commande `speak_word` (non utilisée par l'interface, seulement par la page de test)
ne coupait jamais le mot, sur le bureau comme sur l'iPad ; elle suit désormais le chemin des dictées
(`Reading::synthesize`), test `a_word_alone_is_cut_after_its_carrier_and_its_sentence_is_read_whole`.

**Décision** : contrat **(A) WAV** pour la tablette. Le front, la préparation des mots, la coupe et la sécurité de
la dictée (`speak_series_word`) sont inchangés ; la lecture native (B) est écartée.

## Reste à faire (lot tablette)

- SUB023805 / SUB023806 : arrêt d'une lecture quand une autre commence et à la mise en arrière-plan, reprise après
  une interruption (appel), message au parent sans voix française, état de la voix dans la barre d'état (texte
  propre à l'iPad au lieu de Piper / Rosetta), cache éventuel des phrases sur l'iPad.
- Essayer une voix améliorée téléchargée (choix automatique, qualité, temps de synthèse).
- Moteur intégré (sherpa-onnx, option B des « Options étudiées ») sur iPad : temps de synthèse, mémoire, taille ajoutée à
  l'application (fiche à créer, si la voix Piper est voulue sur tablette).
- Écoute de `fr_FR-siwis-low`.

## Sources

- [The GPL, the App Store, and you — Engadget (2011)](https://www.engadget.com/2011-01-09-the-gpl-the-app-store-and-you.html)
- [FSF position on GPLv2 & current App Store terms — vlc-devel (2010)](https://mailman.videolan.org/pipermail/vlc-devel/2010-November/077027.html)
- [App Store & GPL-2.0 License — Apple Developer Forums](https://developer.apple.com/forums/thread/722101)
- [OHF-Voice/piper1-gpl — releases (GPL-3.0, `libpiper`, espeak-ng embarqué)](https://github.com/OHF-Voice/piper1-gpl/releases)
- [sherpa-onnx — voix Piper](https://github.com/k2-fsa/sherpa/blob/master/docs/source/onnx/tts/piper.rst)
- [sherpa_onnx — plateformes prises en charge (dont iOS arm64)](https://pub.dev/documentation/sherpa_onnx/latest/)
- [ONNX Runtime pour iOS (Objective-C)](https://onnxruntime.ai/docs/get-started/with-obj-c.html)
