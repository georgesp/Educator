# Front et application Tauri

Front React + TypeScript (Vite) et application Tauri 2 (`src-tauri/`). Architecture : `docs/adr/0003-backend-rust.md`.

## Lancer l'application

```bash
npm install
npm run tauri dev
```

Tauri compile le workspace Cargo (premier lancement plus long), ouvre la fenêtre et charge le contenu et la base
en arrière-plan : l'écran « Démarrage… » s'affiche une fraction de seconde. Il n'y a plus de serveur local : le front
appelle des commandes Tauri (`src/api/ui.ts` → `src-tauri/src/commands.rs`).

Le front seul dans un navigateur (`npm run dev`) affiche un message d'erreur : les commandes n'existent que dans la
fenêtre Tauri. Pour travailler l'interface sans l'application, utiliser les tests (commandes simulées).

## Types des commandes

Les types TypeScript (`src/api/types.ts`) sont **générés** depuis `src-tauri/src/contracts.rs`, jamais écrits à la
main :

```bash
npm run api:generate   # cargo test : régénère src/api/types.ts s'il n'est pas à jour
```

À relancer après toute modification d'un contrat, puis commiter `src/api/types.ts`. Le test
`bindings::types_ts_is_up_to_date` et la CI échouent sinon (le premier lancement régénère le fichier et échoue,
le second passe).

## Tests

```bash
npm test            # Vitest (composants)
npm run test:e2e    # Playwright (parcours, commandes Tauri simulées dans la page) ;
                    # la première fois : npx playwright install chromium
```

## Page de démonstration des composants

En développement (`npm run tauri dev`), la route `#/dev/composants` montre tous les composants enfant (`src/components/child/`) sur
une seule page. Elle n'existe pas dans le build de l'application.
