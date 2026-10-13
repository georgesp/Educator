---
type: Issue
title: Zone d'écriture au stylet (canvas, rejet de la paume, effacer)
status: Todo
size: L
parent: specs/lot-tablette-2-stylet-et-capture-de-l-ecriture.md
order: 4020
---

## User story

En tant qu'enfant, je veux écrire mon mot au stylet sur une ligne, comme sur mon cahier, et pouvoir effacer si je me trompe.

## Dépend de

Lot tablette 1 ; peut démarrer avant la fin de l'étude (fiche 01), la capture étant la même pour toutes les voies.

## Comportement attendu

- Composant enfant maison (`web/src/components/…`), sans bibliothèque de dessin généraliste : un canvas qui capture les tracés par Pointer Events (`pointerType === "pen"`, coordonnées, temps, pression), lisse le trait et l'affiche.
- Rejet de la paume : seuls les contacts du stylet écrivent par défaut (le doigt fait défiler ou tape les boutons) ; le doigt peut écrire si aucun stylet n'est détecté (réglage de la fiche 10). `touch-action: none` sur la zone seulement.
- Aspect cahier : lignes de guidage (interlignes de type Seyès), grande zone, trait épais lisible, couleur de la charte.
- Actions : effacer le dernier trait, tout effacer, valider ; boutons de gomme du stylet reconnus si disponibles (à vérifier selon le matériel).
- Modèle de tracé (liste de traits, chaque trait = points `x, y, t, pression`) défini côté Rust (`educator-core`) et exposé via les contrats générés ; le front ne fait que capturer et afficher.
- Accessible : alternative clavier toujours visible.

## Critères d'acceptation

- [ ] Écriture fluide (pas de retard visible) sur la tablette de test ; paume posée sans effet.
- [ ] Tests Vitest du composant (simulation d'événements de pointeur) ; le tracé envoyé au Rust est complet et ordonné.
- [ ] Sur le bureau, le composant n'apparaît pas (ou fonctionne à la souris si activé en réglage de débogage).
