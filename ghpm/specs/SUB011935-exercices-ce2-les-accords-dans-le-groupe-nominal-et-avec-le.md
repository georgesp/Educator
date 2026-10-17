---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXoHI
url: https://github.com/georgesp/Educator/issues/292
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011935 — Exercices CE2 — Les accords dans le groupe nominal et avec le sujet
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#119"
order: 1258
---

Source : ancien lot 4 (SUB001609), découpé par classe (décision de Georges du 2026-10-12) ; revu le 2026-10-16 d'après le
programme de 2024.

## User story

En tant qu'enfant de CE2, je veux m'entraîner à accorder les mots du groupe nominal et le verbe avec son sujet afin
d'écrire sans erreur.

## Ce que dit le programme

- Programme de français du cycle 2 (2024), CE2 : repérer, comprendre et mettre en œuvre les marques d'accord au sein du
  groupe nominal ; marques régulières en nombre (-s) et en genre (-e) ; pluriels irréguliers des noms (-x, -al / -aux) ;
  marques du féminin qui s'entendent dans les noms (lecteur / lectrice) et les adjectifs (joyeux / joyeuse).
- Identifier, dans des situations simples, la relation sujet-verbe ; orthographier les chaînes d'accord dans la phrase
  en dictée.

## Compétences à créer (`content/skills/spelling/`, `theme: "agreement"`)

| Compétence (`id`) | Période | Titre | Contenu | Formats |
|---|---|---|---|---|
| `ce2-accord-gn-pluriel` | 2 | Le pluriel dans le groupe nominal | Chaîne d'accord déterminant + nom + adjectif au pluriel (les petits chats noirs) ; le déterminant annonce le nombre | `sentenceWrite`, `sentenceChoice`, `sortWords`, `dictation` |
| `ce2-pluriels-x-aux` | 3 | Les pluriels en -x et en -aux | Noms et adjectifs en -eau, -eu → -x (des bateaux, des cheveux, beaux) ; -al → -aux (un cheval, des chevaux ; normal, normaux) ; les noms en -ou qui prennent un x (bijou, caillou, chou, genou, hibou, joujou, pou) | `sentenceWrite`, `chooseSpelling`, `dictation` |
| `ce2-accord-sujet-verbe` | 3 | Accorder le verbe avec le sujet | Trouver le sujet (« C'est … qui ») et accorder le verbe au présent (il joue / ils jouent, nous jouons) ; sujet nom ou pronom ; sujet séparé du verbe par un petit mot (Les enfants ne jouent pas) | `clickWord`, `sentenceChoice`, `sentenceWrite`, `sentenceDictation` |
| `ce2-feminin` | 4 | Le féminin des noms et des adjectifs | + e (un ami, une amie ; grand, grande) ; féminins qui s'entendent : -eur / -euse (chanteur), -teur / -trice (lecteur), -eux / -euse (joyeux), -if / -ive (sportif), -er / -ère (boulanger), consonne doublée (bon / bonne, gros / grosse, gentil / gentille) | `sentenceWrite`, `sortWords`, `chooseSpelling`, `dictation` |

Les périodes sont des choix d'Educator (le programme n'en donne pas), signalés dans le champ `source`. Les noms en -ou
en -oux sont un choix d'Educator (le programme ne cite que -x et -al / -aux).

## Exemples d'exercices

- Les petit… chat… noir… dorment. → s, s, s.
- un cheval → des … → chevaux.
- Clique sur le sujet du verbe « courent » : « Dans la cour, les enfants courent. »
- Les oiseaux (chanter) … → chantent.
- Écris au féminin : un lecteur joyeux → une lectrice joyeuse.

## Contenu à créer

- Phrases dans `content/sentences/accords-ce2.json` : au moins 20 par compétence, avec le genre et le nombre des mots,
  le sujet et le verbe marqués, relues par Georges.
- Champs `feminine` et `plural` dans la banque de mots pour les noms et adjectifs utilisés (fiche moteur) ; groupes
  nominaux du CE2 (catégorie `groupe-nominal`) pour la dictée.

## Distracteurs (QCM)

Accord oublié sur l'adjectif ; -s au lieu de -x (des bateaus) ; -als au lieu de -aux ; -s au verbe (ils joues) ; verbe
accordé avec le mot le plus proche au lieu du sujet ; féminin en -eure (une chanteure).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source` (programme 2024).
- [ ] Au moins 20 phrases par compétence et au moins 30 mots ou groupes nominaux pour la dictée, relus par Georges.
- [ ] Le mot qui commande l'accord (nom, sujet) est mis en évidence après une erreur ; astuce lisible au clic.
- [ ] Validation stricte des accents, casse tolérée (règles du lot 2) ; dictée de phrase corrigée mot par mot.
- [ ] Mots et phrases lus correctement en direct (ADR 0004), au clic seulement.
- [ ] Exercices jouables au doigt sur iPad (ADR 0006).
- [ ] Les compétences sont liées à la fiche « Leçons CE2 — Les accords dans le groupe nominal et avec le sujet ».

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)
- [La grammaire du français du CP à la 6e (eduscol)](https://eduscol.education.gouv.fr/sites/default/files/document/guide-la-grammaire-du-francais-du-cp-la-6e-97251.pdf)

## Dépendances

- Fiche « Français CE2 — Exercices de phrase et de conjugaison (moteur et écrans) » : formats de phrase, `content/sentences/`, champs `feminine` et `plural`.
- Fiche « Exercices CE2 — Le verbe et le présent » pour les formes du présent utilisées dans l'accord sujet-verbe.
