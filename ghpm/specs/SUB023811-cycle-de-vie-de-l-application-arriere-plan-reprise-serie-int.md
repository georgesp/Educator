---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsEBY
url: https://github.com/georgesp/Educator/issues/250
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023811 — Cycle de vie de l'application (arrière-plan, reprise, série interrompue)"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#238"
order: 3110
---

## User story

En tant qu'enfant, je veux pouvoir répondre à un appel ou changer d'application et retrouver mon exercice sans perdre mes points.

## Contexte

Sur tablette, le système peut suspendre l'application à tout moment, puis la fermer sans prévenir. Les séries en cours vivent **en mémoire** (50 au plus, derrière un `Mutex`) : une application tuée perd la série en cours. Le défi chrono compte du temps qui ne doit pas courir en arrière-plan.

## Comportement attendu

- Passage en arrière-plan : la voix s'arrête, le chrono du défi est mis en pause et reprend au retour (ou la série est abandonnée proprement : à trancher).
- Série perdue (application relancée) : retour au menu avec un message bienveillant ; les réponses déjà enregistrées (`Session`, `Answer`) sont conservées. Étudier si les séries en cours doivent être persistées (coût, intérêt) : décision et justification dans l'ADR 0005.
- Écran qui reste allumé pendant une série / une dictée (à trancher), mise en veille normale ailleurs.
- Fermeture : base SQLite fermée proprement (WAL).

## Critères d'acceptation

- [ ] Scénarios testés sur la tablette : appel entrant, bascule vers une autre application pendant une série, pendant un défi chrono, fermeture forcée puis relance.
- [ ] Aucun message culpabilisant ; aucun plantage ; aucune base corrompue.
- [ ] Tests unitaires Rust des cas de série introuvable après redémarrage (erreur `404` gérée par le front).
