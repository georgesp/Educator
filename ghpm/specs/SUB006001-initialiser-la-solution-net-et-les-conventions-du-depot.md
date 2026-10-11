---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5fKg
url: https://github.com/georgesp/Educator/issues/61
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB006001 — Initialiser la solution .NET et les conventions du dépôt
status: In progress
size: S
estimate:
startDate:
endDate:
parent: "#60"
order: 2
---

## User story

En tant que développeur, je veux une solution .NET structurée et outillée dès le départ afin que chaque lot s'appuie sur des fondations propres.

## Contenu

- `Educator.sln` avec 4 projets : `Educator.Core`, `Educator.Data`, `Educator.Content`, `Educator.Host`.
- Un projet de tests par projet (xUnit + NSubstitute), catégories `Unit` / `Integration`.
- `Directory.Build.props` repris de GitHubProjectManager (net10.0, nullable, warnings en erreurs, analyseurs).
- `.editorconfig`, `.gitignore`, `README.md`, `CLAUDE.md` / `AGENTS.md` (code en anglais, documentation en français).
- `docs/adr/0001-architecture.md` : reprise de l'architecture GitHubProjectManager et justification du découpage en 4 projets (au lieu de 6).

## Critères d'acceptation

- [x] `dotnet build` et `dotnet test` passent sans avertissement.
- [x] Règle de dépendance vérifiée par un test d'architecture : `Core` ne référence aucun autre projet ni package d'infrastructure.
- [x] ADR 0001 rédigée.

## Implémentation (2026-10-10)

Vérifié dans l'environnement Linux de Claude (.NET 10.0.112) : build Debug et Release sans avertissement. Versions NuGet centralisées (`Directory.Packages.props`), `global.json`. FluentAssertions non repris (licence commerciale en v8).
