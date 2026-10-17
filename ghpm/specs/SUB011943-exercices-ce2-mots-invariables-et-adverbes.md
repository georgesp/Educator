---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoaY
url: https://github.com/georgesp/Educator/issues/300
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011943 — Exercices CE2 — Mots invariables et adverbes
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1254
---

Source : revue du lot 4 du 2026-10-16 (programme de français du cycle 2 de 2024).

## User story

En tant qu'enfant de CE2, je veux m'entraîner à écrire les mots invariables et les adverbes les plus fréquents afin
d'écrire sans erreur.

## Ce que dit le programme

- Programme de français du cycle 2 (2024), CE2, grammaire : l'élève sait orthographier les adverbes les plus fréquents
  (très, si, bien, assez, aujourd'hui, demain…) et les adverbes en -ment.
- Vocabulaire : écrire correctement sous la dictée les mots réguliers et irréguliers fréquemment rencontrés.

## Compétences à créer (`content/skills/spelling/`, `theme: "wordsToKnow"`)

| Compétence (`id`) | Période | Titre | Contenu | Formats |
|---|---|---|---|---|
| `ce2-mots-invariables` | 1 | Les mots invariables du CE2 | Environ 50 mots fréquents : aujourd'hui, demain, hier, toujours, jamais, souvent, longtemps, maintenant, ensuite, bientôt, déjà, encore, beaucoup, assez, très, trop, bien, mal, si, peut-être, pendant, depuis, avant, après, près, loin, dehors, ici, pourquoi, comment, quand, parce que, avec, sans, chez, vers, entre, derrière, devant… (les mots déjà vus au CE1 en révision) | `dictation`, `missingLetters`, `chooseSpelling` |
| `ce2-adverbes-ment` | 4 | Les adverbes en -ment | Adverbe formé sur le féminin de l'adjectif + -ment (lent → lente → lentement ; doux → doucement ; joyeux → joyeusement) ou sur l'adjectif terminé par une voyelle (vrai → vraiment, poli → poliment) ; pas de -amment / -emment (CM1) | `sentenceWrite`, `dictation`, `chooseSpelling` |

Les périodes sont des choix d'Educator (le programme n'en donne pas), signalés dans le champ `source`.

## Exemples d'exercices

- Écris le mot que tu entends : « aujourd'hui ».
- Complète : beauc… (oup).
- La tortue avance (lent) … → lentement.
- Choisis : doucement / doussement / doucemant.

## Contenu à créer

- Mots invariables dans `content/words/` (catégorie `adverbe` ou `mot-outil`, `grades: ["CE2"]`), source de la liste de
  fréquence lexicale ; adjectifs et leurs adverbes en -ment ; phrases pour `sentenceWrite` (au moins 20), relues par Georges.

## Distracteurs (QCM)

Lettre muette oubliée ou ajoutée (toujour, longtemp, jamai), apostrophe ou trait d'union oublié (aujourdhui, peutêtre),
adverbe formé sur le masculin (doux → douxment).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source` (programme 2024).
- [ ] Au moins 30 mots par compétence (au moins 20 phrases pour `sentenceWrite`), relus par Georges.
- [ ] Validation stricte des accents, de l'apostrophe et du trait d'union ; casse tolérée (règles du lot 2).
- [ ] Mots lus correctement en direct (ADR 0004), au clic seulement ; écoute de contrôle `scripts/listen-words.sh --skill <id>`.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006).
- [ ] Les compétences sont liées à la fiche « Leçons CE2 — Mots invariables et adverbes ».

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)
- [Liste de fréquence lexicale (eduscol)](https://eduscol.education.gouv.fr/6873/liste-de-frequence-lexicale)

## Dépendances

- Fiche « Français CE2 — Exercices de phrase et de conjugaison (moteur et écrans) » : `sentenceWrite`.
