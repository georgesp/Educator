---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACa1g
rev: 2026-10-17T13:46:42Z
url: https://github.com/georgesp/Educator/issues/220
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012208 — Exercices CP — Résoudre des problèmes
status: In progress
size: L
estimate:
startDate:
endDate:
parent: "#122"
order: 2060
---

## User story

En tant qu'enfant de CP, je veux m'entraîner à résoudre des petits problèmes, que je peux écouter, afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CP : au moins dix problèmes par semaine, données de 0 à 100 ; l'enfant reformule l'histoire avec ses mots et rencontre des énoncés qui n'induisent pas l'opération à faire.
- Problèmes parties-tout et transformations en une étape (43 cerises moins 18 ; 36 oiseaux dont 21 restent) ; additifs en deux étapes (nombres ≤ 30 : bus à 29 enfants, 12 descendent, 7 montent) ; multiplicatifs en une étape (≤ 30 : trouver le tout à partir de parts égales) ; partages équitables (24 élèves en équipes de 4 ; 18 images pour 3 enfants).
- « Des soustractions par manipulation et cassage de dizaines sont effectuées dès la période 3. »

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cp-problems-one-step` | 2 | Problèmes en une étape | Gabarits parties-tout et transformation (gain, perte), nombres ≤ 59 ; soustractions sans cassage de dizaine (36 − 15) ; inconnue sur l'état final, puis sur la transformation | `problem` |
| `cp-problems-one-step-100` | 3 | Problèmes en une étape jusqu'à 100 | Mêmes sortes, nombres ≤ 100, soustractions avec cassage de dizaine (43 − 18) ; inconnue aussi sur l'état initial (« il en a perdu 5, il lui en reste 12 ») | `problem` |
| `cp-problems-two-steps` | 5 | Problèmes en deux étapes | Gabarits additifs en deux étapes, données et résultat intermédiaire ≤ 30 | `problem` |
| `cp-problems-mixed` | 5 | Problèmes variés | Mélange des sortes du CP, nombres ≤ 30 : additifs en une étape, multiplicatifs en une étape, partages et groupements (sans reste) ; l'enfant choisit lui-même l'opération | `problem` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Sur le fil, il y a 36 oiseaux. 15 s'envolent. Combien en reste-t-il ? → 21 (période 2, sans cassage).
- Léa a 43 cerises, elle en mange 18. Combien en reste-t-il ? → 25 (à partir de la période 3).
- Tom a perdu 5 billes. Il lui en reste 12. Combien en avait-il ? → 17.
- Dans le bus, il y a 29 enfants. 12 descendent, 7 montent. Combien d'enfants dans le bus ? → 24.
- 3 paquets de 7 biscuits : combien de biscuits ? → 21.
- 24 élèves font des équipes de 4. Combien d'équipes ? → 6.

## Évolutions du moteur et de l'écran de série

- Format `problem` (lot 1, CE1) adapté au CP : énoncé lu au clic sur « Écouter l'histoire » (bouton toujours visible à côté de l'énoncé, réécoute illimitée ; rien n'est lu tout seul, décision du 2026-10-15), phrases courtes, image de la situation, réponse au pavé numérique.
- Gabarits CP dans `content/problems/` : contrainte « sans cassage de dizaine » pour la période 2 ; inconnue sur l'état initial, la transformation ou l'état final ; sortes `product` et `sharing` existantes (CE1) reprises avec des nombres ≤ 30 ; `mixed` pour `cp-problems-mixed`.
- Énoncés non inducteurs : chaque sorte a des gabarits où le mot de l'histoire ne donne pas l'opération (« perdu » avec une addition, « gagné » avec une soustraction) ; au moins un énoncé sur quatre dans `cp-problems-one-step-100` et `cp-problems-mixed`.

## Distracteurs (QCM)

Mauvaise opération, une seule étape faite (17 ou 41), nombre de l'énoncé recopié, opération « suggérée » par un mot de l'histoire (12 − 5 = 7 pour « il en a perdu 5, il lui en reste 12 »).

## Critères d'acceptation

- [x] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [x] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [x] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md` (nouveau fichier ; les 11 compétences du lot 1 restent dans `echantillons-lot1.md`), relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement (décision du 2026-10-15).
- [ ] Exercices jouables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [x] Tests du générateur : aucune soustraction avec cassage dans `cp-problems-one-step` ; aucune donnée ni résultat intermédiaire au-dessus de 30 dans `cp-problems-two-steps` ; au moins un énoncé non inducteur sur quatre là où c'est demandé.
- [x] `cp-problems-one-step` et `cp-problems-one-step-100` sont liées à la fiche « Leçon CP — Résoudre des problèmes en une étape », `cp-problems-two-steps` et `cp-problems-mixed` à la fiche « Leçon CP — Problèmes en deux étapes et problèmes variés » (`skillIds` des leçons).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Remarques et décisions

- Au CP, l'enfant ne lit pas toujours seul : l'énoncé est lu au clic (rien n'est lu tout seul, décision du 2026-10-15) et peut être réécouté autant de fois que l'enfant veut ; les mots écrits restent courts.
- Les problèmes multiplicatifs et de partage sont aussi travaillés hors contexte par `cp-times-as-repeated-add` et `cp-share-group` ; `cp-problems-mixed` les mêle aux problèmes additifs pour que l'enfant choisisse l'opération.
- Revue du 2026-10-17 : périodes et plages alignées sur le programme (cassage dès la période 3), deux compétences ajoutées.
- Implémentation (2026-10-17) : option `noCarry` des compétences de problèmes (ni retenue ni cassage) et liste `templates` (gabarits choisis) ; champ `grades` des gabarits : les gabarits multiplicatifs du CP disent « fois » au lieu de × ; nouveaux gabarits (oiseaux, cerises, « il a perdu 5, il lui en reste 12 », bus qui descend puis monte) ; `answer > 1` ajouté à quatre gabarits pour ne plus écrire « 1 cerises » ou « 1 euros ».
