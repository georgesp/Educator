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

L'architecture reprend celle de **GitHubProjectManager** (`../GitHubProjectManager`), allégée : s'y référer en cas de
doute (ADR 0001 stockage SQLite, ADR 0002 front Tauri, `web/src-tauri/src/backend.rs`, `Directory.Build.props`).

## Stack technique

| Couche | Technologie |
|---|---|
| Coque desktop | Tauri 2 (Rust) — `web/src-tauri/` — **aucune logique métier** |
| Front | React 19 + TypeScript (Vite), TanStack Query, Framer Motion — `web/` |
| Backend | `Educator.Host` (ASP.NET Core Minimal API), lancé par Tauri comme processus annexe |
| Runtime | .NET 10, C# dernière version |
| Stockage | SQLite + EF Core 10 (`Educator.Data`) |
| Synthèse vocale | Piper (binaire + voix française embarqués) |
| Logs | Microsoft.Extensions.Logging + Serilog (fichier) |
| Tests .NET | xUnit + NSubstitute (catégories `Unit` / `Integration`) |
| Tests front | Vitest (composants), Playwright (parcours) |

## Structure cible

```
Educator/
├── src/
│   ├── Educator.Core/      # Domaine + moteur d'exercices — code pur, aucune dépendance d'infrastructure
│   ├── Educator.Data/      # EF Core, AppDbContext, entités, migrations
│   ├── Educator.Content/   # Chargement / validation du contenu (JSON, SVG, mots, leçons) + Piper
│   └── Educator.Host/      # API locale /api/ui, injection de dépendances, démarrage, logs
├── tests/                  # Un projet de tests par projet src
├── content/                # Contenu pédagogique (règles d'exercices, mots, leçons, SVG) + schémas JSON (lot 1)
├── web/                    # Front React + coque Tauri (src-tauri/), voir web/README.md
│   └── openapi/openapi.json    # Description de l'API exportée par le host (source du client TS généré)
├── scripts/                # fetch-piper.sh / .ps1, compare-voices.sh
├── vendor/piper/           # Moteur Piper + voix, téléchargés par scripts/fetch-piper (non commités)
├── docs/adr/               # Décisions d'architecture (0001 architecture, 0002 voix Piper)
├── ghpm/                   # Backlog en fiches (voir ci-dessous)
├── Directory.Build.props
├── Educator.sln
└── CLAUDE.md
```

Règle de dépendance : `Core` ne référence rien ; `Data` et `Content` → `Core` ; `Host` → tous. Vérifiée par un test
d'architecture.

## Conventions

- **Code** (types, méthodes, variables, commentaires, messages de commit techniques) : en **anglais**.
- **Documentation** (README, ADR, fiches) et **communication avec Georges** : en **français**.
- **Textes affichés à l'enfant** : en français, centralisés dans un fichier de ressources du front, jamais en dur
  dans les composants.
- Namespace racine `Educator` ; conventions Microsoft C# ; nullable activé ; warnings traités comme erreurs.
- Versions NuGet **centralisées** dans `Directory.Packages.props` : jamais de `Version=` dans un `.csproj`.
- Tests : xUnit + NSubstitute, `Assert` de xUnit (pas de FluentAssertions, dont la v8 est sous licence commerciale) ;
  chaque classe porte `[Trait("Category", "Unit")]` ou `[Trait("Category", "Integration")]`.
- Toutes les I/O sont `async`.
- **Pas de logique métier dans le front** : il affiche, saisit et appelle l'API. Génération des exercices, validation
  des réponses, points, règles d'orthographe : côté .NET.
- **Pas de logique métier dans Rust** : la coque gère la fenêtre et le cycle de vie du host, rien d'autre.
- Client API du front **généré** depuis l'OpenAPI du host (`npm run api:generate`), jamais écrit à la main.
- Le **contenu** (règles, mots, leçons, barèmes) vit dans des fichiers validés par schéma JSON : une règle mal écrite
  doit faire échouer le build ou les tests, jamais planter devant l'enfant.
- Pas de bibliothèque de composants généraliste : composants enfant maison (gros boutons, lisibilité, animations).

## Règles pédagogiques à respecter

- Contenu conforme aux programmes et repères annuels officiels (eduscol / Éducation nationale).
- Orthographe : **aucune tolérance sur les accents** ; majuscules tolérées pour l'instant.
- Retours bienveillants : jamais de message culpabilisant pour l'enfant.

## API locale

- Écoute en loopback (`127.0.0.1`) sur un port libre ; le host écrit `{ baseUrl, token }` sur sa sortie standard,
  lu par Tauri et transmis au front.
