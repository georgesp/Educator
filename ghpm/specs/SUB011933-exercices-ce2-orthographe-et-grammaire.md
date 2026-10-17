---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAQvKU
rev: 2026-10-12T19:26:28Z
url: https://github.com/georgesp/Educator/issues/231
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011933 — Exercices CE2 — Les sons et leurs écritures
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 1250
---

Source : ancien lot 4 (SUB001607), découpé par classe (décision de Georges du 2026-10-12) ; réécrite le 2026-10-16
d'après le programme de français du cycle 2 de 2024 (revue du lot 4). Le français du CE2 est découpé en huit thèmes,
chacun avec une fiche d'exercices et une fiche de leçons ; les types d'exercices communs sont dans la fiche « Français
CE2 — Exercices de phrase et de conjugaison (moteur et écrans) ».

## User story

En tant qu'enfant de CE2, je veux m'entraîner à écrire les sons qui ont plusieurs écritures dans les mots de ma classe
afin d'écrire sans erreur.

## Ce que dit le programme

- Programme de français du cycle 2 (2024), CE2, écriture : en dictée, transcrire correctement les phonèmes pouvant
  s'écrire à l'aide de plusieurs graphèmes (/o/, /é/, /è/, /an/, /s/…) dans différents mots fréquents.
- Vocabulaire : écrire correctement sous la dictée les mots réguliers et irréguliers fréquemment rencontrés ; tenir
  compte des accents.

## Compétences à créer (`content/skills/spelling/`, `theme: "lettersAndSounds"`)

| Compétence (`id`) | Période | Titre | Graphies travaillées | Formats |
|---|---|---|---|---|
| `ce2-son-o` | 1 | Le son [o] : o, au, eau, ô | o, au, eau, ô | `dictation`, `missingLetters`, `chooseSpelling` |
| `ce2-son-e-ferme` | 1 | Le son [é] : é, er, ez, ée | é, er (le boulanger), ez (le nez), ée (la journée) | `dictation`, `missingLetters`, `chooseSpelling` |
| `ce2-son-e-ouvert` | 2 | Le son [è] : è, ê, ai, ei, et, e + deux consonnes | è, ê, ai, ei, et, e devant deux consonnes (belle, verre, chouette) | `dictation`, `missingLetters`, `chooseSpelling` |
| `ce2-son-an` | 2 | Le son [an] : an, en, am, em | an, en, am, em (m devant m, b, p) | `dictation`, `missingLetters`, `chooseSpelling` |
| `ce2-son-s` | 3 | Le son [s] : s, ss, c, ç, sc, t | s, ss, c (ce, ci), ç, sc (piscine), t (attention) | `dictation`, `missingLetters`, `chooseSpelling` |

Les périodes sont des choix d'Educator (le programme n'en donne pas), signalés dans le champ `source`.

## Exemples d'exercices

- Écris le mot que tu entends : « un chapeau ».
- Complète : bient…t (ô).
- Choisis la bonne écriture : une fourch…tte / une fourchète → une fourchette.
- Complète : une pi…ine (sc).

## Contenu à créer

- Mots du CE2 (`grades: ["CE2"]`) dans `content/words/`, tirés en priorité de la liste de fréquence lexicale d'eduscol,
  source renseignée pour chaque mot ; graphies et positions renseignées pour les mots à trous ; nouveaux sons
  (`ê`, `ô`, `sc`, `t` de « tion ») ajoutés à `content/spelling/sounds.json` si besoin.
- Les mots du CE1 sur les mêmes sons restent dans leurs compétences ; les mots du CE2 sont plus longs et moins fréquents.

## Distracteurs (QCM)

Graphie voisine du même son (bateau / batau / bato), accent oublié ou changé (fenêtre / fenètre), consonne simple au
lieu de double (chouete).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source` (programme 2024).
- [ ] Au moins 30 mots par compétence, adaptés au CE2, relus par Georges (aucun mot inadapté).
- [ ] Validation stricte des accents, casse tolérée (règles du lot 2).
- [ ] Mots lus correctement en direct (ADR 0004), au clic seulement ; écoute de contrôle `scripts/listen-words.sh --skill <id>`.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006), clavier avec lettres accentuées.
- [ ] Les compétences sont liées à la fiche « Leçons CE2 — Les sons et leurs écritures » (`skillIds` des leçons).

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)
- [Liste de fréquence lexicale (eduscol)](https://eduscol.education.gouv.fr/6873/liste-de-frequence-lexicale)
