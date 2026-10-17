---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXpAA
url: https://github.com/georgesp/Educator/issues/315
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011958 — Leçons CE2 — Le verbe et le présent
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1264
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte sur l'infinitif et sur le présent, avec le tableau de conjugaison
animé, avant de m'entraîner.

## Leçons

Une leçon par compétence de la fiche « Exercices CE2 — Le verbe et le présent » (8 à 12 slides, 3 à 5 minutes).

| Leçon (`content/lessons/spelling/`) | `skillIds` | Période | Contenu clé (« Je retiens ») | Interactions | Quiz (exemples) |
|---|---|---|---|---|---|
| `ce2-verbe-infinitif.json` | `ce2-verbe-infinitif` | 1 | Un verbe conjugué a un radical (le début) et une terminaison (la fin, qui change). Pour le nommer, je dis son infinitif : « il faut … » (il faut jouer). Les verbes en -er sont du 1er groupe. | `conjugationTable` (radical et terminaison en couleurs) ; `wordClassSort` (1er groupe / autres) | écris l'infinitif de « ils regardent » ; clique sur la terminaison |
| `ce2-present-etre-avoir-er.json` | `ce2-present-etre-avoir-er` | 1 | Être : je suis, tu es, il est, nous sommes, vous êtes, ils sont. Avoir : j'ai, tu as, il a, nous avons, vous avez, ils ont. Verbes en -er : -e, -es, -e, -ons, -ez, -ent (nous mangeons, nous commençons). | `conjugationTable` ; `agreementChain` (du sujet à la terminaison) | conjugue *chanter* avec *nous* ; QCM tu as / tu a |
| `ce2-present-irreguliers.json` | `ce2-present-irreguliers` | 2 | Huit verbes à connaître par cœur : je fais, je vais, je dis, je viens, je peux, je vois, je veux, je prends ; attention à vous faites, vous dites, ils vont. | `conjugationTable` ; `letterTiles` | écris « vous faites » ; QCM ils allent / ils vont |

Les textes ci-dessus donnent l'intention ; la rédaction définitive se fait dans les fichiers JSON.

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
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `conjugationTable`, `agreementChain`, `wordClassSort` ; réponse « phrase » courte au quiz.
