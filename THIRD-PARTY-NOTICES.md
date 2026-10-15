# Composants tiers

Educator est une application gratuite. Elle utilise les composants suivants, sous leur propre licence.

## Embarqués avec l'application

Téléchargés par `scripts/fetch-piper.*`, jamais commités. Voir `docs/adr/0002-voix-piper.md`.

| Composant | Version | Licence | Source |
|---|---|---|---|
| Piper (moteur de synthèse vocale) | 2023.11.14-2 | MIT | https://github.com/rhasspy/piper |
| piper-phonemize | 2023.11.14-4 | MIT | https://github.com/rhasspy/piper-phonemize |
| espeak-ng (phonétisation, inclus dans Piper) | 1.52 | GPL-3.0-or-later | https://github.com/espeak-ng/espeak-ng |
| ONNX Runtime (inclus dans Piper) | 1.14.1 | MIT | https://github.com/microsoft/onnxruntime |
| Voix `fr_FR-siwis-medium` | — | CC BY 4.0 | https://huggingface.co/rhasspy/piper-voices |

**Attribution de la voix** : voix entraînée sur la *SIWIS French Speech Synthesis Database*
(https://datashare.is.ed.ac.uk/handle/10283/2353), licence Creative Commons Attribution 4.0. La citation exacte
demandée par les auteurs est à reprendre de cette page avant la première distribution (lot packaging).

**espeak-ng (GPL-3.0)** : Piper et ses bibliothèques sont exécutés comme programme séparé. Leur code source est
disponible aux adresses ci-dessus ; le texte de la licence GPL-3.0 sera joint aux installeurs (lot packaging).

## Cœur Rust (crates)

| Crate | Licence |
|---|---|
| rusqlite (avec SQLite embarqué, `bundled`) | MIT (SQLite : domaine public) |
| rand, rand_chacha, uuid, serde, serde_json, thiserror, log, chrono, roxmltree, sha2 | MIT ou Apache-2.0 |
| tauri-plugin-log | MIT ou Apache-2.0 |
| ts-rs, jsonschema, regex, tempfile (développement uniquement) | MIT ou Apache-2.0 |

## Front et coque (npm, crates)

| Paquet | Licence |
|---|---|
| React, React DOM, React Router, TanStack Query, Motion | MIT |
| Tauri et ses plugins | MIT ou Apache-2.0 |
| Polices Lexend et Barlow Condensed (`@fontsource/lexend`, `@fontsource/barlow-condensed`, embarquées dans le front) | SIL Open Font License 1.1 |
| Police Fredoka du thème Cosmos (`@fontsource/fredoka`, embarquée dans le front) | SIL Open Font License 1.1 |
| Police OpenDyslexic, option « police adaptée à la dyslexie » (`@fontsource/opendyslexic`, embarquée dans le front ; © Abbie Gonzalez, nom de police réservé « OpenDyslexic ») | SIL Open Font License 1.1 |
| `@tauri-apps/plugin-dialog` / `tauri-plugin-dialog` (fenêtres d'enregistrement et d'ouverture de la sauvegarde) | MIT ou Apache-2.0 |
| Vite, Vitest, Playwright, TypeScript (développement uniquement) | MIT / Apache-2.0 |

La liste exhaustive des dépendances transitives sera générée au lot packaging.
