---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXozw
url: https://github.com/georgesp/Educator/issues/309
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011952 — Leçons CE2 — Les accords dans le groupe nominal et avec le sujet
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1259
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte sur chaque accord, avec la chaîne d'accord dessinée, avant de
m'entraîner.

## Leçons

Une leçon par compétence de la fiche « Exercices CE2 — Les accords dans le groupe nominal et avec le sujet » (8 à 12
slides, 3 à 5 minutes).

| Leçon (`content/lessons/spelling/`) | `skillIds` | Période | Contenu clé (« Je retiens ») | Interactions | Quiz (exemples) |
|---|---|---|---|---|---|
| `ce2-accord-gn-pluriel.json` | `ce2-accord-gn-pluriel` | 2 | Dans le groupe nominal, le nom commande : le déterminant et l'adjectif s'accordent avec lui. Au pluriel, j'ajoute -s à tous (les petits chats noirs). | `agreementChain` ; `wordClassSort` (singulier / pluriel) | complète « les grand… arbres » ; QCM des beaux / des beau jouets |
| `ce2-pluriels-x-aux.json` | `ce2-pluriels-x-aux` | 3 | Après -eau et -eu, le pluriel est -x (des bateaux, des cheveux) ; -al devient -aux (des chevaux) ; sept noms en -ou prennent un x (bijoux, cailloux, choux, genoux, hiboux, joujoux, poux). | `agreementChain` ; `letterTiles` | écris « des journaux » ; QCM des hibous / des hiboux |
| `ce2-accord-sujet-verbe.json` | `ce2-accord-sujet-verbe` | 3 | Le verbe s'accorde avec son sujet. Pour trouver le sujet, je dis « C'est … qui » devant le verbe. Il joue → ils jouent. | `sentenceGroups` (le sujet s'entoure) ; `agreementChain` (flèche du sujet vers le verbe) | clique sur le sujet ; complète « Les enfants jou… » |
| `ce2-feminin.json` | `ce2-feminin` | 4 | Au féminin, j'ajoute souvent -e (une amie, grande). Parfois la fin du mot change et ça s'entend : chanteur → chanteuse, lecteur → lectrice, joyeux → joyeuse, sportif → sportive, bon → bonne. | `wordClassSort` (masculin / féminin) ; `letterTiles` | écris le féminin de « lecteur » ; QCM une chanteure / une chanteuse |

Les textes ci-dessus donnent l'intention ; la rédaction définitive se fait dans les fichiers JSON. Nouvelle animation
« arbre d'accord » prévue par l'ancienne fiche : c'est `agreementChain`.

## Méthode

Collecte des sources par Claude (eduscol, Réseau Canopé, Lumni, documents académiques), rédaction originale et
reformulée, relecture par Georges.

## Critères d'acceptation

- [ ] Une leçon par compétence, validée par `lesson.schema.json`, `skillIds` renseignés, sources listées.
- [ ] 8 à 12 slides, 3 à 5 minutes mesurées avec la voix ; au moins deux slides où l'enfant manipule, à la souris et au doigt (iPad, ADR 0006).
- [ ] Quiz de 3 à 5 questions avec explications, sans tolérance sur les accents ; fin de leçon : bouton « Je m'entraîne ».
- [ ] Rien n'est lu tout seul (décision du 2026-10-15).
- [ ] Relues et validées par Georges.

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)

## Dépendances

- Format des leçons (SUB001409), lecteur (SUB001410), quiz (SUB001411).
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `agreementChain`, `sentenceGroups`, `wordClassSort` ; réponse « phrase » courte au quiz.
