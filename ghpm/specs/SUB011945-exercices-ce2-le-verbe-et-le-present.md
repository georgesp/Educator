---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXof0
url: https://github.com/georgesp/Educator/issues/302
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011945 — Exercices CE2 — Le verbe et le présent
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 1263
---

Source : revue du lot 4 du 2026-10-16 — la conjugaison, au programme du CE2, manquait au lot.

## User story

En tant qu'enfant de CE2, je veux m'entraîner à trouver l'infinitif d'un verbe et à conjuguer au présent afin d'écrire
les verbes sans erreur.

## Ce que dit le programme

- Programme de français du cycle 2 (2024), CE2 : apprendre à conjuguer au présent, à l'imparfait, au futur et au passé
  composé de l'indicatif être et avoir, les verbes du premier groupe et les verbes irréguliers du 3e groupe faire, aller,
  dire, venir, pouvoir, voir, vouloir, prendre.
- Identifier le radical et la terminaison d'un verbe conjugué au programme et trouver son infinitif ; opérer des
  transformations de personne et de temps ; orthographier les formes verbales étudiées en dictée.

## Compétences à créer (`content/skills/spelling/`, `theme: "verbs"`)

| Compétence (`id`) | Période | Titre | Contenu | Formats |
|---|---|---|---|---|
| `ce2-verbe-infinitif` | 1 | Infinitif, radical et terminaison | Trouver l'infinitif d'un verbe conjugué (nous jouons → jouer ; il fait → faire) ; séparer radical et terminaison des verbes en -er ; reconnaître les verbes du 1er groupe | `sentenceWrite`, `clickWord`, `sortWords` |
| `ce2-present-etre-avoir-er` | 1 | Le présent d'être, d'avoir et des verbes en -er | Présent d'être, d'avoir et des verbes en -er de la liste (avec nous mangeons, nous commençons) ; toutes les personnes | `conjugate`, `sentenceChoice`, `sentenceWrite`, `sentenceDictation` |
| `ce2-present-irreguliers` | 2 | Le présent des verbes faire, aller, dire, venir, pouvoir, voir, vouloir, prendre | Présent des huit verbes du programme (vous faites, vous dites, ils vont, ils viennent, je peux, ils veulent, ils prennent…) | `conjugate`, `sentenceChoice`, `sentenceWrite` |

Les périodes sont des choix d'Educator (le programme n'en donne pas), signalés dans le champ `source`.

## Exemples d'exercices

- Quel est l'infinitif de « ils regardent » ? → regarder.
- Clique sur la terminaison : « nous jou|ons ».
- Conjugue *chanter* au présent avec *nous* → chantons.
- Tu (avoir) … un chat → as.
- Vous (faire) … du vélo → faites.
- Ils (prendre) … le train → prennent.

## Contenu à créer

- Verbes dans `content/verbs/verbs.json` (fiche moteur) : être, avoir, au moins 40 verbes en -er fréquents, les huit
  verbes irréguliers ; phrases dans `content/sentences/verbes-ce2.json` (au moins 20 par compétence), relues par Georges.
- Seuls les verbes du programme sont proposés au CE2 (pas de verbes du 2e groupe ni d'autres verbes du 3e groupe).

## Distracteurs (QCM)

Terminaison d'une autre personne (nous chantez) ; -s à la 3e personne (il chantes) ; forme régulière d'un verbe irrégulier
(vous faisez, ils allent) ; « e » oublié (nous mangons).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source` (programme 2024).
- [ ] Formes générées par `educator-core::conjugation` testées (fiche moteur) ; au moins 20 phrases par compétence, relues par Georges.
- [ ] Validation stricte des accents, casse tolérée (règles du lot 2).
- [ ] Consigne (« Conjugue *chanter* au présent avec *nous* ») et phrases lues correctement en direct (ADR 0004), au clic seulement, sans dire la réponse.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006).
- [ ] Les compétences sont liées à la fiche « Leçons CE2 — Le verbe et le présent ».

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)

## Dépendances

- Fiche « Français CE2 — Exercices de phrase et de conjugaison (moteur et écrans) » : `conjugate`, `content/verbs/`, thème `verbs`.
