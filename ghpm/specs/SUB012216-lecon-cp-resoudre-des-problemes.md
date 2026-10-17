---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACa-o
rev: 2026-10-17T13:46:51Z
url: https://github.com/georgesp/Educator/issues/228
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012216 — Leçon CP — Résoudre des problèmes en une étape
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2070
---

## User story

En tant qu'enfant de CP, je veux une leçon courte et animée qui m'apprend à comprendre une petite histoire de nombres et à y répondre, avant de m'entraîner.

## Leçon

- **Classe / période** : CP, période 2.
- **Compétences liées** (`skillIds`) : `cp-problems-one-step`, `cp-problems-one-step-100` — exercices : la fiche « Exercices CP — Résoudre des problèmes ». Les problèmes en deux étapes, multiplicatifs et de partage ont leur propre leçon (« Leçon CP — Problèmes en deux étapes et problèmes variés »).
- **Objectif** : écouter l'histoire, la raconter avec ses mots, la jouer ou la schématiser, comprendre ce que l'on cherche sans se fier à un seul mot, calculer et vérifier.
- **Durée visée** : 3 à 5 minutes, 10 slides.
- **Vocabulaire du programme** : « problème », « histoire », « question », « réponse ».

## Déroulé (scénario des slides)

| # | À l'écran et à la voix | Animation / interaction |
|---|---|---|
| 1 | **Titre** — « Résoudre des problèmes en une étape » : bandeau damier, la mascotte annonce ce qu'on va apprendre. | `titleCard` |
| 2 | **Une histoire** — « Sur le parking, il y a 36 voitures. 15 voitures partent. » : l'histoire s'écoute au clic, les voitures s'animent. | `groupObjects` |
| 3 | **Raconter avec ses mots** — Des voitures partent : il en reste moins qu'au début. On joue l'histoire en retirant les voitures. | `groupObjects` (remove) |
| 4 | **La réponse** — 36 − 15 = 21 : il reste 21 voitures sur le parking. | `calcChain` |
| 5 | **Le schéma** — Une barre pour le tout (36), une partie qui part (15), une partie qui reste (?). | `barModel` (tout / parties) |
| 6 | **À toi** — Interaction : « Léo a 24 casques. On lui en donne 13. » Joue l'histoire. | `groupObjects` (join, interactif) |
| 7 | **Attention au mot** — « Tom a perdu 5 drapeaux. Il lui en reste 12. Combien en avait-il ? » « Perdu » ne veut pas dire « on enlève » : on cherche le début, 12 + 5 = 17. | `barModel` (inconnue : le tout) |
| 8 | **À toi** — Interaction : choisis le schéma qui raconte l'histoire. | `barModel` (QCM) |
| 9 | **Vérifier** — Je relis la question : 21 voitures, c'est bien moins que 36 ; 17 drapeaux, c'est bien plus que 12. | `calcChain` |
| 10 | **Je retiens** — « J'écoute l'histoire, je la raconte avec mes mots, je la joue ou je la dessine. Je cherche ce qui se passe, pas un mot. Puis je calcule et je vérifie. » | `recapCard` |

Les textes ci-dessus donnent l'intention de chaque slide : la rédaction définitive (texte affiché court, texte lu plus
détaillé) se fait dans le fichier JSON. « Interaction » : l'enfant manipule (glisser, cliquer) ; la leçon attend son
geste et le guide doucement en cas d'erreur, sans message culpabilisant.

## Animations

- Utilisées : `barModel`, `calcChain`, `groupObjects`, `recapCard`, `titleCard`, décrites dans la fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- **Patte graphique** : charte « S1 · Grand Prix » (SUB000105) — fond blanc, surfaces `#F3F3F5`, texte graphite,
  rouge course `#D7262B` pour ce qui est nouveau ou important, Lexend pour le texte et Barlow Condensed italique pour
  les titres, bandeau damier, parallélogrammes ; les objets (voitures, casques, drapeaux, pneus) reprennent la palette
  des personnages (`content/characters/palette.json`). Mouvements courts (< 600 ms par étape), synchronisés avec la voix,
  remplacés par des fondus si le système demande de réduire les animations.

## Quiz de fin de leçon (SUB001411)

1. Sur le parking, 36 voitures ; 15 partent. Combien en reste-t-il ? (saisie)
2. Léo a 24 casques, on lui en donne 13. Combien en a-t-il ? (saisie)
3. Tom a perdu 5 drapeaux, il lui en reste 12. Combien en avait-il ? (QCM : 7, 12, 17)
4. Quel schéma correspond à l'histoire ? (QCM)

Chaque question a une explication courte, affichée après la réponse et lisible au clic (rien n'est lu tout seul, décision du 2026-10-15).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Les Fondamentaux (Réseau Canopé) — vidéos de mathématiques, pour les idées d'illustration uniquement](https://www.reseau-canope.fr/lesfondamentaux/)

Rédaction originale et reformulée : aucun texte ni visuel copié.

## Critères d'acceptation

- [x] Leçon `content/lessons/math/cp-problemes-une-etape.json`, validée par `lesson.schema.json`, `skillIds` = `cp-problems-one-step`, `cp-problems-one-step-100`.
- [ ] 8 à 12 slides, durée de 3 à 5 minutes mesurée avec la voix Piper.
- [ ] Au moins deux slides où l'enfant manipule (glisser, cliquer), utilisables à la souris et au doigt (iPad, ADR 0006).
- [x] Animations de la bibliothèque uniquement, dans la charte Grand Prix (couleurs et polices par variables CSS du thème).
- [x] Nombres et vocabulaire conformes au programme du cycle 2, sources listées dans le fichier.
- [x] Quiz de 3 à 5 questions avec explications ; fin de leçon : bouton « Je m'entraîne » vers les exercices liés.
- [ ] Relue et validée par Georges.

## Dépendances

- Format des leçons (SUB001409), lecteur de leçons (SUB001410), quiz (SUB001411).
- Fiche « Bibliothèque d'animations des leçons de mathématiques (charte Grand Prix) ».
- Mascotte (SUB001417) : facultative, la leçon fonctionne sans.

## Remarques et décisions

- Relecture (2026-10-17) : en période 2, les calculs de la leçon n'ont ni retenue ni cassage (12 + 5, 16 − 4, 18 − 13).
