---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5R0U
rev: 2026-10-10T19:29:04Z
url: https://github.com/georgesp/Educator/issues/9
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000108 — Règles CP–CE1 addition et soustraction
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 90
---

## User story

En tant que parent, je veux que les exercices correspondent à ce qui est travaillé en classe à cette période de l'année afin que l'application accompagne l'école.

## Contenu retenu (programme de mathématiques du cycle 2 de 2024)

Pour les mathématiques, les repères annuels de progression de 2020 sont remplacés par le **programme du cycle 2**
(arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024), en vigueur depuis la rentrée 2025. Le tableau suit ce
programme. Quand il ne place pas une notion dans une période précise, la période est un choix d'Educator, signalé
dans le champ `source` de la compétence.

| Classe | Période | Compétence (`id`) |
|---|---|---|
| CP | 1 | Additions jusqu'à 10 (`cp-add-sum-10`) ; Ajouter ou enlever 1 ou 2 (`cp-add-sub-1-2`) |
| CP | 2 | Tables d'addition, dans les deux sens (`cp-add-tables`) |
| CP | 3 | Ajouter ou enlever des dizaines (`cp-add-sub-tens`) ; Soustractions jusqu'à 20 (`cp-sub-within-20`) |
| CP | 4 | Ajouter 9 (`cp-add-9`) ; Ajouter un petit nombre en passant la dizaine (`cp-add-small-crossing`) ; Compléments à la dizaine (`cp-complement-next-ten`) |
| CP | 5 | Additions de deux nombres plus petits que 100 (`cp-add-2-digits`) ; Enlever un petit nombre à une dizaine (`cp-sub-from-tens`) |
| CE1 | 1 | Tables d'addition (`ce1-add-tables`) ; Additions de nombres plus petits que 100 (`ce1-add-2-digits`) |
| CE1 | 2 | Ajouter 9, 19 ou 29 (`ce1-add-9-19-29`) ; Enlever 9 (`ce1-sub-9`) |
| CE1 | 3 | Enlever un petit nombre (`ce1-sub-small`) ; Soustractions de nombres plus petits que 100 (`ce1-sub-2-digits`) |
| CE1 | 4 | Ajouter ou enlever des dizaines et des centaines (`ce1-add-sub-tens-hundreds`) |
| CE1 | 5 | Additions jusqu'à 1 000 (`ce1-add-3-digits`) ; Soustractions jusqu'à 1 000 (`ce1-sub-3-digits`) |

Les « compléments à 10 » seuls ne permettent que 5 exercices différents, sous le minimum de 20. Ils sont donc
intégrés aux compléments à la dizaine supérieure, comme le prévoit le programme (« pour 74, il faut ajouter 6 »).

## Critères d'acceptation

- [x] Le tableau ci-dessus est vérifié contre les **repères annuels de progression** du CP et du CE1 publiés sur eduscol (version en vigueur à la rentrée 2026) et corrigé si besoin.
- [x] Une compétence JSON par ligne, avec le champ `source` renseigné.
- [x] Revue manuelle d'un échantillon de 20 exercices générés par compétence.
- [x] Les classes CE2 à CM2 sans compétence affichent un message clair (« Les exercices de ta classe arrivent bientôt ») et proposent les compétences CE1.

## Notes techniques

- Sources : « Repères annuels de progression et attendus de fin d'année du CP à la troisième » (eduscol) ; programmes du cycle 2 en vigueur.

## Implémentation (2026-10-11)

- 19 compétences dans `content/skills/math/`, chacune avec sa `source`.
- Revue : 20 exercices générés par compétence dans `docs/specs/echantillons-lot1.md`, relus par Claude. Cette relecture a conduit à ajuster les distracteurs des nombres manquants (voir SUB000107). Une relecture par Georges reste conseillée.
- CE2 à CM2 : le host renvoie les compétences du CE1 (`skillsGrade` dans `GET /api/ui/skills`), et l'écran des exercices affiche « Les exercices de ta classe arrivent bientôt. En attendant, voici ceux du CE1. » et propose les compétences du CE1.
