---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoJs
url: https://github.com/georgesp/Educator/issues/293
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011936 — Exercices CE2 — Familles de mots, préfixes et suffixes"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1252
---

Source : revue du lot 4 du 2026-10-16 (programme de français du cycle 2 de 2024).

## User story

En tant qu'enfant de CE2, je veux m'appuyer sur les mots de la même famille pour écrire les lettres muettes et
comprendre les mots construits, afin d'écrire sans erreur.

## Ce que dit le programme

- Programme de français du cycle 2 (2024), CE2, vocabulaire : s'appuyer sur des critères morphologiques (radical,
  préfixe et suffixe) et analogiques pour mémoriser l'orthographe des mots (« il s'appuie sur *beau* pour orthographier
  *beauté* »).
- Trier et apparier des mots et leurs dérivés selon les préfixes et suffixes ; dérivation avec changement de classe
  (port / portuaire / aéroport ; observation, observer, observable) ; sens négatif des préfixes dé-, mal-, in-
  (déforestation, malhabile, immobile).

## Compétences à créer (`content/skills/spelling/`)

| Compétence (`id`) | Période | Thème | Titre | Contenu | Formats |
|---|---|---|---|---|---|
| `ce2-lettres-muettes-famille` | 2 | `silentLetters` | Trouver la lettre muette | Lettre muette finale retrouvée par le féminin ou un mot de la même famille (grand → grande, chant → chanter, dent → dentiste, bavard → bavarder, gris → grise) | `missingLetters`, `chooseSpelling`, `dictation` |
| `ce2-familles-mots` | 3 | `spellingRules` | Les familles de mots | Mots d'une même famille (terre, terrain, enterrer ; beau, beauté, embellir) ; trouver l'intrus ; écrire un mot grâce à sa famille | `sortWords`, `chooseSpelling`, `sentenceChoice` |
| `ce2-prefixes-suffixes` | 4 | `spellingRules` | Préfixes et suffixes | Préfixes dé-, in- / im- (im- devant m, b, p), mal-, re- ; suffixes -eur / -euse, -ette, -age, -able, -tion ; changer de classe (laver → lavage → lavable) | `sortWords`, `chooseSpelling`, `sentenceChoice` |

Les périodes sont des choix d'Educator (le programme n'en donne pas), signalés dans le champ `source`.

## Exemples d'exercices

- Complète : un chan… (pense à « chanter ») → t.
- Range ces mots par famille : dent, terre, dentiste, terrain, dentifrice, enterrer.
- Choisis le contraire de « possible » : inpossible / impossible / dépossible → impossible.
- Choisis le contraire de « faire » : défaire / malfaire / infaire → défaire.

## Contenu à créer

- Champ `family` dans la banque de mots (fiche « Français CE2 — Exercices de phrase et de conjugaison (moteur et écrans) ») : familles de
  la liste de fréquence lexicale et du vocabulaire courant ; mots à lettre muette avec leur mot « témoin ».
- Phrases pour `sentenceChoice` (au moins 20 par compétence), relues par Georges.

## Distracteurs (QCM)

Lettre muette d'une autre famille (un chanp / un chand), préfixe mal choisi (inpossible), suffixe d'une autre classe
(lavable pour lavage).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source` (programme 2024).
- [ ] Au moins 30 mots ou 20 phrases par compétence, adaptés au CE2, relus par Georges.
- [ ] Validation stricte des accents, casse tolérée (règles du lot 2).
- [ ] Mots et phrases lus correctement en direct (ADR 0004), au clic seulement.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006).
- [ ] Les compétences sont liées à la fiche « Leçons CE2 — Familles de mots, préfixes et suffixes ».

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)
- [Liste de fréquence lexicale (eduscol)](https://eduscol.education.gouv.fr/6873/liste-de-frequence-lexicale)

## Dépendances

- Fiche « Français CE2 — Exercices de phrase et de conjugaison (moteur et écrans) » : `sortWords`, `sentenceChoice`, champ `family`.
