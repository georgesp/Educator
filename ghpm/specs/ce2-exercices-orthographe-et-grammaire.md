---
type: Issue
title: Exercices CE2 — Orthographe et grammaire
status: Todo
size: L
parent: "specs/US0119-lot-1-1-ce2-exercices-et-lecons-de-mathematiques.md"
order: 1630
---

Source : ancien lot 4 (SUB001607 à SUB001610), découpé par classe — décision de Georges du 2026-10-12.

## User story

En tant qu'enfant de CE2, je veux m'entraîner à l'orthographe et à la grammaire de ma classe afin d'écrire sans erreur.

## Contenu

- **Banque de mots CE2** : doubles consonnes, lettres muettes, accents (é / è / ê), mots invariables, familles de mots ; mots tirés de la liste de fréquence lexicale d'eduscol et des repères annuels.
- **Homophones** : a / à, et / est, son / sont, on / ont.
- **Accords** : groupe nominal (déterminant, nom, adjectif) ; sujet / verbe au présent.
- **Grammaire** : nature des mots : nom, verbe, adjectif, déterminant (à confirmer avec les programmes en vigueur).

## Comportement attendu

- Dictée de mots, mots à trous et « choisis la bonne écriture » (lot 2) avec les compétences CE2 par période.
- **Création** des types « phrase à trou » avec choix de l'homophone (astuce de vérification affichée et lue après une erreur), « phrase à compléter » (terminaison, mot qui commande l'accord mis en évidence) et « clique sur le verbe / trie les mots ». Banques de phrases dans `content/sentences/*.json`.

## Prérequis

Le lot 1 (formats de dictée de mots, mots à trous et bonne écriture du lot 2).

## Critères d'acceptation

- [ ] Compétences par période vérifiées contre les repères annuels CE2, `source` renseignée.
- [ ] Au moins 30 mots par compétence, audio Piper généré.
- [ ] Au moins 20 phrases par paire d'homophones et par notion d'accord, relues.
- [ ] Validation stricte des accents, casse tolérée (règles du lot 2).
