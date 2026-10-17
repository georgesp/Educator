---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXo8o
url: https://github.com/georgesp/Educator/issues/313
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011956 — Leçons CE2 — Mots invariables et adverbes
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1255
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte sur les mots invariables et les adverbes en -ment avant de m'entraîner.

## Leçons

Une leçon par compétence de la fiche « Exercices CE2 — Mots invariables et adverbes » (8 à 12 slides, 3 à 5 minutes).

| Leçon (`content/lessons/spelling/`) | `skillIds` | Période | Contenu clé (« Je retiens ») | Interactions | Quiz (exemples) |
|---|---|---|---|---|---|
| `ce2-mots-invariables.json` | `ce2-mots-invariables` | 1 | Un mot invariable s'écrit toujours de la même façon, au singulier comme au pluriel : je l'apprends par cœur, avec ses lettres muettes (toujours, longtemps) et ses signes (aujourd'hui, peut-être). | `letterTiles` (reconstituer le mot) ; `soundSort` (ranger par moment : hier, aujourd'hui, demain) | écris « beaucoup » ; QCM aujourdhui / aujourd'hui |
| `ce2-adverbes-ment.json` | `ce2-adverbes-ment` | 4 | Pour former un adverbe en -ment, je pars du féminin de l'adjectif : lente → lentement, douce → doucement ; si l'adjectif finit par une voyelle, j'ajoute -ment : vrai → vraiment. | `wordFamily` (adjectif → féminin → adverbe) ; `letterTiles` | écris l'adverbe de « lent » ; QCM joyeusement / joyeuxment |

Les textes ci-dessus donnent l'intention ; la rédaction définitive se fait dans les fichiers JSON.

## Méthode

Collecte des sources par Claude (eduscol, Réseau Canopé, Lumni, documents académiques), rédaction originale et
reformulée, relecture par Georges.

## Critères d'acceptation

- [ ] Une leçon par compétence, validée par `lesson.schema.json`, `skillIds` renseignés, sources listées.
- [ ] 8 à 12 slides, 3 à 5 minutes mesurées avec la voix ; au moins une animation `letterTiles` ou `soundSort` ; au moins deux slides où l'enfant manipule, à la souris et au doigt (iPad, ADR 0006).
- [ ] Quiz de 3 à 5 questions avec explications, sans tolérance sur les accents ; fin de leçon : bouton « Je m'entraîne ».
- [ ] Rien n'est lu tout seul (décision du 2026-10-15).
- [ ] Relues et validées par Georges.

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)

## Dépendances

- Format des leçons (SUB001409), lecteur (SUB001410), quiz (SUB001411).
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `wordFamily`.
