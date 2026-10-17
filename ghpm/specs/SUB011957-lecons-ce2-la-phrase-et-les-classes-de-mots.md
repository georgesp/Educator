---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXo90
url: https://github.com/georgesp/Educator/issues/314
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011957 — Leçons CE2 — La phrase et les classes de mots
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1262
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte sur la phrase et sur chaque classe de mots avant de m'entraîner.

## Leçons

Une leçon par compétence de la fiche « Exercices CE2 — La phrase et les classes de mots » (8 à 12 slides, 3 à 5 minutes).

| Leçon (`content/lessons/spelling/`) | `skillIds` | Période | Contenu clé (« Je retiens ») | Interactions | Quiz (exemples) |
|---|---|---|---|---|---|
| `ce2-phrase-groupes.json` | `ce2-phrase-groupes` | 1 | Le verbe change quand je change le temps (hier, demain). Le groupe sujet répond à « C'est … qui ». Le reste, ce sont les compléments. Il, elle, ils, elles remplacent un groupe sujet. | `sentenceGroups` ; `homophoneSwap` (groupe sujet ↔ pronom) | clique sur le verbe ; QCM le pronom de « Les filles » |
| `ce2-classes-nom-det-adj.json` | `ce2-classes-nom-det-adj` | 2 | Le nom désigne une personne, un animal, une chose (nom propre avec une majuscule) ; le déterminant est devant le nom ; l'adjectif dit comment il est. | `wordClassSort` ; `sentenceGroups` | QCM la classe de « petit » ; clique sur le nom propre |
| `ce2-classes-verbe-pronom.json` | `ce2-classes-verbe-pronom` | 2 | Le verbe se conjugue ; son infinitif sert à le nommer (je chante → chanter). Je, tu, il, elle, on, nous, vous, ils, elles sont des pronoms personnels sujets. | `wordClassSort` ; `conjugationTable` (le verbe change avec la personne) | QCM la classe de « nous » ; écris l'infinitif de « il chante » |
| `ce2-types-formes-phrases.json` | `ce2-types-formes-phrases` | 3 | Une phrase déclarative finit par un point, une interrogative par « ? », une impérative donne un ordre. À la forme négative, j'encadre le verbe avec ne … pas (ne … plus, ne … jamais). Les paroles sont entre guillemets. | `sentenceGroups` (la négation encadre le verbe) ; `wordClassSort` (trier des phrases par type) | QCM le type de « Ferme la porte. » ; QCM la phrase négative juste |
| `ce2-adverbe.json` | `ce2-adverbe` | 4 | L'adverbe précise le verbe ou l'adjectif (très, bien, souvent, lentement) ; il ne change jamais : c'est un mot invariable. | `wordClassSort` (variables / invariables) ; `sentenceGroups` | clique sur l'adverbe ; QCM variable ou invariable |

Les textes ci-dessus donnent l'intention ; la rédaction définitive se fait dans les fichiers JSON.

## Méthode

Collecte des sources par Claude (eduscol, Réseau Canopé, Lumni, documents académiques), rédaction originale et
reformulée, relecture par Georges ; termes de la terminologie grammaticale d'eduscol.

## Critères d'acceptation

- [ ] Une leçon par compétence, validée par `lesson.schema.json`, `skillIds` renseignés, sources listées.
- [ ] 8 à 12 slides, 3 à 5 minutes mesurées avec la voix ; au moins deux slides où l'enfant manipule, à la souris et au doigt (iPad, ADR 0006).
- [ ] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne ».
- [ ] Rien n'est lu tout seul (décision du 2026-10-15).
- [ ] Relues et validées par Georges.

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)

## Dépendances

- Format des leçons (SUB001409), lecteur (SUB001410), quiz (SUB001411).
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `sentenceGroups`, `wordClassSort`, `conjugationTable`, `homophoneSwap`.
