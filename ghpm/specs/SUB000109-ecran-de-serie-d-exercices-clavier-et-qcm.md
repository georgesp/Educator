---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5R1s
rev: 2026-10-10T19:29:05Z
url: https://github.com/georgesp/Educator/issues/10
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000109 — Écran de série d'exercices (clavier et QCM)
status: In progress
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

- [x] Parcours complet réalisable à la souris seule et au clavier seul.
- [x] Le champ de saisie n'accepte que des chiffres (10 caractères max).
- [ ] Aucun délai perceptible (< 200 ms) entre la validation et le retour.
- [x] Test de parcours (Playwright) avec une graine fixe : série complète en mode clavier puis en mode QCM.

## Implémentation (2026-10-11)

- Écrans `SkillsScreen` (période en cours mise en avant), `ModeScreen` (« J'écris la réponse » / « Je choisis la réponse ») et `SeriesScreen`.
- Dans la série :
  - le calcul s'affiche en très grand, avec le champ de réponse ou le « ? » à la place du nombre demandé ;
  - mode clavier : `NumberPad` et clavier physique, validation par Entrée ;
  - mode QCM : 4 tuiles ;
  - la progression est une piste à 10 pastilles avec une voiture qui avance jusqu'au drapeau à damier ;
  - « Quitter » demande une confirmation simple, puis abandonne la série (les bonnes réponses sont gardées) ;
  - récapitulatif final avec « Recommencer » et « Menu ».
- Juste : animation et son, puis exercice suivant après 0,9 s. Faux : la réponse donnée est barrée, la bonne réponse est affichée et lue, puis le bouton « Suivant » apparaît.
- Délai mesuré sur le host (Linux) entre l'envoi et le retour d'une réponse : moins de 5 ms. Le critère < 200 ms est à confirmer sur Mac dans l'application complète.
- Les réponses de plus de 9 chiffres sont plafonnées côté front : ce sont de simples mauvaises réponses.
- Tests : `SeriesScreen.test.tsx` ; `e2e/lot1.spec.ts` fait une série complète au clavier (Entrée), puis une série en QCM à la souris. Elle utilise une série fixe simulée, l'équivalent d'une graine fixe.
