---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5R1s
url: https://github.com/georgesp/Educator/issues/10
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000109 — Écran de série d'exercices (clavier et QCM)
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#1"
order: 100
---

## User story

En tant qu'enfant, je veux enchaîner une série de calculs en tapant ma réponse ou en choisissant la bonne afin de m'entraîner à mon rythme.

## Parcours

1. **Exercices → Mathématiques** : liste des compétences de la classe du profil, la période en cours mise en avant (« C'est ce que tu travailles en ce moment »), les autres accessibles.
2. Choix du mode : **« J'écris la réponse »** (clavier) ou **« Je choisis la réponse »** (QCM).
3. Série de 10 exercices, un à la fois :
   - calcul affiché en très grand (ex. `7 + ? = 12`) ;
   - consigne lue par Piper au premier exercice, rejouable avec le bouton haut-parleur ;
   - mode clavier : `NumberPad` à l'écran + clavier physique, bouton **Valider** (ou Entrée) ;
   - mode QCM : 4 tuiles `ChoiceGrid`.
4. Après chaque réponse :
   - juste : animation + son, passage automatique à l'exercice suivant ;
   - faux : la bonne réponse est montrée et lue (« 7 + 5 = 12 »), puis bouton **Suivant** ; pas de perte de points.
5. Fin de série : récapitulatif simple (nombre de bonnes réponses sur 10, points gagnés), boutons **Recommencer** et **Menu**.

- Barre de progression (10 pastilles) en haut de l'écran.
- Bouton **Quitter** toujours visible (confirmation simple, la série en cours est abandonnée).

## Critères d'acceptation

- [ ] Parcours complet réalisable à la souris seule et au clavier seul.
- [ ] Le champ de saisie n'accepte que des chiffres (10 caractères max).
- [ ] Aucun délai perceptible (< 200 ms) entre la validation et le retour.
- [ ] Test de parcours (Playwright) avec une graine fixe : série complète en mode clavier puis en mode QCM.
