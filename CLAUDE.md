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
serveur local ni de token. Cela permet la version **iPad** (lot tablette 1, ADR 0006 : iPad seul, paysage seul,
iPadOS 17 minimum), dans le même dépôt : iOS n'autorise pas de processus annexe.

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
├── content/                # Contenu pédagogique (règles d'exercices, gabarits de problèmes, banque de mots, sons, homophones, leçons, mascotte, personnages, calendrier, points) + schémas JSON
│                           # Lot 3 : rewards/ (niveaux, étoiles, objectif du jour, badges + icônes), accessories/ (accessoires d'avatar + planche),
│                           # progress/ (seuils de la difficulté adaptative, intervalles des révisions), themes/ (liste des thèmes)
├── web/                    # Front React + application Tauri (src-tauri/), voir web/README.md
│   ├── src/api/types.ts    # Types des commandes, générés depuis web/src-tauri/src/contracts.rs
│   └── src-tauri/gen/apple/ # Projet Xcode de la version iPad (`tauri ios init`, commité ; `project.yml` fait foi)
├── scripts/                # fetch-piper.sh / .ps1, compare-voices.sh, listen-words.sh / .ps1
├── vendor/piper/           # Moteur Piper + voix, téléchargés par scripts/fetch-piper (non commités)
├── docs/adr/               # Décisions d'architecture (0001 remplacée par 0003, 0002 voix Piper, 0003 backend Rust, 0004 voix en direct pour les mots, 0005 voix sur tablette — étude, 0006 application tablette (iPad)) ; `docs/tablette.md` : préparer le Mac et l'iPad
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
- Thèmes (SUB001513) : aucune couleur, police ni forme en dur dans les composants ; tout passe par les variables des
  blocs `[data-theme="…"]` de `web/src/styles.css`. Un nouveau thème s'ajoute à `content/themes/themes.json` et
  redéfinit toutes les variables du thème `sport` (vérifié, avec le contraste AA, par `web/src/themes.test.ts`).

## Règles pédagogiques à respecter

- Contenu conforme aux programmes et repères annuels officiels (eduscol / Éducation nationale).
- Orthographe : **aucune tolérance sur les accents** ; majuscules tolérées pour l'instant ; « oe » ne remplace pas « œ » (le clavier à l'écran le propose).
- Retours bienveillants : jamais de message culpabilisant pour l'enfant.

## Commandes Tauri

