---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAQvVw
rev: 2026-10-12T19:26:35Z
url: https://github.com/georgesp/Educator/issues/234
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012036 — Leçons CM1 — Orthographe et grammaire
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#120"
order: 1630
---

Source : ancien lot 4 (SUB001611), découpé par classe — décision de Georges du 2026-10-12. Contenu précisé le
2026-10-16 d'après le programme de français du cycle 3 (arrêté du 10 avril 2025).

## User story

En tant qu'enfant de CM1, je veux une leçon courte avant chaque nouvelle notion d'orthographe, de conjugaison ou de grammaire.

## Contenu

Une leçon par compétence de la fiche « Exercices CM1 — Orthographe et grammaire » (8 à 12 slides, 3 à 5 minutes),
avec quiz de fin de leçon, astuces et animations interactives dans la charte Grand Prix (nouvelles animations si
besoin : arbre d'accord, frise des temps). Chaque leçon est disponible dès la période de sa compétence.

| Leçon (`content/lessons/spelling/`) | Période | Compétence | L'essentiel (« Je retiens ») |
|---|---|---|---|
| `cm1-mots-frequents.json` | 1 | `cm1-words-frequent` | Les mots les plus fréquents s'apprennent par cœur ; je les relis lettre à lettre. |
| `cm1-homophones-revision.json` | 1 | `cm1-homophones-review` | Pour choisir, je remplace par un autre temps : a → avait, est → était, sont → étaient. |
| `cm1-verbe-et-sujet.json` | 1 | `cm1-gram-verb-subject` | Le verbe change avec le temps ; le sujet répond à « qui est-ce qui ? ». Dans un groupe nominal sujet, je cherche le nom noyau. |
| `cm1-types-et-formes-de-phrases.json` | 1 | `cm1-gram-sentence-types` | Déclarative (.), interrogative (?), impérative (ordre) ; forme négative (ne… pas), exclamative (!). |
| `cm1-present.json` | 1 | `cm1-conj-present` | Radical + terminaison ; les verbes du 3e groupe du programme s'apprennent par cœur. |
| `cm1-familles-de-mots.json` | 2 | `cm1-words-morphology` | Un mot de la même famille m'aide à écrire : grand → grande, donc grand finit par « d ». |
| `cm1-accord-sujet-verbe.json` | 2 | `cm1-agree-subject-verb` | Le verbe s'accorde avec le nom noyau du sujet. |
| `cm1-accords-groupe-nominal.json` | 2 | `cm1-agree-noun-group` | Le nom noyau donne son genre et son nombre au déterminant et à l'adjectif. |
| `cm1-determinants.json` | 2 | `cm1-gram-determiners` | Articles (le, un…), possessifs (mon, ses…), démonstratifs (ce, ces…). |
| `cm1-ces-ses-ce-se.json` | 2 | `cm1-homophones-ces-ses` | ces : je peux ajouter « -là » (ces voitures-là) ; ses : ce sont les siennes ; se et ce : se devant un verbe, ce devant un nom. |
| `cm1-imparfait.json` | 2 | `cm1-conj-imparfait` | Terminaisons : -ais, -ais, -ait, -ions, -iez, -aient. |
| `cm1-accents.json` | 3 | `cm1-words-accents` | é, è, ê se lisent différemment ; l'accent circonflexe remplace souvent un « s » ancien (forêt → forestier). |
| `cm1-pronoms.json` | 3 | `cm1-gram-pronouns` | Le pronom remplace un nom : sujet (il, elle…) ou complément (le, la, lui, leur…). |
| `cm1-conjonctions-adverbes.json` | 3 | `cm1-gram-coord-adverbs` | mais, ou, et, donc, or, ni, car relient ; les adverbes (souvent, toujours…) ne s'accordent jamais. |
| `cm1-ou-ou.json` | 3 | `cm1-homophones-ou` | Si je peux dire « ou bien », j'écris ou ; pour le lieu, j'écris où. |
| `cm1-futur.json` | 3 | `cm1-conj-futur` | Terminaisons : -ai, -as, -a, -ons, -ez, -ont. |
| `cm1-participe-passe-etre.json` | 4 | `cm1-agree-pp-etre` | Avec être, le participe passé s'accorde avec le sujet. |
| `cm1-complements.json` | 4 | `cm1-gram-complements` | Le complément circonstanciel se déplace ou se supprime ; le complément d'objet, non. |
| `cm1-leur-leurs.json` | 4 | `cm1-homophones-leur` | Devant un verbe, leur ne prend jamais de « s » ; devant un nom pluriel, leurs. |
| `cm1-passe-compose.json` | 4 | `cm1-conj-passe-compose` | Passé composé = auxiliaire (être ou avoir) au présent + participe passé. |
| `cm1-cod-coi.json` | 5 | `cm1-gram-cod-coi` | COD : « quoi ? / qui ? » après le verbe ; COI : « à quoi ? / de quoi ? ». |
| `cm1-revision-des-temps.json` | 5 | `cm1-conj-mixed` | Les indices du temps (hier, demain, autrefois) m'aident à choisir. |

## Méthode

Collecte des sources par Claude (eduscol, Réseau Canopé, Lumni, documents académiques), rédaction originale et reformulée, relecture par Georges.

## Critères d'acceptation

- [ ] Une leçon validée par compétence, sources listées, `skillIds` renseignés.
- [ ] Couverture complète des 5 périodes de la classe ; aucune leçon disponible après ses exercices.
- [ ] Aucune notion hors programme du CM1 (liste « Pas au CM1 » de la fiche d'exercices).
- [ ] Exemples lus par Piper vérifiés à l'écoute.

## Sources

- [Programme de français du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025)](https://www.education.gouv.fr/sites/default/files/programme-de-fran-ais-pour-le-cycle-3-439824.pdf)
