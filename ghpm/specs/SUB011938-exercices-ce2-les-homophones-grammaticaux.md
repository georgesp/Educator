---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoO8
url: https://github.com/georgesp/Educator/issues/295
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011938 — Exercices CE2 — Les homophones grammaticaux
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1256
---

Source : ancien lot 4 (SUB001608), découpé par classe (décision de Georges du 2026-10-12) ; revu le 2026-10-16.

## User story

En tant qu'enfant de CE2, je veux m'entraîner à choisir entre a et à, et et est, son et sont, on et ont afin d'écrire
sans erreur.

## Ce que dit le programme

- Le programme de français du cycle 2 de 2024 ne cite pas d'homophones : il demande d'orthographier les chaînes d'accord
  et les formes verbales étudiées (a, est, sont, ont sont des formes d'avoir et d'être au présent).
- **Choix d'Educator** : travailler ces quatre paires au CE2, comme la plupart des manuels ; signalé dans le champ
  `source`. Les paires ou / où, ces / ses, ce / se, leur / leurs sont au CM1 (lot 5).

## Compétences à créer (`content/skills/spelling/`, `theme: "homophones"`)

| Compétence (`id`) | Période | Titre | Contenu | Formats |
|---|---|---|---|---|
| `ce2-homophones-a-et` | 1 | a / à, et / est | a (verbe avoir, remplaçable par « avait ») / à ; est (verbe être, remplaçable par « était ») / et (remplaçable par « et puis ») | `sentenceChoice`, `sentenceDictation` |
| `ce2-homophones-son-on` | 2 | son / sont, on / ont | son (remplaçable par « mon ») / sont (« étaient ») ; on (« il ») / ont (« avaient ») | `sentenceChoice`, `sentenceDictation` |
| `ce2-homophones-revision` | 5 | Les homophones : révision | Les quatre paires mélangées, dans des phrases plus longues | `sentenceChoice`, `sentenceDictation` |

Les périodes sont des choix d'Educator, signalés dans le champ `source`.

## Exemples d'exercices

- Léo … un vélo rouge. (a / à)
- Le chat … sur le mur … il dort. (est, et)
- Les enfants … contents : ils … gagné. (sont / son ; ont / on)
- Écris la phrase que tu entends : « On a mis son pull. »

## Contenu à créer

- Phrases dans `content/sentences/homophones-ce2.json` : au moins 20 par paire et par compétence, avec l'astuce de
  remplacement de chaque forme, relues par Georges ; la phrase est lue au clic avec « blanc » à la place du mot (ADR 0004).
- Les groupes de `content/spelling/homophones.json` restent la référence des paires.

## Distracteurs (QCM)

L'autre forme de la paire ; jamais une forme qui n'existe pas.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source` (choix d'Educator signalé).
- [ ] Au moins 20 phrases par paire d'homophones et par compétence, relues par Georges.
- [ ] Astuce de remplacement affichée après une erreur, lisible au clic.
- [ ] Validation stricte des accents (à), casse tolérée (règles du lot 2).
- [ ] Phrases lues correctement en direct (ADR 0004), au clic seulement.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006).
- [ ] Les compétences sont liées à la fiche « Leçons CE2 — Les homophones grammaticaux ».

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)

## Dépendances

- Fiche « Français CE2 — Exercices de phrase et de conjugaison (moteur et écrans) » : `sentenceChoice`, `sentenceDictation`, thème `homophones`.
