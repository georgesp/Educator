---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhACX74
rev: 2026-10-11T18:48:58Z
url: https://github.com/georgesp/Educator/issues/90
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB000122 — Exercices CE1 — Pair, impair, doubles et moitiés"
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 1240
---

## User story

En tant qu'enfant de CE1, je veux m'entraîner à reconnaître les nombres pairs et impairs et connaître doubles et moitiés afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CE1 : notion de parité (dire si un nombre est pair ou impair ; ex. les nombres pairs entre 767 et 778).
- Faits multiplicatifs usuels : doubles de 1 à 15 et de 20, 25, 30, 35, 40, 45, 50, 100, 150, 200, 250, 300, 500 ; moitiés des nombres pairs de 2 à 30, des dizaines de 40 à 100 et des centaines de 200 à 1 000 ; multiples de 25 (25, 50, 75, 100).
- Calcul mental : moitié d'un nombre pair en le décomposant (46 = 40 + 6, donc 23).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `ce1-parity` | 2 | Pair ou impair | Nombre ≤ 1 000 : pair ? (oui / non) ; trouver les nombres pairs d'une liste ou d'un intervalle | `yesNo`, `choice` |
| `ce1-double-half-facts` | 3 | Doubles et moitiés à connaître | Doubles et moitiés de la liste du programme ; multiples de 25 | `phrase`, `result` |
| `ce1-half-even` | 4 | Moitié d'un nombre pair | Moitié d'un nombre pair ≤ 200 en décomposant (46 → 23, 84 → 42, puis 56 → 28) | `phrase` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 573 est-il pair ? → non.
- Le double de 35 → 70.
- La moitié de 300 → 150.
- La moitié de 46 → 23.

## Évolutions du moteur et de l'écran de série

- Format `phrase` : consigne en mots générée (« Quel est le double de 35 ? »), calcul interne en × 2 ou ÷ 2 — créé ici, réutilisé au CP et au CE2.
- Format `yesNo` (deux grands boutons Oui / Non) et `choice` à plusieurs bonnes réponses — créés ici, réutilisés au CM1 et au CM2.

## Distracteurs (QCM)

Parité jugée sur le premier chiffre, double et moitié confondus, moitié prise chiffre par chiffre sur un nombre à dizaines impaires (56 → 23).

## Critères d'acceptation

- [x] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [x] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [x] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-ce1.md`, relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper.
- [ ] La compétence est liée à la fiche « Leçon CE1 — Pair, impair, doubles et moitiés » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)

## Réalisation (2026-10-11)

- Format « plusieurs bonnes réponses » nommé `multiChoice` dans le JSON (pour ne pas le confondre avec le mode de réponse `choice`).
- Restent ouverts : relecture des échantillons (`docs/specs/echantillons-ce1.md`), lecture par Piper (voix coupée dans les maths pour l'instant), lien avec la leçon (format des leçons au lot 2).