- Toutes les routes `/api/ui/*` exigent l'en-tête `X-Local-Token` (comparaison à temps constant, jamais journalisé).
- CORS limité à `http://localhost:1421`, `tauri://localhost`, `http://tauri.localhost`.
- Erreurs au format problem+json. OpenAPI : `GET /openapi/v1.json`.
- Pas de SSE (pas d'événement temps réel à pousser).
- Port libre choisi par l'OS (`--port 0`) ; le token vient de `EDUCATOR_UI_TOKEN` (aléatoire si absent).
- Endpoints (lot 0) : `GET /status`, `POST /exercises/additions`, `POST /exercises/additions/{id}/answer`,
  `GET /exercises/additions/{id}/speech` (WAV, `503` si la voix est indisponible).
- DTO dans `Educator.Host/Api/Contracts.cs`, toutes les propriétés `required` (schéma OpenAPI exact).
  Après toute modification d'un endpoint ou d'un DTO : `cd web && npm run api:generate`, puis commiter
  `web/openapi/openapi.json` et `web/src/api/schema.d.ts` (le test `CommittedOpenApiFile_MatchesTheApi` le vérifie).
- Le host prend aussi `--parent-pid` (il s'arrête si la coque disparaît) et `--export-openapi <fichier>`.

## Synthèse vocale (Piper)

Voir `docs/adr/0002-voix-piper.md`. `ISpeechSynthesizer` (Core) est implémenté par `PiperSpeechSynthesizer`
(Content) : un processus `piper --json-input` longue durée, préchauffé au démarrage du host.

- Installation en dev : `scripts/fetch-piper.sh` (macOS / Linux) ou `scripts/fetch-piper.ps1` (Windows).
- Sur macOS, Piper est un binaire Intel : Rosetta 2 est nécessaire sur Apple Silicon.
- Variables : `EDUCATOR_PIPER_DIR` (racine Piper), `EDUCATOR_PIPER_VOICE` (défaut `fr_FR-siwis-medium`).
- Sans Piper, l'application fonctionne sans le son (`/status` indique pourquoi).

## Variables d'environnement

| Variable | Rôle |
|---|---|
| `EDUCATOR_UI_TOKEN` | Token de l'API locale (fixé par la coque Tauri) |
| `EDUCATOR_BACKEND_URL` | Avec `EDUCATOR_UI_TOKEN` : la coque utilise un host déjà lancé au lieu d'en démarrer un |
| `EDUCATOR_DATA_DIR` | Dossier de données (base, logs) au lieu du dossier standard — tests, débogage |
| `EDUCATOR_PIPER_DIR`, `EDUCATOR_PIPER_VOICE` | Emplacement de Piper et voix utilisée |

## Stockage local (SQLite)

Base `educator.db` dans le dossier de données de l'application :
macOS `~/Library/Application Support/Educator/`, Windows `%APPDATA%\Educator\`. Logs dans le sous-dossier `logs/`.

Conventions (reprises de l'ADR 0001 de GitHubProjectManager) :
- Toujours `IDbContextFactory<AppDbContext>` (contexte court par opération).
- `MigrateAsync` au démarrage, sauvegarde `VACUUM INTO` avant migration d'une base existante, mode WAL.
- Identifiants Guid v7 ; tables au singulier en PascalCase ; dates UTC en millisecondes Unix ; enums en texte.
- `CreatedAt` / `UpdatedAt` renseignés par un intercepteur alimenté par `TimeProvider`.
- Suffixe `At` pour les instants UTC.

Migrations :
```bash
dotnet tool restore
dotnet ef migrations add <NomDeLaMigration> --project src/Educator.Data
```

## Commandes

Georges lance lui-même les builds, tests et la coque Tauri sur son Mac. Quand un changement est prêt, indiquer
les commandes à exécuter et ce qu'elles doivent produire.

```bash
scripts/fetch-piper.sh                 # une fois : moteur Piper + voix dans vendor/piper
dotnet build
dotnet test                            # --filter "Category=Unit" pour les tests unitaires seuls
cd web && npm install
cd web && npm run tauri dev            # application complète
cd web && npm run api:generate         # après une modification de l'API
cd web && npm run build && npm test && npm run test:e2e
```

La CI (`.github/workflows/ci.yml`) compile et teste .NET, le front et la coque (clippy) sur macOS et Windows,
et lance Playwright sur Linux.

## Backlog (fiches ghpm)

Le backlog est géré en fiches Markdown dans `ghpm/specs/`, synchronisées avec le projet GitHub
`georgesp/projects/7` par GitHubProjectManager (format décrit dans `ghpm/README.md`).

- Lots : 0 socle technique, 1 socle mathématiques, 2 orthographe / leçons / examens, 3 motivation et espace parent,
  4 extension jusqu'au CM2 ; packaging et signature dans un lot dédié.
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
