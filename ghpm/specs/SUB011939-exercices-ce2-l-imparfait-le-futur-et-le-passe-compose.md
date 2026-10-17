---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoRY
url: https://github.com/georgesp/Educator/issues/296
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011939 — Exercices CE2 — L'imparfait, le futur et le passé composé"
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 1265
---

Source : revue du lot 4 du 2026-10-16 — la conjugaison, au programme du CE2, manquait au lot.

## User story

En tant qu'enfant de CE2, je veux m'entraîner à conjuguer au futur, à l'imparfait et au passé composé, et à reconnaître
le temps d'un verbe, afin d'écrire les verbes sans erreur.

## Ce que dit le programme

- Programme de français du cycle 2 (2024), CE2 : conjuguer au présent, à l'imparfait, au futur et au passé composé de
  l'indicatif être et avoir, les verbes du premier groupe et faire, aller, dire, venir, pouvoir, voir, vouloir, prendre.
- Opérer des transformations de personne et de temps et observer les modifications (je joue au ballon / nous avons joué
  au ballon) ; orthographier les formes verbales étudiées en dictée.

## Compétences à créer (`content/skills/spelling/`, `theme: "verbs"`)

| Compétence (`id`) | Période | Titre | Contenu | Formats |
|---|---|---|---|---|
| `ce2-futur` | 3 | Le futur | Terminaisons -rai, -ras, -ra, -rons, -rez, -ront ; être (je serai), avoir (j'aurai), verbes en -er (je chanterai) ; je ferai, j'irai, je dirai, je viendrai, je pourrai, je verrai, je voudrai, je prendrai | `conjugate`, `sentenceChoice`, `sentenceWrite` |
| `ce2-imparfait` | 4 | L'imparfait | Terminaisons -ais, -ais, -ait, -ions, -iez, -aient ; être (j'étais), avoir (j'avais), verbes en -er (je mangeais, je commençais, nous mangions) ; je faisais, j'allais, je disais, je venais, je pouvais, je voyais, je voulais, je prenais | `conjugate`, `sentenceChoice`, `sentenceWrite` |
| `ce2-passe-compose` | 5 | Le passé composé | Auxiliaire avoir ou être au présent + participe passé : j'ai chanté, j'ai été, j'ai eu, j'ai fait, j'ai dit, j'ai pu, j'ai vu, j'ai voulu, j'ai pris ; avec être : je suis allé(e), elle est venue, ils sont allés (le participe s'accorde avec le sujet) | `conjugate`, `sentenceChoice`, `sentenceWrite`, `sentenceDictation` |
| `ce2-temps-revision` | 5 | Passé, présent ou futur ? | Reconnaître le temps d'un verbe (indices hier, aujourd'hui, demain) ; transformer une phrase d'un temps à l'autre | `sortWords`, `chooseSentence`, `conjugate` |

Les périodes sont des choix d'Educator (le programme n'en donne pas), signalés dans le champ `source`.

## Exemples d'exercices

- Conjugue *jouer* au futur avec *nous* → jouerons.
- Hier, nous (manger) … à la cantine (imparfait) → mangions.
- Elles (venir, passé composé) … me voir → sont venues.
- Range ces phrases : Hier, j'ai joué. / Demain, j'irai à la piscine. / Aujourd'hui, je dessine.
- Seuls les verbes du programme sont conjugués (être, avoir, 1er groupe, les huit verbes irréguliers).

## Contenu à créer

- Formes de l'imparfait, du futur et du passé composé calculées pour les verbes en -er, écrites pour être, avoir et les
  huit verbes irréguliers (`content/verbs/verbs.json`, fiche moteur) ; phrases (au moins 20 par compétence) relues par Georges.

## Distracteurs (QCM)

Terminaison d'un autre temps (je chantais au lieu de je chanterai) ; radical du futur oublié (je fairai, j'allerai) ;
« e » oublié (je mangais) ; participe passé en -er (j'ai chanter) ;
participe non accordé avec être (elles sont venu).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source` (programme 2024).
- [ ] Formes générées par `educator-core::conjugation` testées (fiche moteur) ; au moins 20 phrases par compétence, relues par Georges.
- [ ] Validation stricte des accents, casse tolérée (règles du lot 2) ; « allé » et « allée » acceptés seulement quand le sujet ne dit pas le genre (je, tu, nous, vous).
- [ ] Consignes et phrases lues correctement en direct (ADR 0004), au clic seulement, sans dire la réponse.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006).
- [ ] Les compétences sont liées à la fiche « Leçons CE2 — L'imparfait, le futur et le passé composé ».

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)

## Dépendances

- Fiche « Français CE2 — Exercices de phrase et de conjugaison (moteur et écrans) » : `conjugate`, `content/verbs/`.
- Fiche « Exercices CE2 — Le verbe et le présent » (auxiliaires au présent pour le passé composé).
