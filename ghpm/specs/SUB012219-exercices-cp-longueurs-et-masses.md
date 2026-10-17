---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdp-Q
url: https://github.com/georgesp/Educator/issues/329
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012219 — Exercices CP — Longueurs et masses
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#122"
order: 2073
---

Source : décision de Georges du 2026-10-16 (grandeurs et mesures), étendue au CP par la revue du 2026-10-17.

## User story

En tant qu'enfant de CP, je veux m'entraîner à comparer et à mesurer des longueurs et à comparer des masses, afin de
suivre le programme de ma classe.

## Ce que dit le programme

- Programme du cycle 2 (2024), CP, longueurs : utiliser le lexique des longueurs ; comparer des objets et des segments
  selon leur longueur ; mesurer la longueur d'un segment avec une règle graduée ; connaître et utiliser le mètre et le
  centimètre (m, cm) ; connaître quelques longueurs de référence ; savoir qu'un mètre est égal à cent centimètres.
- Masses : utiliser le lexique des masses ; comparer des objets selon leur masse.

## Compétences à créer (`content/skills/math/`)

| Compétence (`id`) | Période | Titre | Règle de génération | Formats |
|---|---|---|---|---|
| `cp-length-compare-measure` | 3 | Comparer et mesurer des longueurs | Deux ou trois segments ou objets dessinés (crayons, rubans, voitures) : le plus long, le plus court, ranger du plus court au plus long ; segment posé sur une règle graduée en centimètres, début au 0, longueur entière de 2 à 15 cm | `compare`, `order`, `measureRead` |
| `cp-mass-compare` | 4 | Comparer des masses | Balance à plateaux dessinée : quel objet est le plus lourd, le plus léger ? Deux pesées → ranger trois objets ; le plus gros objet n'est pas toujours le plus lourd | `choice` (clic sur l'image), `order` |
| `cp-length-m-cm` | 5 | Le mètre et le centimètre | 1 m = 100 cm ; choisir l'unité (m ou cm) ; choisir la longueur vraisemblable parmi trois (banque d'objets de référence) ; comparer 1 m et un nombre de centimètres ≤ 100 | `estimate`, `compare` |

Quand le programme ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé dans
le champ `source` (comme pour le CP et le CE1).

## Exemples d'exercices

- Quel ruban est le plus long ? (deux rubans dessinés, décalés)
- Quelle est la longueur du crayon ? (crayon posé sur la règle, de 0 à 9) → 9 cm.
- Sur la balance, le plateau du ballon descend. Quel objet est le plus lourd ? → le ballon.
- Une porte mesure : 2 cm, 2 m ou 20 m ? → 2 m.
- 1 m … 80 cm → >.

## Évolutions du moteur et de l'écran de série

- Type « mesure », réponse en plusieurs cases, formats `measureRead` et `estimate`, `compare` étendu aux mesures, voix des
  unités : fiche « Exercices CE2 — Longueurs » (SUB011940), dans une version CP : unités m et cm seulement, règle graduée
  en centimètres sans millimètres, longueurs entières. Si le lot 7 passe avant le lot 4, cette fiche porte ces évolutions
  (voir « Ordre de réalisation » dans la fiche du lot).
- Comparer à l'œil : objets dessinés parfois décalés (pas alignés au départ) pour que l'enfant regarde les deux bouts ;
  écart d'au moins 20 % entre deux longueurs à comparer sans règle.
- Balance `scale` (fiche « Extensions des animations et du quiz des leçons du CE2 », SUB011946) en mode plateaux, sans
  masses marquées : le plateau le plus lourd descend ; format `choice` sur image (fiche « Exercices CP — Compter de 1 en 1,
  de 2 en 2, de 10 en 10 et à rebours »).
- Banque d'objets de référence `content/measures/references.json` (fiche SUB011940) : objets familiers marqués `CP`.
- Famille « Grandeurs et mesures » du code couleur (SUB001514).

## Distracteurs (QCM)

Lecture de la règle à partir de 1 au lieu de 0 ; objet le plus gros pris pour le plus lourd ; m et cm confondus (une porte
de 2 cm) ; comparaison des seuls nombres sans les unités (80 > 1, donc 80 cm > 1 m).

## Critères d'acceptation

- [ ] Une compétence JSON par ligne du tableau, validée par `skill.schema.json`, avec sa `source`.
- [ ] Chaque compétence produit au moins 20 exercices distincts (test existant du contenu).
- [ ] Tests de propriété du générateur sur les nouvelles plages (2 000 graines par compétence), y compris les règles de variété de SUB000112.
- [ ] Échantillon de 20 exercices par compétence ajouté à `docs/specs/echantillons-cp.md` (nouveau fichier ; les 11 compétences du lot 1 restent dans `echantillons-lot1.md`), relu par Georges.
- [ ] Consignes et nombres lus correctement par Piper (ordinateur) et par la voix du système (iPad, ADR 0005), au clic seulement (décision du 2026-10-15).
- [ ] Exercices jouables au doigt sur iPad (ADR 0006) comme à la souris et au clavier.
- [ ] Les compétences sont liées à la fiche « Leçon CP — Longueurs et masses » (`skillIds` de la leçon).

## Sources

- [Programme de mathématiques du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%204%20%E2%80%93%20Programme%20de%20math%C3%A9matiques%20du%20cycle%202-403821.pdf)
- [Résumé par niveau du programme de mathématiques du cycle 2 (2024) — attendus de grandeurs et mesures du CP, à vérifier sur le PDF officiel](https://maitresselunicole.fr/wp-content/uploads/2025/03/resume-par-niveaux-programme-de-mathematiques-cycle-2-2024-1.pdf)

## Remarques et décisions

- Les attendus ci-dessus viennent d'un résumé du programme par une enseignante : les formulations et les éventuelles
  périodes sont à vérifier sur le PDF officiel au moment de rédiger les fichiers JSON.
- Les unités de masse (g, kg) ne sont pas au programme du CP : on compare sans mesurer.
