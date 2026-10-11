# ADR 0002 — Synthèse vocale avec Piper

- **Statut** : acceptée pour le moteur ; **voix proposée, à confirmer à l'écoute** (voir « Choix de la voix »)
- **Date** : 2026-10-10
- **Fiche** : SUB006006 (lot 0 — socle technique)

## Contexte

Educator lit à voix haute les consignes, les calculs et, au lot 2, les dictées. La voix doit être en français,
claire pour un enfant de 6 à 11 ans, fonctionner **hors ligne** et être **gratuite à redistribuer** dans un
installeur. Le cadrage a retenu Piper (synthèse neuronale locale, voix ONNX).

État de Piper en octobre 2026 :

- Le dépôt d'origine [`rhasspy/piper`](https://github.com/rhasspy/piper) (licence MIT) a été **archivé le
  6 octobre 2025**. Le développement continue dans [`OHF-Voice/piper1-gpl`](https://github.com/OHF-Voice/piper1-gpl)
  (licence **GPL-3.0**), distribué surtout comme paquet Python (`pip install piper-tts`), sans exécutable autonome.
- La dernière version publiée de `rhasspy/piper` (`2023.11.14-2`) fournit un exécutable pour Windows x64 et Linux.
  **Pour macOS, elle est incomplète** : l'archive « aarch64 » contient en réalité le même binaire Intel (x86_64)
  que l'archive x64, et les bibliothèques natives (`libespeak-ng`, `libpiper_phonemize`, `libonnxruntime`) n'y
  sont pas. Constaté le 2026-10-10 en inspectant les archives.
- Ces bibliothèques, compilées pour Intel, sont publiées par le projet frère
  [`rhasspy/piper-phonemize`](https://github.com/rhasspy/piper-phonemize) (version `2023.11.14-4`).

## Décision

### Moteur

1. **Exécutable `piper` de la version `2023.11.14-2`**, lancé par le host comme **processus annexe longue durée**
   en mode `--json-input` : le modèle de voix est chargé une seule fois, puis chaque phrase est une ligne JSON
   sur l'entrée standard ; Piper écrit un WAV dans un dossier privé et imprime son chemin sur la sortie standard.
   Implémentation : `PiperSpeechSynthesizer` (`Educator.Content`), derrière l'interface `ISpeechSynthesizer`
   (`Educator.Core`). Le processus est préchauffé au démarrage du host et relancé après un plantage, un délai
   dépassé ou un changement de vitesse (`--length_scale` est global au processus).
2. **macOS** : binaire Intel de Piper + bibliothèques Intel de `piper-phonemize`, chargées via
   `DYLD_LIBRARY_PATH`. Sur Apple Silicon, l'ensemble tourne avec **Rosetta 2**.
3. Les fichiers ne sont **pas commités** : `scripts/fetch-piper.sh` (macOS, Linux) et `scripts/fetch-piper.ps1`
   (Windows) les téléchargent dans `vendor/piper/` en vérifiant les sommes SHA-256 des archives du moteur.

Disposition, cherchée par `PiperLocator` dans cet ordre : variable `EDUCATOR_PIPER_DIR`, puis dossier `piper/` à côté
de l'application installée, puis premier `vendor/piper/` au-dessus du host (développement) :

```
vendor/piper/
  macos/    piper, *.dylib, espeak-ng-data/
  windows/  piper.exe, *.dll, espeak-ng-data/
  linux/    piper, *.so, espeak-ng-data/
  voices/   fr_FR-siwis-medium.onnx, .onnx.json, .MODEL_CARD
```

Sans Piper, l'application fonctionne **sans le son** : `GET /api/ui/status` indique pourquoi, et la lecture
répond `503`.

### Choix de la voix

Voix françaises disponibles ([rhasspy/piper-voices](https://huggingface.co/rhasspy/piper-voices/tree/main/fr/fr_FR)),
licences lues dans leur `MODEL_CARD` le 2026-10-10 :

| Voix | Locuteur | Licence du jeu de données | Redistribution |
|---|---|---|---|
| `fr_FR-siwis-medium` | femme | CC BY 4.0 (SIWIS, Université d'Édimbourg) | Oui, avec attribution |
| `fr_FR-upmc-medium` | 2 voix (femme, homme) | CC BY-SA 4.0 (UPMC / MaryTTS) | Oui, attribution + partage à l'identique de la voix |
| `fr_FR-tom-medium` | homme | AGPL-3.0 | Possible, mais contraintes fortes : écartée |
| `fr_FR-gilles-low`, `fr_FR-mls-medium` | homme / plusieurs | à vérifier | qualité « low » ou locuteurs hétérogènes |

**Voix par défaut proposée : `fr_FR-siwis-medium`** — voix féminine claire, licence la plus simple (attribution).
`scripts/compare-voices.sh` génère les mêmes phrases (calcul, mot avec accents, félicitations) avec chaque voix
candidate dans `vendor/piper/samples/` : **Georges choisit à l'écoute** et met à jour cette section. Une autre voix
s'utilise sans changer le code : `scripts/fetch-piper.sh <voix>` puis `EDUCATOR_PIPER_VOICE=<voix>`.

Mesure dans l'environnement de développement de Claude (Linux x64, voix siwis-medium) : 0,9 s pour une phrase de
10 mots au premier appel après démarrage, 0,13 s pour l'énoncé d'une addition une fois le moteur préchauffé.

## Licences

- **Piper** (`rhasspy/piper`, 2023.11.14-2) : MIT.
- **piper-phonemize** : MIT ; il embarque **espeak-ng** (**GPL-3.0**) et **ONNX Runtime** (MIT).
- **Voix** : selon le tableau ci-dessus ; `fr_FR-siwis-medium` impose de citer le jeu de données SIWIS.

espeak-ng étant sous GPL-3.0, l'ensemble « moteur Piper » est redistribué sous GPL-3.0. Educator l'exécute comme
**programme séparé** (processus annexe, échange par entrée / sortie standard), sans lier son code : le reste de
l'application n'est pas soumis à la GPL. Obligations à la distribution (lot packaging) : livrer les textes de
licence, indiquer où obtenir les sources de Piper, piper-phonemize et espeak-ng. Tout est consigné dans
`THIRD-PARTY-NOTICES.md`.

## Conséquences

- Pas de binaire natif Apple Silicon pour l'instant : Rosetta 2 est nécessaire sur les Mac M1 et suivants
  (`softwareupdate --install-rosetta --agree-to-license`). À reprendre au **lot packaging** : compiler `libpiper`
  de `piper1-gpl` pour arm64, ou réimplémenter l'inférence en .NET (ONNX Runtime + espeak-ng), sans changer
  `ISpeechSynthesizer`.
- Piper n'étant plus maintenu sous sa forme d'origine, les correctifs viendront de `piper1-gpl` (GPL-3.0).
- Les tests d'intégration de la voix (`PiperSpeechSynthesizerIntegrationTests`) ne s'exécutent que si Piper est
  installé ; la CI ne télécharge pas le moteur.
- Les voix ré-empaquetées par d'autres projets (par exemple sherpa-onnx) peuvent utiliser une version d'ONNX trop
  récente pour ce Piper (« Unsupported model IR version ») : toujours prendre les fichiers de `rhasspy/piper-voices`.
