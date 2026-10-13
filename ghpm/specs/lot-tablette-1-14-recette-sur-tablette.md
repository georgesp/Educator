---
type: Issue
title: Recette sur tablette (parcours complet, performances, rapport)
status: Todo
size: M
parent: specs/lot-tablette-1-application-tablette.md
order: 3140
---

## User story

En tant que Georges, je veux une recette écrite de l'application sur la tablette, pour savoir ce qui marche, ce qui est lent et ce qui reste à corriger avant de la donner à mon enfant.

## Dépend de

Toutes les autres fiches du lot.

## Comportement attendu

- Checklist de recette dans `docs/tablette.md` : un test par écran et par type d'exercice (clavier, QCM, dictée, mots à trous, « Choisis la bonne écriture »), leçons et animations, examen, défi chrono, mascotte, profils, options, voix, hors ligne, rotation, mise en arrière-plan.
- Mesures notées : temps de démarrage à froid, latence de la voix (mot, phrase), fluidité des animations (Framer Motion) sur une tablette d'entrée de gamme, taille installée, consommation mémoire.
- Liste des défauts classés (bloquant / gênant / cosmétique), chacun avec une fiche ou une correction.
- Écoute de contrôle de la voix sur le vocabulaire des dictées CP–CE1 (mots difficiles à prononcer) : liste des mots mal lus, utile pour le lot suivant et pour la banque de mots.

## Critères d'acceptation

- [ ] Rapport de recette rédigé, sans défaut bloquant ouvert.
- [ ] Parcours du critère d'acceptation du lot déroulé entièrement sur la tablette, sans réseau.
