# Front et coque Tauri

Front React + TypeScript (Vite) et coque Tauri 2 (`src-tauri/`). Architecture : `docs/adr/0001-architecture.md`.

## Lancer l'application

```bash
npm install
npm run tauri dev
```

La coque lance le host .NET avec `dotnet run` (compilation au premier lancement, d'où quelques secondes
d'attente), lui transmet un token aléatoire et le port libre qu'il a choisi, puis l'arrête à la fermeture.

## Front seul dans un navigateur

Utile pour travailler l'interface avec le rechargement à chaud du navigateur :

```bash
# terminal 1, à la racine du dépôt
EDUCATOR_UI_TOKEN=dev dotnet run --project src/Educator.Host -- --port 5250
# terminal 2, dans web/ (cp .env.example .env.local la première fois)
npm run dev   # http://localhost:1421
```

## Client de l'API

Le client TypeScript est **généré** depuis l'OpenAPI du host, jamais écrit à la main :

```bash
npm run api:generate   # host → openapi/openapi.json → src/api/schema.d.ts
```

À relancer après toute modification d'un endpoint ou d'un DTO, puis commiter les deux fichiers.
Le test .NET `CommittedOpenApiFile_MatchesTheApi` et la CI échouent sinon.

## Tests

```bash
npm test            # Vitest (composants)
npm run test:e2e    # Playwright (parcours, API simulée) ; la première fois : npx playwright install chromium
```

## Page de démonstration des composants

Avec `npm run dev`, la route `#/dev/composants` montre tous les composants enfant (`src/components/child/`) sur
une seule page. Elle n'existe pas dans le build de l'application.