- Déclarées dans `web/src-tauri/src/commands.rs`, implémentées dans `services.rs`, contrats dans `contracts.rs`
  (camelCase, énumérations aux noms de l'ancienne API : `"Keyboard"`, `"CE1"`).
- `get_startup` (le front l'interroge jusqu'à `ready` / `failed`), `get_status`, `get_skills`, `get_characters`,
  `draw_character`, `get_character_svg`, `list_profiles`, `create_profile`, `update_profile`, `select_profile`,
  `reroll_character`, `delete_profile`, `start_series`, `answer_series` (réponse typée : nombre, fraction, signe,
  oui / non, suite, ensemble, texte), `abandon_series`, `get_multiplication_table` (« Mes tables »), `speak` (WAV en
  binaire, `503` si la voix est indisponible).
- Lot 2 : `start_exam`, `get_timed_challenge`, `start_timed`, `finish_series` (fin d'un défi chrono seulement),
  `list_lessons`, `get_lesson`, `save_lesson_progress`, `start_lesson_quiz`, `speak_series_word` (mot d'une dictée,
  jamais envoyé au front avant la réponse ; `example` : sa phrase, `slow` : « Plus lentement »), `speak_word`,
  `get_mascot`, `get_mascot_phrase`.
- Lot 3 (espace parent) : `get_parent_gate` (barrière « adulte », tirée à chaque appel), `check_parent_gate`
  (toute réponse consomme la barrière : `educator-core::parent_gate`), `get_profile_statistics` (statistiques d'un
  profil calculées par `educator-core::statistics` à partir des séances et réponses enregistrées),
  `get_parent_profile` / `update_parent_profile` (carte des parents de l'écran « Qui joue ? » : nom et personnage,
  créée par défaut, stockée dans `AppSetting` par `educator-core::parent_profile` ; ce n'est pas un profil enfant).
- Lot 3 (motivation, réglages) : `get_profile_overview` (niveau, objectif du jour, badges, révisions dues : menu),
  `get_badges`, `get_accessories`, `update_avatar` (enregistré à chaque clic, accessoire verrouillé refusé),
  `get_avatar_svg` (avatar composé par `educator-core::rewards::accessories::compose_avatar`), `get_advice`
  (« Je te conseille… », `educator-core::adaptive`), `start_review` (« Mes révisions », `educator-core::reviews`),
  `get_profile_settings` / `update_profile_settings` / `get_settings_options` (réglages de chaque enfant,
  `educator-core::profile_settings`), `export_backup` / `import_backup` (sauvegarde de tous les profils dans un fichier
  choisi avec `tauri-plugin-dialog`, `educator-data::backup`). La fin de chaque séance (`SeriesSummaryResponse.rewards`)
  porte les étoiles, le niveau, l'objectif du jour, les badges et accessoires gagnés (`educator-core::rewards`).
  `speak` et `speak_word` reçoivent la vitesse de voix choisie par l'enfant (`voiceSpeed`).
- Côté front : `web/src/api/ui.ts` (`UiApi`, injectée par `BackendContext`, simulée dans les tests).
- Après toute modification d'un contrat : `cd web && npm run api:generate`, puis commiter `web/src/api/types.ts`
  (le test `bindings::types_ts_is_up_to_date` le vérifie).

## Synthèse vocale (Piper)

Voir `docs/adr/0002-voix-piper.md` et `docs/adr/0004-voix-en-direct-pour-les-mots.md`. Le trait `SpeechSynthesizer`
(`educator-core`) est implémenté par `PiperSpeechSynthesizer` (`educator-content::speech`, compilé seulement sur le
bureau) : un processus `piper --json-input` longue durée **par vitesse** (consignes 0,9 et mots 0,85 préchauffés au
démarrage ; écoute lente 0,5 arrêtée après 2 min sans demande), avec un cache disque plafonné à 30 Mo
(`SpeechCache`, éviction des moins récemment lus).

- Mots d'orthographe lus **en direct**, sans fichier pré-généré : un nom avec son article (« un stylo. »), les autres
  mots lus dans « Le mot : jaune. », coupé à la pause du deux-points (l'enfant n'entend que « jaune ») ; mot et phrase à la même vitesse ; champ `pronunciation` pour corriger une
  prononciation ; un homophone a toujours sa phrase. Les mots d'une série sont préparés en arrière-plan
  (`educator_core::speech::SpeechPreparer`) : la demande de l'enfant passe avant, jamais deux synthèses du même mot.
- Sans voix, la dictée et « Choisis la bonne écriture » sont grisées ; les examens n'ont que des questions jouables
  sans écoute.
- **Rien n'est lu tout seul** (décision du 2026-10-15) : la voix ne part qu'au clic sur un bouton (« Écouter le mot »,
  haut-parleurs `SpeakButton`, qui n'a plus de lecture automatique).
- Écoute de contrôle : `scripts/listen-words.sh` / `.ps1` (`--sample`, `--skill`, `--slow`, `--again`).

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
- Statistiques (lot 2, migration `0002_sessions`) : `Session` (une par série, examen, défi, quiz de leçon ou
  révision), `Answer` (une par réponse, avec sa durée), `SkillMastery` (20 dernières réponses par compétence),
  `LessonProgress` (slide où une leçon a été laissée). Tout est supprimé avec le profil.
- Lot 3 (migration `0004_motivation`) : `Profile.Avatar` (JSON : personnage + accessoires portés et leur couleur ;
  `CharacterId` garde le personnage), `EarnedBadge`, `UnlockedAccessory`, `DailyGoalDay` (jours où l'objectif est
  atteint, bonus donné une fois), `ReviewItem` (file de révision espacée), `ProfileSetting` (réglages de chaque enfant).
  Le niveau et les étoiles ne sont pas stockés : ils sont recalculés à partir des points et des séances.

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
cd web && npm run tauri ios dev "iPad Air 13-inch (M2)"   # version iPad dans le simulateur (docs/tablette.md)
cd web && npm run tauri ios dev -- --open                 # version iPad sur l'iPad, via Xcode (docs/tablette.md)
cd web && npm run api:generate         # après une modification des contrats
scripts/listen-words.sh --sample 50    # écoute de contrôle des mots (après ajout ou modification de mots)
EDUCATOR_WRITE_PLANCHE=1 cargo test -p educator-content --test accessories   # régénère content/accessories/planche.html
cd web && npm run build && npm test && npm run test:e2e
```

La CI (`.github/workflows/ci.yml`) lance `cargo fmt --check`, `clippy` et `cargo test` sur macOS et Windows,
compile et teste le front sur macOS et Windows, et lance Playwright sur Linux.

## Backlog (fiches ghpm)

Le backlog est géré en fiches Markdown dans `ghpm/specs/`, synchronisées avec le projet GitHub
`georgesp/projects/7` par GitHubProjectManager (format décrit dans `ghpm/README.md`).

- Lots : 0 socle technique, 1 socle mathématiques, 1bis migration du backend vers Rust, 2 orthographe / leçons /
  examens, 2bis voix en direct pour les mots, 3 motivation et espace parent, 4 / 5 / 6 exercices et leçons du CE2 / CM1 / CM2 (maths, orthographe, grammaire) ; packaging et signature dans un lot dédié.
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
