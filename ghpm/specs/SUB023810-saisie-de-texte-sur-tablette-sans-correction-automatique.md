---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAsD9U
rev: 2026-10-14T11:29:11Z
url: https://github.com/georgesp/Educator/issues/249
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB023810 — Saisie de texte sur tablette sans correction automatique
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#238"
order: 3100
---

## User story

En tant que parent, je veux que la tablette ne corrige pas l'orthographe à la place de mon enfant pendant une dictée ou un exercice de mots.

## Contexte

Sur tablette, le clavier à l'écran peut corriger, compléter ou mettre en majuscule automatiquement : une dictée ne vaut plus rien si le système a corrigé le mot. Orthographe : aucune tolérance sur les accents (CLAUDE.md), il faut donc aussi pouvoir saisir é, è, ê, à, ç, etc.

## Comportement attendu

- Sur tous les champs de réponse orthographique : `autocorrect="off"`, `autocapitalize="off"`, `spellcheck="false"`, `autocomplete="off"`, suggestions désactivées (`inputmode`, `enterkeyhint` adaptés).
- Accents : vérifier le clavier français de la tablette (appui long) ; évaluer un petit clavier de lettres accentuées dans l'application (rangée « é è ê ë à â ç î ï ô ù û ») à côté du champ, dans l'esprit du pavé numérique maison. Décision notée dans la fiche après essai sur la tablette.
- La saisie reste une chaîne normalisée par le Rust, comme aujourd'hui (SUB001405) : aucune logique de validation dans le front.
- Le clavier à l'écran s'ouvre et se ferme proprement (focus après l'audio, pas de saut de page).

## Critères d'acceptation

- [ ] Sur la tablette de test : saisie de « élève », « garçon », « où » sans correction ni majuscule automatique, avec le clavier du système ou le clavier accentué.
- [ ] Test Vitest : les attributs sont présents sur chaque champ d'orthographe.
- [ ] Les exercices de mathématiques (pavé numérique) ne sont pas affectés.

## Implémentation (2026-10-15)

- **Décision** : sur écran tactile, les champs de réponse (calcul `SeriesScreen`, quiz `LessonQuiz`, orthographe `SpellingAnswer`) ont `inputMode="none"` (`web/src/app/device.ts`) : le clavier du système ne s'ouvre pas ; l'enfant écrit avec `NumberPad` et `LetterKeyboard` (AZERTY, rangée d'accents « é è ê à â ù û ç ë ï î ô œ »). Aucune correction ni majuscule automatique possible, aucun clavier qui masque le champ.
- Champ d'orthographe : `autocorrect`, `autocapitalize`, `spellcheck`, `autocomplete` désactivés, `enterkeyhint="done"` (test `SpellingAnswer.test.tsx`) ; `inputMode` testé (`device.test.ts`).
- À l'ordinateur : clavier physique inchangé (`numeric` / `text`).
- Saisie toujours normalisée et validée par le Rust (aucune logique dans le front).

## Tests à faire (revue)

1. iPad : toucher le champ de réponse d'une série de calcul, d'un quiz de leçon et d'une dictée → le clavier de l'iPad ne s'ouvre jamais ; seuls le pavé numérique ou le clavier de lettres de l'application servent.
2. Dictée : écrire « élève », « garçon », « où » avec le clavier de l'application (rangée des accents) → mots saisis tels quels, sans majuscule ni correction ; validation acceptée.
3. Mathématiques : pavé numérique inchangé (chiffres, effacer, valider).
4. Bureau : le clavier physique marche toujours dans les champs (calcul, dictée).
5. `npm test` → `device.test.ts` (2) et `SpellingAnswer.test.tsx` (2) verts.
