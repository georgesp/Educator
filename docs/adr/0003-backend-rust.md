# ADR 0003 — Backend en Rust, dans l'application Tauri

- **Statut** : acceptée
- **Date** : 2026-10-11
- **Fiche** : lot 1bis — Migration du backend .NET vers Rust
- **Remplace** : [ADR 0001](0001-architecture.md) (architecture à host .NET)

## Contexte

Une version tablette (iPad, Android) d'Educator est envisagée dans un lot ultérieur. Sur iOS, une application ne
peut pas lancer de processus annexe : le host .NET `educator-host`, lancé par la coque Tauri et joint par une API
HTTP locale, ne peut donc pas être embarqué. Sur Android, ce serait possible mais lourd (runtime .NET à embarquer).

Au moment de la décision, le code se limite aux lots 0 et 1 (environ 4 600 lignes de C# et 2 500 lignes de tests).
Chaque fiche du lot 2 écrite en C# aurait alourdi une migration devenue inévitable.

## Décision

Toute la logique passe dans le cœur Rust de l'application Tauri. Il n'y a plus de serveur, de port, de token ni de
processus annexe (hors Piper sur le bureau) : le front appelle des **commandes Tauri** (`invoke`).

Le workspace Cargo (racine du dépôt) reprend le découpage des projets .NET :

| Crate | Rôle | Dépend de |
|---|---|---|
| `crates/educator-core` | Domaine et moteur d'exercices, code pur et testable | rien (hors `rand`, `rand_chacha`, `uuid`, `thiserror`) |
| `crates/educator-content` | Chargement et validation du contenu ; Piper sur le bureau | `core` |
| `crates/educator-data` | SQLite (`rusqlite`, SQLite embarqué), migrations, profils et réglages | `core` |
| `web/src-tauri` (`educator-shell`) | Commandes Tauri, état partagé, démarrage, logs | tous |

Même règle de dépendance qu'à l'ADR 0001, vérifiée par `crates/educator-core/tests/architecture.rs` (sur
`cargo metadata`) : `core` ne dépend d'aucune crate d'infrastructure ; `data` et `content` dépendent de `core`
et pas l'un de l'autre.

Principes retenus :

- **Traits synchrones** dans `core` (`ProfileStore`, `AppSettingsStore`, `SkillCatalog`, `CharacterCatalog`,
  `SpeechSynthesizer`, `RandomSource`, `Clock`) : SQLite et Piper sont bloquants ; chaque commande Tauri s'exécute
  sur un thread de travail (`spawn_blocking`).
- **Séries en cours en mémoire**, derrière un `Mutex` (50 au plus), comme le `ConcurrentDictionary` du C#.
- **Tirages reproductibles** : le moteur utilise ChaCha8 (`rand_chacha`) à partir de la graine de la série. Les
  tirages diffèrent de ceux de `System.Random` : les tests vérifient des propriétés, pas des valeurs recopiées.
- **Contrats** (`web/src-tauri/src/contracts.rs`) : mêmes champs et mêmes valeurs d'énumération que l'ancienne API
  (`"Keyboard"`, `"CE1"`), en camelCase. Les types TypeScript (`web/src/api/types.ts`) sont générés par `ts-rs`
  (`npm run api:generate`) ; un test échoue si le fichier commité n'est pas à jour.
- **Erreurs** : une commande rejette un `CommandError { status, title, detail, field }` qui garde la signification
  HTTP de l'ancienne API (400, 404, 409, 503) ; le front n'a pas changé de logique d'affichage.
- **Démarrage** : le contenu et la base sont chargés dans un thread au lancement ; le front interroge
  `get_startup` jusqu'à `ready` (ou `failed`, avec le message). La voix est préchauffée ensuite.
- **Voix** : `educator-content::speech` (module compilé seulement hors iOS / Android) reprend l'ADR 0002 à
  l'identique : processus `piper --json-input` longue durée, relancé si la vitesse change ou s'il s'arrête, cache
  disque des phrases. Sur tablette, le synthétiseur « indisponible » du `core` est utilisé en attendant le lot dédié.
- **Logs** : `tauri-plugin-log`, dans la console et dans `logs/educator.log` du dossier de données.

## Base existante

La base `educator.db` créée par la version .NET est **reprise** : la migration 1 (`0001_initial`) recrée exactement
le schéma laissé par EF Core (`InitialCreate` + `AddProfile`) avec `IF NOT EXISTS`. Une base .NET (version 0 pour
`PRAGMA user_version`) est sauvegardée (`VACUUM INTO`), puis adoptée avec ses profils ; la table
`__EFMigrationsHistory` est ignorée. Les identifiants restent du texte Guid en majuscules, les dates des
millisecondes Unix, les énumérations du texte. L'application n'étant pas diffusée, repartir d'une base neuve aurait
été acceptable, mais la reprise ne coûte qu'une migration écrite avec `IF NOT EXISTS`.

Les migrations suivantes s'ajoutent à la liste `MIGRATIONS` de `crates/educator-data/src/migrations.rs` ; la version
de la base est `PRAGMA user_version`. Une migration publiée n'est jamais modifiée.

## Conséquences

- `npm run tauri dev` suffit : plus de .NET à installer, une seule chaîne de build (Cargo + npm).
- L'application peut tourner sur iOS et Android sans changer d'architecture (lot tablette : adaptation tactile,
  build mobile, voix).
- Le scénario Playwright simule les commandes Tauri dans la page (`window.__TAURI_INTERNALS__`) au lieu d'intercepter
  des requêtes HTTP.
- Le contenu (`content/`) est embarqué comme ressource Tauri (`bundle.resources`) ; en développement, il est lu dans
  le dépôt (ou `EDUCATOR_CONTENT_DIR`).
