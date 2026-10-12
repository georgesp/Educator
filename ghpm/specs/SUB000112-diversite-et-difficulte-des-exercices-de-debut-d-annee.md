---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAAvqs
rev: 2026-10-11T15:22:47Z
url: https://github.com/georgesp/Educator/issues/73
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000112 — Diversité et difficulté des exercices de début d'année
status: In progress
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 121
---

Source : retours de Georges après les premiers essais du lot 1 (2026-10-11).

## User story

En tant qu'enfant de CP, je veux des exercices variés et à ma portée dès la rentrée afin de m'entraîner aussi bien à ajouter qu'à enlever.

## Constat

- Les séries manquent de diversité : on enchaîne des calculs qui se ressemblent.
- En période 1 du CP, il y a des additions et des soustractions, mais très peu de soustractions avec de petits nombres (autour de 10).
- Aujourd'hui, la période 1 du CP ne compte que deux compétences : `cp-add-sum-10` (additions seules) et `cp-add-sub-1-2` (ajouter ou enlever 1 ou 2, premier nombre jusqu'à 30, opération tirée au hasard). Les soustractions « jusqu'à 20 » n'arrivent qu'en période 3 (`cp-sub-within-20`).

## Comportement attendu

- **Nouvelle compétence de soustraction avec de petits nombres** (ex. `cp-sub-within-10` « Soustractions jusqu'à 10 ») : premier nombre de 2 à 10, résultat positif, formats « résultat » et « nombre manquant », clavier et QCM. Période proposée : 1 ou 2, à confirmer contre le programme du cycle 2 de 2024 ; si le choix vient d'Educator, il est signalé dans `source`.
- **Réévaluer la difficulté de la période 1 du CP** :
  - `cp-add-sub-1-2` : resserrer le premier nombre autour de 10 (ex. 0 à 12) en période 1, et équilibrer additions et soustractions dans une même série (au moins 4 soustractions sur 10).
  - Vérifier que `cp-add-sum-10` ne propose pas trop de calculs triviaux (`+ 0`, `1 + 1` répétés).
- **Plus de variété dans une série** (moteur, `SeriesGenerator`) :
  - pas deux fois le même calcul dans une série ;
  - pas deux fois de suite la même forme (même opération et même format) quand la compétence en offre plusieurs ;
  - alternance des formats « résultat » et « nombre manquant » quand la compétence les autorise ;
  - éviter qu'un même opérande revienne plus de 3 fois dans une série.

## Critères d'acceptation

- [x] Nouvelle compétence de soustraction jusqu'à 10 disponible au CP, avec sa `source`.
- [x] En période 1 du CP, une série de `cp-add-sub-1-2` contient au moins 4 soustractions sur 10.
- [x] Tests de propriété du moteur : aucun doublon dans une série, pas deux formes identiques consécutives quand la compétence en a plusieurs (sur un grand nombre de graines).
- [ ] `docs/specs/echantillons-lot1.md` régénéré pour les compétences modifiées et relu par Georges.

## Notes techniques

- Règles dans `content/skills/math/` (validées par `skill.schema.json`), moteur dans `educator-core` (`generate_series`).
- Si le lot 1bis (migration Rust) est déjà engagé, les règles de variété sont portées dans `educator-core` avec leurs tests.

## Implémentation (2026-10-11)

- Nouvelle compétence `cp-sub-within-10` « Soustractions jusqu'à 10 », CP période 1 : premier nombre de 2 à 10,
  second de 1 à 9, ni `− 0` ni `n − n` (résultat ≥ 1), formats « résultat » et « nombre manquant », clavier et QCM
  (45 exercices différents). Le programme de 2024 ne place pas cette notion dans une période précise : la période 1
  est un choix d'Educator, signalé dans `source`.
- `cp-add-sub-1-2` : premier nombre de 0 à 12 (au lieu de 0 à 30), résultat jusqu'à 14 (48 exercices différents).
- `cp-add-sum-10` vérifiée : `+ 0` était déjà exclu, et un même calcul (`1 + 1`) ne peut plus revenir dans une
  série. Elle reste à 25 exercices différents.
- Moteur (`educator-core`, `generate_series`) :
  - une compétence `mixed` a autant d'additions que de soustractions (5 sur 10) ;
  - jamais deux fois de suite la même forme (opération et format) quand la compétence en offre plusieurs ;
  - pour une seule opération avec les deux formats, « résultat » et « nombre manquant » alternent ;
  - un même nombre ne revient pas plus de 3 fois comme opérande. La règle ne compte pas les opérandes à moins de
    4 valeurs, comme « 1 ou 2 ». Le tirage est glouton : il refait jusqu'à 30 essais avec la même graine avant
    d'assouplir cette seule règle. Le résultat reste déterministe.
- Tests de propriété (`crates/educator-core/tests/generator.rs`, 2 000 graines par compétence) : pas de doublon,
  pas deux formes identiques de suite, alternance des formats, 5 soustractions sur 10, 3 répétitions au plus.
  Tests du contenu (`content_validation.rs`) pour `cp-sub-within-10` et `cp-add-sub-1-2`.
- `docs/specs/echantillons-lot1.md` est régénéré pour toutes les compétences, puisque le moteur a changé, avec le
  nouveau programme `cargo run -p educator-content --example samples`.
- Reste à faire : la relecture des échantillons par Georges.
