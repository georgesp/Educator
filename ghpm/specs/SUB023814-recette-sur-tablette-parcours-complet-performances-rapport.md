---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsEOo
rev: 2026-10-14T11:29:22Z
url: https://github.com/georgesp/Educator/issues/253
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB023814 — Recette sur tablette (parcours complet, performances, rapport)"
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#238"
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

## Implémentation (2026-10-15)

- Checklist de recette (19 lignes : écrans, exercices, leçons, examen, défi chrono, voix, hors ligne, rotation, fermeture forcée, mesures, écoute des mots CP–CE1) dans `docs/tablette.md`, étape 11, avec classement bloquant / gênant / cosmétique.
- Défaut connu à confirmer à la recette : la sauvegarde des profils (espace parent) passe par le dialogue de fichier du bureau, absent sur l'iPad.

## Corrections après relecture (2026-10-15)

- Sauvegarde des profils : pas de boîte de dialogue de fichiers sur l'iPad pour l'instant ; l'espace parents affiche « La sauvegarde dans un fichier n'est pas encore possible sur l'iPad… » à la place des boutons (`isIpad()` dans `app/device.ts`, tests Vitest).
- À vérifier sur l'iPad : espace parents › Sauvegarde des profils : le message, sans bouton ; sur le Mac : les deux boutons, inchangés.

## Tests à faire (revue)

1. Installer une version autonome (`docs/tablette.md`, étape 10), passer l'iPad en mode avion.
2. Dérouler les 19 lignes de la checklist (`docs/tablette.md`, étape 11) ; chronométrer le démarrage à froid ; relever taille installée et mémoire.
3. Copier le tableau rempli dans une section « Recette » de cette fiche ; pour chaque défaut bloquant ou gênant, une fiche ou une correction.
4. Écoute des mots CP–CE1 : noter les mots mal prononcés par Thomas.
