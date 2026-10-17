---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAQvOM
rev: 2026-10-12T19:26:30Z
url: https://github.com/georgesp/Educator/issues/232
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011934 — Leçons CE2 — Les sons et leurs écritures
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1251
---

Source : ancien lot 4 (SUB001611), découpé par classe (décision de Georges du 2026-10-12) ; réécrite le 2026-10-16
(revue du lot 4) : une fiche de leçons par thème de français, comme en mathématiques.

## User story

En tant qu'enfant de CE2, je veux une leçon courte sur chaque son et ses écritures avant de m'entraîner.

## Leçons

Une leçon par compétence de la fiche « Exercices CE2 — Les sons et leurs écritures » (8 à 12 slides, 3 à 5 minutes),
organisée comme les leçons du CE1 (SUB001413) : le son, ses écritures, où on les trouve dans le mot, un moyen de s'en
souvenir, deux manipulations, « Je retiens », quiz.

| Leçon (`content/lessons/spelling/`) | `skillIds` | Période | Contenu clé (« Je retiens ») | Interactions | Quiz (exemples) |
|---|---|---|---|---|---|
| `ce2-son-o.json` | `ce2-son-o` | 1 | [o] s'écrit o, au, eau ou ô ; « eau » souvent en fin de mot (un bateau) ; ô dans des mots à connaître (bientôt, un hôpital). | trier des mots par écriture (`soundSort`) ; placer la graphie (`letterTiles`) | écris « un chapeau » ; QCM aussi / ausi / ossi |
| `ce2-son-e-ferme.json` | `ce2-son-e-ferme` | 1 | [é] : é dans le mot ; -er à l'infinitif et dans des noms (le boulanger) ; -ez avec vous et dans quelques mots (le nez, assez) ; -ée pour des noms féminins (la journée). | `soundSort` ; `letterTiles` | complète une journ… ; QCM le dîné / le dîner |
| `ce2-son-e-ouvert.json` | `ce2-son-e-ouvert` | 2 | [è] : è, ê, ai, ei, et ; e sans accent devant deux consonnes (belle, verre, la chouette). | `soundSort` ; `wordFamily` (une fête, fêter) | complète une fen…tre ; QCM chouète / chouette |
| `ce2-son-an.json` | `ce2-son-an` | 2 | [an] : an ou en ; am et em devant m, b, p (une jambe, le temps — lettre muette). | `soundSort` ; `letterTiles` | écris « ensemble » ; QCM une jambe / une janbe |
| `ce2-son-s.json` | `ce2-son-s` | 3 | [s] : s, ss entre deux voyelles, c devant e et i, ç devant a, o, u, sc (la piscine), t devant ion (l'attention). | `soundSort` ; `letterTiles` | complète une le…on (ç) ; QCM une piscine / une pissine |

Les textes ci-dessus donnent l'intention ; la rédaction définitive (texte affiché court, texte lu plus détaillé, exemples
de mots du CE2) se fait dans les fichiers JSON.

## Méthode

Collecte des sources par Claude (eduscol, Réseau Canopé, Lumni, documents académiques), rédaction originale et
reformulée, relecture par Georges.

## Critères d'acceptation

- [ ] Une leçon par compétence, validée par `lesson.schema.json`, `skillIds` renseignés, sources listées.
- [ ] 8 à 12 slides, 3 à 5 minutes mesurées avec la voix ; au moins une animation `letterTiles` ou `soundSort` (test `every_spelling_skill_has_its_lesson_with_a_spelling_animation`).
- [ ] Au moins deux slides où l'enfant manipule, utilisables à la souris et au doigt (iPad, ADR 0006).
- [ ] Quiz de 3 à 5 questions avec explications, sans tolérance sur les accents ; fin de leçon : bouton « Je m'entraîne ».
- [ ] Rien n'est lu tout seul : texte et mots lus au clic (décision du 2026-10-15).
- [ ] Relues et validées par Georges.

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)

## Dépendances

- Format des leçons (SUB001409), lecteur (SUB001410), quiz (SUB001411), leçons d'orthographe du CE1 (SUB001413).
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `wordFamily`.
