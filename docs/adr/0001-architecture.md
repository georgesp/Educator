# ADR 0001 — Architecture d'Educator

- **Statut** : acceptée
- **Date** : 2026-10-10
- **Fiche** : SUB006001 (lot 0 — socle technique)

## Contexte

Educator est une application de bureau gratuite, pour macOS et Windows, destinée aux enfants du CP au CM2.
Elle fonctionne entièrement hors ligne : profils locaux, exercices générés en direct, synthèse vocale locale (Piper),
statistiques pour les parents.

Georges a déjà construit **GitHubProjectManager** (GHPM) sur une architecture éprouvée : coque Tauri 2, front React +
TypeScript, backend .NET 10 lancé comme processus annexe et exposant une API HTTP locale, stockage SQLite / EF Core
(ADR 0001 et 0002 de GHPM). Réutiliser cette architecture permet de partir d'un socle connu et de recopier le code
d'infrastructure (lancement du host, token, CORS, migrations).

## Décision

Educator reprend l'architecture de GHPM : une **coque Tauri 2** sans logique métier, un **front React + TypeScript**
qui affiche et saisit, et un **host .NET 10** (`educator-host`) qui porte toute la logique et l'expose par une API
locale `/api/ui` protégée par un token.

La solution .NET est découpée en **4 projets** au lieu des 6 de GHPM :

| Projet | Rôle | Dépend de |
|---|---|---|
| `Educator.Core` | Domaine et moteur d'exercices, code pur et testable | rien |
| `Educator.Data` | EF Core, SQLite, migrations | `Core` |
| `Educator.Content` | Chargement et validation du contenu (règles, mots, leçons, SVG), intégration de Piper | `Core` |
| `Educator.Host` | API locale, injection de dépendances, démarrage, logs | tous |

Différences avec GHPM :

- **Pas de projet `Infrastructure`** : Educator ne parle à aucun service distant. Les seules entrées / sorties hors
  base sont le contenu pédagogique et Piper, regroupés dans `Content`.
- **`Composition` et `LocalApi` fusionnés dans `Host`** : un seul exécutable les utilise ; les séparer n'apporterait
  qu'une indirection.
- **Pas de SSE** : aucun événement en temps réel à pousser vers le front.
- **Contenu validé par schéma JSON** : une règle d'exercice mal écrite doit faire échouer les tests, jamais planter
  devant l'enfant.

Outillage commun : `Directory.Build.props` (net10.0, nullable, warnings en erreurs, analyseurs), versions NuGet
centralisées dans `Directory.Packages.props`, xUnit + NSubstitute avec les catégories `Unit` / `Integration`.

## Conséquences

- La règle de dépendance est vérifiée par un test d'architecture (`tests/Educator.Host.Tests/Architecture`), à la fois
  sur les fichiers projet et sur les assemblys compilés : `Core` ne référence aucun projet ni paquet d'infrastructure.
- Le moteur d'exercices, au cœur du produit, se teste sans base, sans HTTP et sans Piper.
- Si un projet grossit trop (par exemple `Content` avec Piper et l'audio), il pourra être redécoupé plus tard sans
  toucher à `Core`.
- Toute évolution utile de l'infrastructure de GHPM (lancement du host, migrations) doit être reportée à la main :
  il n'y a pas de socle partagé entre les deux dépôts.
