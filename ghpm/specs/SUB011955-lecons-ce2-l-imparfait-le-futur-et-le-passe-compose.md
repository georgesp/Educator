---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXo64
url: https://github.com/georgesp/Educator/issues/312
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB011955 — Leçons CE2 — L'imparfait, le futur et le passé composé"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#119"
order: 1266
---

## User story

En tant qu'enfant de CE2, je veux une leçon courte sur chaque temps, avec le tableau de conjugaison animé, avant de
m'entraîner.

## Leçons

Une leçon par temps de la fiche « Exercices CE2 — L'imparfait, le futur et le passé composé » (8 à 12 slides, 3 à 5
minutes) ; la compétence de révision des temps est liée à la leçon du passé composé.

| Leçon (`content/lessons/spelling/`) | `skillIds` | Période | Contenu clé (« Je retiens ») | Interactions | Quiz (exemples) |
|---|---|---|---|---|---|
| `ce2-futur.json` | `ce2-futur` | 3 | Le futur dit ce qui se passera (demain). Terminaisons -rai, -ras, -ra, -rons, -rez, -ront ; pour les verbes en -er, j'ajoute à l'infinitif (je chanterai). À connaître : je serai, j'aurai, je ferai, j'irai, je viendrai, je pourrai, je verrai, je voudrai, je dirai, je prendrai. | `conjugationTable` (passage présent → futur) ; `wordClassSort` | conjugue *jouer* avec *nous* ; QCM j'irai / j'allerai |
| `ce2-imparfait.json` | `ce2-imparfait` | 4 | L'imparfait raconte le passé (autrefois, hier). Terminaisons -ais, -ais, -ait, -ions, -iez, -aient pour tous les verbes : j'étais, j'avais, je jouais, je faisais ; je mangeais, je commençais. | `conjugationTable` ; `letterTiles` | écris « nous étions » ; QCM je mangais / je mangeais |
| `ce2-passe-compose.json` | `ce2-passe-compose`, `ce2-temps-revision` | 5 | Le passé composé a deux mots : l'auxiliaire avoir ou être au présent et le participe passé (j'ai joué, j'ai fait, j'ai pris). Avec être (aller, venir), le participe s'accorde avec le sujet : elles sont venues. | `conjugationTable` ; `agreementChain` (sujet → participe avec être) ; `wordClassSort` (passé, présent, futur) | conjugue *chanter* avec *j'* ; QCM elles sont venu / venues |

Les textes ci-dessus donnent l'intention ; la rédaction définitive se fait dans les fichiers JSON.

## Méthode

Collecte des sources par Claude (eduscol, Réseau Canopé, Lumni, documents académiques), rédaction originale et
reformulée, relecture par Georges.

## Critères d'acceptation

- [ ] Une leçon par temps, validée par `lesson.schema.json`, `skillIds` renseignés (toutes les compétences du thème ont une leçon), sources listées.
- [ ] 8 à 12 slides, 3 à 5 minutes mesurées avec la voix ; au moins deux slides où l'enfant manipule, à la souris et au doigt (iPad, ADR 0006).
- [ ] Quiz de 3 à 5 questions avec explications, sans tolérance sur les accents ; fin de leçon : bouton « Je m'entraîne ».
- [ ] Rien n'est lu tout seul (décision du 2026-10-15).
- [ ] Relues et validées par Georges.

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)

## Dépendances

- Format des leçons (SUB001409), lecteur (SUB001410), quiz (SUB001411).
- Fiche « Extensions des animations et du quiz des leçons du CE2 » : `conjugationTable`, `agreementChain`, `wordClassSort` ; réponse « phrase » courte au quiz.
