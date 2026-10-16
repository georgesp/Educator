---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsEBY
rev: 2026-10-15T20:19:07Z
url: https://github.com/georgesp/Educator/issues/250
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023811 — Cycle de vie de l'application (arrière-plan, reprise, série interrompue)"
status: In Review
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

## Note (2026-10-15)

- Déjà fait dans SUB023806 : la voix s'arrête quand l'application passe en arrière-plan, et la session audio est réactivée au retour.

## Implémentation (2026-10-15)

- Décisions dans l'ADR 0006 (« Cycle de vie ») : séries en cours **non persistées** (front et cœur repartent ensemble ; réponses déjà enregistrées) ; veille normale de l'écran (à revoir à la recette) ; SQLite en WAL résiste à une fermeture brutale.
- Temps d'une question (examen, évaluation) **en pause** en arrière-plan (`QuestionTimer`, `web/src/app/visibility.ts`).
- Défi chrono : le temps réel continue, comme le cœur qui refuse les réponses après la fin ; au retour, un défi terminé s'arrête aussitôt avec ses résultats (`TimerBar`). Tests `TabletTimers.test.tsx`.
- Voix arrêtée en arrière-plan, session audio réactivée (SUB023806).
- Série introuvable : 404 déjà couvert par `series_errors_map_to_400_404_and_409`, message bienveillant habituel côté front.

## Corrections après relecture (2026-10-15)

- Accepté le 2026-10-15 : sur l'ordinateur aussi, la voix s'arrête et le temps d'une question se met en pause quand la fenêtre est réduite ou cachée (noté dans l'ADR 0006).
- À vérifier sur le Mac : réduire la fenêtre pendant une question d'examen chronométrée, revenir : le temps n'a pas avancé.

## Tests à faire (revue)

1. Examen avec chrono par question (Options › chrono de l'examen) : pendant une question, passer 30 s dans une autre app, revenir → le temps restant est le même qu'au départ (en pause).
2. Défi chrono de 1 min : au bout de 20 s, passer 2 min dans une autre app, revenir → le défi se termine aussitôt avec ses résultats, sans message d'erreur.
3. Défi chrono : passer 10 s ailleurs puis revenir → le défi continue (le temps a couru), réponses acceptées.
4. Série en cours : fermer Educator de force (balayage vers le haut), relancer → « Qui joue ? », pas de plantage ; dans l'espace parent, les réponses déjà données de la série figurent dans les statistiques.
5. Appel FaceTime pendant une dictée → au retour, « Écouter le mot » fonctionne.
6. `npm test` → `TabletTimers.test.tsx` (2) vert.
