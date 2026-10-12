# CLAUDE.md — Educator

Instructions et contexte pour travailler sur ce dépôt avec Claude (ou tout agent de code).

## Vue d'ensemble

**Educator** est une application de bureau gratuite (macOS et Windows) qui enseigne les **mathématiques** (calcul)
et l'**orthographe** aux enfants de l'école primaire, du **CP au CM2**, en suivant les programmes officiels de
l'Éducation nationale.

- Plusieurs **profils locaux** par ordinateur (nom + personnage SVG), sans mot de passe, choisis à l'ouverture.
- L'enfant est autonome ; un **espace parent** affiche les statistiques (enregistrées au lot 2, interfaces au lot 3).
- Menu principal : **Leçons**, **Exercices**, **Examen**, **Options** ; mode contre-la-montre.
- Exercices de maths **générés en direct** à partir de règles par niveau (pas de banque figée).
- Voix (consignes, dictées) : synthèse vocale **locale** avec **Piper**, hors ligne.
- Aucun compte en ligne, aucune donnée envoyée : tout reste sur l'ordinateur.

Depuis le lot 1bis (ADR 0003), toute la logique est en **Rust**, dans l'application Tauri : plus de host .NET, de
serveur local ni de token. Cela prépare une version tablette (iOS n'autorise pas de processus annexe).

## Stack technique

| Couche | Technologie |
|---|---|
| Application | Tauri 2 (Rust) — `web/src-tauri/` : commandes, état partagé, démarrage, logs — **aucune règle métier** |
| Cœur | Workspace Cargo (`crates/`) : `educator-core`, `educator-content`, `educator-data` |
| Front | React 19 + TypeScript (Vite), TanStack Query, Framer Motion — `web/` |
| Stockage | SQLite embarqué (`rusqlite`, `bundled`), migrations versionnées (`educator-data`) |
| Synthèse vocale | Piper (binaire + voix française), lancé depuis Rust sur le bureau |
| Logs | `log` + `tauri-plugin-log` (console et fichier) |
| Tests Rust | `cargo test` (tests d'intégration dans `crates/*/tests/`, tests unitaires des commandes dans `web/src-tauri`) |
| Tests front | Vitest (composants), Playwright (parcours) |

## Structure

```
Educator/
├── Cargo.toml              # Workspace Cargo (crates/* + web/src-tauri)
├── crates/
│   ├── educator-core/      # Domaine + moteur d'exercices — code pur, aucune dépendance d'infrastructure
│   ├── educator-content/   # Chargement / validation du contenu (JSON, SVG) + Piper (bureau)
│   └── educator-data/      # SQLite : base, migrations, profils, réglages
├── content/                # Contenu pédagogique (règles d'exercices, gabarits de problèmes, personnages, calendrier, points) + schémas JSON
├── web/                    # Front React + application Tauri (src-tauri/), voir web/README.md
│   └── src/api/types.ts    # Types des commandes, générés depuis web/src-tauri/src/contracts.rs
├── scripts/                # fetch-piper.sh / .ps1, compare-voices.sh
├── vendor/piper/           # Moteur Piper + voix, téléchargés par scripts/fetch-piper (non commités)
├── docs/adr/               # Décisions d'architecture (0001 remplacée par 0003, 0002 voix Piper, 0003 backend Rust)
├── ghpm/                   # Backlog en fiches (voir ci-dessous)
└── CLAUDE.md
```

Règle de dépendance : `educator-core` ne dépend d'aucune crate d'infrastructure (seulement `rand`, `rand_chacha`,
`uuid`, `thiserror`) ; `educator-data` et `educator-content` → `core` (pas l'un de l'autre) ; l'application → toutes.
Vérifiée par `crates/educator-core/tests/architecture.rs`.

## Conventions

- **Code** (types, fonctions, variables, commentaires, messages de commit techniques) : en **anglais**.
- **Documentation** (README, ADR, fiches) et **communication avec Georges** : en **français**.
- **Textes affichés à l'enfant** : en français, centralisés dans un fichier de ressources du front, jamais en dur
  dans les composants. Seule exception : les messages d'erreur de profil (`educator-core::profiles`), affichés tels quels.
- Rust : édition 2021, `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` (warnings en erreurs en CI),
  `unsafe` interdit ; versions des dépendances partagées dans `[workspace.dependencies]` du `Cargo.toml` racine.
- Traits du `core` **synchrones** (SQLite et Piper sont bloquants) ; les commandes Tauri les exécutent sur un thread
  de travail (`spawn_blocking`).
- Erreurs du domaine : `educator_core::Error` ; les commandes renvoient un `CommandError { status, title, detail, field }`
  qui garde la signification HTTP de l'ancienne API (400, 404, 409, 503).
- **Pas de logique métier dans le front** : il affiche, saisit et appelle les commandes. Génération des exercices,
  validation des réponses, points, règles d'orthographe : dans les crates Rust.
- **Pas de règle métier dans `web/src-tauri`** : adaptateurs fins (`services.rs`) entre les commandes et les crates.
- Types TypeScript des commandes **générés** (`npm run api:generate`), jamais écrits à la main.
- Le **contenu** (règles, mots, leçons, barèmes) vit dans des fichiers validés par schéma JSON : une règle mal écrite
  doit faire échouer les tests, jamais planter devant l'enfant.
- Pas de bibliothèque de composants généraliste : composants enfant maison (gros boutons, lisibilité, animations).

## Règles pédagogiques à respecter

- Contenu conforme aux programmes et repères annuels officiels (eduscol / Éducation nationale).
- Orthographe : **aucune tolérance sur les accents** ; majuscules tolérées pour l'instant.
- Retours bienveillants : jamais de message culpabilisant pour l'enfant.

## Commandes Tauri

- Déclarées dans `web/src-tauri/src/commands.rs`, implémentées dans `services.rs`, contrats dans `contracts.rs`
  (camelCase, énumérations aux noms de l'ancienne API : `"Keyboard"`, `"CE1"`).
- `get_startup` (le front l'interroge jusqu'à `ready` / `failed`), `get_status`, `get_skills`, `get_characters`,
  `draw_character`, `get_character_svg`, `list_profiles`, `create_profile`, `update_profile`, `select_profile`,
  `reroll_character`, `delete_profile`, `start_series`, `answer_series` (réponse typée : nombre, fraction, signe,
  oui / non, suite, ensemble, texte), `abandon_series`, `get_multiplication_table` (« Mes tables »), `speak` (WAV en
  binaire, `503` si la voix est indisponible).
- Côté front : `web/src/api/ui.ts` (`UiApi`, injectée par `BackendContext`, simulée dans les tests).
- Après toute modification d'un contrat : `cd web && npm run api:generate`, puis commiter `web/src/api/types.ts`
  (le test `bindings::types_ts_is_up_to_date` le vérifie).

## Synthèse vocale (Piper)

Voir `docs/adr/0002-voix-piper.md`. Le trait `SpeechSynthesizer` (`educator-core`) est implémenté par
`PiperSpeechSynthesizer` (`educator-content::speech`, compilé seulement sur le bureau) : un processus
`piper --json-input` longue durée, préchauffé au démarrage, avec un cache disque (`SpeechCache`).

- Installation en dev : `scripts/fetch-piper.sh` (macOS / Linux) ou `scripts/fetch-piper.ps1` (Windows).
- Sur macOS, Piper est un binaire Intel : Rosetta 2 est nécessaire sur Apple Silicon.
- Variables : `EDUCATOR_PIPER_DIR` (racine Piper), `EDUCATOR_PIPER_VOICE` (défaut `fr_FR-siwis-medium`).
- Sans Piper, l'application fonctionne sans le son (la barre d'état indique pourquoi).

## Variables d'environnement

| Variable | Rôle |
|---|---|
| `EDUCATOR_DATA_DIR` | Dossier de données (base, logs) au lieu du dossier standard — tests, débogage |
| `EDUCATOR_PIPER_DIR`, `EDUCATOR_PIPER_VOICE` | Emplacement de Piper et voix utilisée |
| `EDUCATOR_CONTENT_DIR` | Dossier du contenu pédagogique au lieu de la copie embarquée (ressource Tauri) ou du `content/` du dépôt |

## Stockage local (SQLite)

Base `educator.db` dans le dossier de données de l'application :
macOS `~/Library/Application Support/Educator/`, Windows `%APPDATA%\Educator\`. Logs dans le sous-dossier `logs/`.

Conventions :
- Une connexion partagée (`educator_data::Database`) derrière un mutex.
- Migrations au démarrage (`crates/educator-data/src/migrations.rs`, version = `PRAGMA user_version`), sauvegarde
  `VACUUM INTO` avant migration d'une base existante, mode WAL. Une migration publiée n'est jamais modifiée : en ajouter une.
- Identifiants Guid v7 (texte en majuscules) ; tables au singulier en PascalCase ; dates UTC en millisecondes Unix ;
  enums en texte. Suffixe `At` pour les instants UTC.
- `CreatedAt` / `UpdatedAt` renseignés par les stores à partir du `Clock` injecté ; `UpdatedAt` ne bouge que si une
  colonne change.
- Une base créée par l'ancienne version .NET est reprise avec ses profils (ADR 0003).

## Commandes

Georges lance lui-même les builds, tests et l'application sur son Mac. Quand un changement est prêt, indiquer
les commandes à exécuter et ce qu'elles doivent produire.

```bash
scripts/fetch-piper.sh                 # une fois : moteur Piper + voix dans vendor/piper
cargo test --workspace                 # tous les tests Rust
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
cd web && npm install
cd web && npm run tauri dev            # application complète
cd web && npm run api:generate         # après une modification des contrats
cd web && npm run build && npm test && npm run test:e2e
```

La CI (`.github/workflows/ci.yml`) lance `cargo fmt --check`, `clippy` et `cargo test` sur macOS et Windows,
compile et teste le front sur macOS et Windows, et lance Playwright sur Linux.

## Backlog (fiches ghpm)

Le backlog est géré en fiches Markdown dans `ghpm/specs/`, synchronisées avec le projet GitHub
`georgesp/projects/7` par GitHubProjectManager (format décrit dans `ghpm/README.md`).

- Lots : 0 socle technique, 1 socle mathématiques, 1bis migration du backend vers Rust, 2 orthographe / leçons /
  examens, 3 motivation et espace parent, 4 / 5 / 6 exercices et leçons du CE2 / CM1 / CM2 (maths, orthographe, grammaire) ; packaging et signature dans un lot dédié.
- Ne jamais modifier les clés « Managed by GHPM » de l'en-tête d'une fiche.
- Les codes (`US0060`, `SUB006001`…) sont attribués par GHPM à la synchronisation et ne changent plus.
- En implémentant une fiche : respecter ses critères d'acceptation, cocher ceux qui sont remplis, passer `status`
  à `In progress` puis `Done`.

## Règles de travail

- Avant de modifier un fichier existant (hors code en cours d'écriture pour la fiche), annoncer le changement
  et attendre la confirmation de Georges.
- Ne jamais supprimer de fichier sans confirmation explicite.
- Ne jamais commiter ni pousser sans demande explicite.
- Licences : toute dépendance embarquée (Piper, voix, polices, images) est consignée dans `THIRD-PARTY-NOTICES.md`.
