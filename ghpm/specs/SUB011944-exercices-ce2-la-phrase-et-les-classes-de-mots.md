---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXodY
url: https://github.com/georgesp/Educator/issues/301
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011944 — Exercices CE2 — La phrase et les classes de mots
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 1261
---

Source : ancien lot 4 (SUB001610), découpé par classe (décision de Georges du 2026-10-12) ; revu le 2026-10-16 d'après le
programme de 2024 (la mention « à confirmer avec les programmes » est levée).

## User story

En tant qu'enfant de CE2, je veux m'entraîner à reconnaître le sujet et le verbe, les types de phrases et la classe des
mots afin de m'en servir pour écrire sans erreur.

## Ce que dit le programme

- Programme de français du cycle 2 (2024), CE2 : identifier la phrase simple et ses principaux constituants : le groupe
  sujet, le verbe et les compléments, sans distinguer ces derniers entre eux ; substituer un pronom personnel sujet à un
  groupe nominal sujet et inversement.
- Reconnaître et produire les trois types de phrases (déclarative, interrogative, impérative) et les formes négative et
  exclamative ; ponctuation de fin de phrase (. ! ?) et marques du discours rapporté (« … »).
- Différencier et nommer les classes de mots : déterminant, nom commun, nom propre, adjectif, verbe, pronom personnel
  sujet, adverbe ; distinguer mots variables et invariables.

## Compétences à créer (`content/skills/spelling/`, `theme: "sentence"`)

| Compétence (`id`) | Période | Titre | Contenu | Formats |
|---|---|---|---|---|
| `ce2-phrase-groupes` | 1 | Le sujet, le verbe et les compléments | Cliquer sur le verbe conjugué, puis sur le groupe sujet ; le reste de la phrase : les compléments ; remplacer le groupe sujet par il, elle, ils, elles (et l'inverse) | `clickWord`, `chooseSentence` |
| `ce2-classes-nom-det-adj` | 2 | Le nom, le déterminant et l'adjectif | Nom commun / nom propre (majuscule), déterminant, adjectif dans le groupe nominal | `clickWord`, `sortWords` |
| `ce2-classes-verbe-pronom` | 2 | Le verbe et le pronom personnel sujet | Verbe conjugué ou à l'infinitif ; pronoms personnels sujets (je, tu, il, elle, on, nous, vous, ils, elles) | `clickWord`, `sortWords` |
| `ce2-types-formes-phrases` | 3 | Les types et les formes de phrases | Déclarative, interrogative, impérative ; ponctuation . ? ! ; forme négative (ne … pas, ne … plus, ne … jamais) et exclamative ; transformer une phrase à la forme négative ; repérer les paroles entre guillemets | `chooseSentence`, `sentenceChoice`, `clickWord` |
| `ce2-adverbe` | 4 | L'adverbe, un mot invariable | Reconnaître l'adverbe (très, bien, souvent, lentement…) ; trier mots variables et invariables (« Je m'amuse bien / nous nous amusons bien ») | `clickWord`, `sortWords` |

Les périodes sont des choix d'Educator (le programme n'en donne pas), signalés dans le champ `source`.

## Exemples d'exercices

- Clique sur le verbe : « Chaque matin, ma sœur prend le bus. »
- Quel pronom peut remplacer « Les enfants » ? (Ils / Elles / Nous)
- Range les mots : Paris, chanter, petit, une, maison, il.
- Choisis la phrase à la forme négative : « Jean, ne ferme pas la porte. »
- Quelle ponctuation ? « Quel âge as-tu … » → ?

## Contenu à créer

- Phrases dans `content/sentences/phrase-ce2.json` découpées en mots avec leur classe, le groupe sujet et le verbe
  (fiche moteur) : au moins 20 par compétence, relues par Georges ; transformations négatives écrites à la main (pas
  générées).

## Distracteurs (QCM)

Nom pris pour le sujet parce qu'il est en tête de phrase (« Chaque matin ») ; adjectif pris pour un nom ; infinitif pris
pour un nom ; « ne » oublié à la forme négative ; point d'exclamation pour une question.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source` (programme 2024).
- [ ] Au moins 20 phrases par compétence, relues par Georges ; termes grammaticaux conformes à la terminologie d'eduscol.
- [ ] Après une erreur, la bonne réponse est mise en évidence avec une astuce lisible au clic (« C'est … qui », « je change le temps : le verbe change »).
- [ ] Phrases lues correctement en direct (ADR 0004), au clic seulement.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006).
- [ ] Les compétences sont liées à la fiche « Leçons CE2 — La phrase et les classes de mots ».

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)
- [La grammaire du français — terminologie grammaticale (eduscol)](https://eduscol.education.gouv.fr/sites/default/files/document/guide-la-grammaire-du-francais-terminologie-grammaticale-67998.pdf)

## Dépendances

- Fiche « Français CE2 — Exercices de phrase et de conjugaison (moteur et écrans) » : `clickWord`, `sortWords`, `chooseSentence`, thème `sentence`.
