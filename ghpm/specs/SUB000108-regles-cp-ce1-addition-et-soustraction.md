---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5R0U
url: https://github.com/georgesp/Educator/issues/9
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000108 — Règles CP–CE1 addition et soustraction
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 90
---

## User story

En tant que parent, je veux que les exercices correspondent à ce qui est travaillé en classe à cette période de l'année afin que l'application accompagne l'école.

## Contenu proposé (à confronter aux repères annuels officiels)

| Classe | Période | Compétence |
|---|---|---|
| CP | 1 | Additions dont le résultat est ≤ 10 |
| CP | 2 | Compléments à 10 ; soustractions dans 10 |
| CP | 3 | Additions jusqu'à 20 avec passage de la dizaine |
| CP | 4 | Soustractions jusqu'à 20 |
| CP | 5 | Additions et soustractions de dizaines entières jusqu'à 100 |
| CE1 | 1 | Additions de nombres à 2 chiffres sans retenue |
| CE1 | 2 | Soustractions de nombres à 2 chiffres sans retenue |
| CE1 | 3 | Additions à 2 chiffres avec retenue |
| CE1 | 4 | Soustractions à 2 chiffres avec retenue |
| CE1 | 5 | Additions et soustractions jusqu'à 1 000 (cas simples) |

## Critères d'acceptation

- [ ] Le tableau ci-dessus est vérifié contre les **repères annuels de progression** du CP et du CE1 publiés sur eduscol (version en vigueur à la rentrée 2026) et corrigé si besoin.
- [ ] Une compétence JSON par ligne, avec le champ `source` renseigné.
- [ ] Revue manuelle d'un échantillon de 20 exercices générés par compétence.
- [ ] Les classes CE2 à CM2 sans compétence affichent un message clair (« Les exercices de ta classe arrivent bientôt ») et proposent les compétences CE1.

## Notes techniques

- Sources : « Repères annuels de progression et attendus de fin d'année du CP à la troisième » (eduscol) ; programmes du cycle 2 en vigueur.
