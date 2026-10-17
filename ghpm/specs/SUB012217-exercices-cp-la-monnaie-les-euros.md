---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdp4o
url: https://github.com/georgesp/Educator/issues/327
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB012217 — Exercices CP — La monnaie : les euros"
status: Backlog
size: M
estimate:
startDate:
endDate:
parent: "#122"
order: 2075
---

Source : décision de Georges du 2026-10-16 (grandeurs et mesures), étendue au CP par la revue du 2026-10-17.

## User story

En tant qu'enfant de CP, je veux m'entraîner à compter l'argent d'un porte-monnaie, à faire une somme et à rendre la
monnaie, afin de suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CP, monnaie : utiliser le lexique de la monnaie ; comparer les valeurs de deux ensembles
  de pièces, ou de pièces et de billets ; déterminer la valeur en euros d'un ensemble de pièces et de billets ;
  constituer une somme donnée avec des pièces et des billets ; simuler des achats avec des pièces et des billets fictifs,
  rendre la monnaie.
- En numération, le programme cite aussi la monnaie fictive comme matériel (pièces de 1 €, billets de 10 €).

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cp-money-value` | 4 | Compter l'argent | Porte-monnaie de pièces de 1 € et 2 € et de billets de 5, 10, 20 et 50 € (total ≤ 100 €) : combien d'euros ? Comparer deux porte-monnaie (<, >, =) | `measureRead`, `compare` |
| `cp-money-make` | 5 | Payer et rendre la monnaie | Faire une somme ≤ 50 € en glissant pièces et billets ; achat d'un objet ≤ 20 € payé avec un billet de 10 € ou de 20 € : combien te rend-on ? (en complétant du prix jusqu'au billet) | `coinsMake`, `result` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- 2 billets de 10 € et 3 pièces de 1 € : combien d'euros ? → 23 €.
- Qui a le plus d'argent ? (A : un billet de 20 € et un de 5 € ; B : deux billets de 10 € et une pièce de 2 €) → A.
- Fais 35 € avec les billets et les pièces.
- Un livre coûte 7 €. Tu paies avec un billet de 10 €. Combien te rend-on ? → 3 €.

## Évolutions du moteur et de l'écran de série

- Composant `coins` et format `coinsMake` (fiche « Exercices CE2 — Payer et rendre la monnaie », SUB011942 ; dessins
  originaux, pas de reproduction des pièces et billets réels) limités aux euros entiers : pièces de 1 € et 2 €, billets
  de 5, 10, 20 et 50 €.
- Type « mesure » (grandeur `money`, fiche SUB011940) : au CP, la réponse est un nombre entier d'euros, une seule case.
- Retours qui rappellent la numération : un billet de 10 €, c'est une dizaine d'euros.
- Si le lot 7 passe avant le lot 4, cette fiche porte ces évolutions (voir « Ordre de réalisation » dans la fiche du lot).

## Distracteurs (QCM)

Nombre de pièces et de billets compté au lieu de leur valeur (5 objets → 5 €) ; prix ajouté au lieu de la monnaie rendue
(7 + 10 = 17) ; billet de 5 € compté comme 1.

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md` (nouveau fichier ; les 11 compétences du lot 1 restent dans `echantillons-lot1.md`), relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement (décision du 2026-10-15).
- [ ] Exercices jouables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [ ] Les compétences sont liées à la fiche « Leçon CP — La monnaie : les euros » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Résumé par niveau du programme de mathématiques du cycle 2 (2024) — attendus de grandeurs et mesures du CP, à vérifier sur le PDF officiel](https://maitresselunicole.fr/wp-content/uploads/2025/03/resume-par-niveaux-programme-de-mathematiques-cycle-2-2024-1.pdf)

## Remarques et décisions

- Euros entiers seulement au CP (choix d'Educator, signalé dans `source`) : les centimes viennent dans les classes suivantes.
- Attendus à vérifier sur le PDF officiel au moment de rédiger les fichiers JSON.
