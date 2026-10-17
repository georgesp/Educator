---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXo3E
url: https://github.com/georgesp/Educator/issues/310
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011953 — Leçons CE2 — Familles de mots, préfixes et suffixes"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1253
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte sur les familles de mots, les préfixes et les suffixes avant de
m'entraîner.

## Leçons

Une leçon par compétence de la fiche « Exercices CE2 — Familles de mots, préfixes et suffixes » (8 à 12 slides, 3 à 5 minutes).

| Leçon (`content/lessons/spelling/`) | `skillIds` | Période | Contenu clé (« Je retiens ») | Interactions | Quiz (exemples) |
|---|---|---|---|---|---|
| `ce2-lettres-muettes-famille.json` | `ce2-lettres-muettes-famille` | 2 | Pour trouver la lettre muette à la fin d'un mot, je cherche son féminin ou un mot de sa famille : grand → grande, un chant → chanter. | `wordFamily` (la lettre muette « s'entend » dans le dérivé) ; `letterTiles` | complète un ran… (rangée) ; QCM gri / gris / grit |
| `ce2-familles-mots.json` | `ce2-familles-mots` | 3 | Les mots d'une même famille ont le même radical et un sens proche : beau, beauté, embellir. Le radical m'aide à les écrire. | `wordFamily` ; `wordClassSort` (ranger par famille) | trouve l'intrus : terre, terrain, terrible ; écris « beauté » |
| `ce2-prefixes-suffixes.json` | `ce2-prefixes-suffixes` | 4 | Un préfixe se met devant le radical (dé-, in-, mal-, re-) et change souvent le sens ; im- devant m, b, p. Un suffixe se met après et peut changer la classe du mot (laver → le lavage). | `wordFamily` (préfixe, radical, suffixe en couleurs) ; `wordClassSort` | QCM le contraire de « possible » ; écris le nom de « laver » en -age |

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
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `wordFamily`, `wordClassSort`.
