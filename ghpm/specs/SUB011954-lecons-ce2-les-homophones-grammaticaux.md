---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXo5I
url: https://github.com/georgesp/Educator/issues/311
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011954 — Leçons CE2 — Les homophones grammaticaux
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1257
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte avec une astuce pour chaque paire d'homophones avant de m'entraîner.

## Leçons

Une leçon par paire de la fiche « Exercices CE2 — Les homophones grammaticaux » (8 à 12 slides, 3 à 5 minutes) ; la
compétence de révision est liée à la seconde leçon.

| Leçon (`content/lessons/spelling/`) | `skillIds` | Période | Contenu clé (« Je retiens ») | Interactions | Quiz (exemples) |
|---|---|---|---|---|---|
| `ce2-homophones-a-et.json` | `ce2-homophones-a-et` | 1 | « a » est le verbe avoir : je peux dire « avait ». « est » est le verbe être : je peux dire « était ». Sinon j'écris « à » ou « et ». | `homophoneSwap` (le remplacement fonctionne ou non) ; `wordClassSort` (ranger des phrases dans la colonne a ou à) | QCM Léo … un vélo (a / à) ; QCM Le ciel … bleu (et / est) |
| `ce2-homophones-son-on.json` | `ce2-homophones-son-on`, `ce2-homophones-revision` | 2 | « sont » et « ont » sont des verbes : je peux dire « étaient », « avaient ». « son » peut se remplacer par « mon », « on » par « il ». | `homophoneSwap` ; `wordClassSort` | QCM Ils … partis (son / sont) ; QCM … joue dehors (On / Ont) |

Les textes ci-dessus donnent l'intention ; la rédaction définitive se fait dans les fichiers JSON.

## Méthode

Collecte des sources par Claude (eduscol, Réseau Canopé, Lumni, documents académiques), rédaction originale et
reformulée, relecture par Georges.

## Critères d'acceptation

- [ ] Une leçon par paire, validée par `lesson.schema.json`, `skillIds` renseignés (toutes les compétences du thème ont une leçon), sources listées.
- [ ] 8 à 12 slides, 3 à 5 minutes mesurées avec la voix ; au moins deux slides où l'enfant manipule, à la souris et au doigt (iPad, ADR 0006).
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne ».
- [ ] Rien n'est lu tout seul (décision du 2026-10-15).
- [ ] Relues et validées par Georges.

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)

## Dépendances

- Format des leçons (SUB001409), lecteur (SUB001410), quiz (SUB001411).
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `homophoneSwap`, `wordClassSort`.
