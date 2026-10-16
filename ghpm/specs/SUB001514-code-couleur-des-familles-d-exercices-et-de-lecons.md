---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhA8GSk
rev: 2026-10-16T19:30:20Z
url: https://github.com/georgesp/Educator/issues/287
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001514 — Code couleur des familles d'exercices et de leçons
status: Done
size: M
estimate:
startDate:
endDate:
parent: "#15"
order: 625
---

## User story

En tant qu'enfant, je veux reconnaître d'un coup d'œil le type de chaque exercice et de chaque leçon (les nombres,
les additions, les sons…) afin de retrouver facilement ce que je cherche dans la liste.

## Contexte

Les écrans **Exercices** et **Leçons** (SUB000109, SUB001410, SUB001416) listent les compétences par période :
« En ce moment » en rouge, les révisions, puis toutes les autres. Au CE1, cela fait 27 cartes de maths qui se
ressemblent toutes : on ne distingue pas les additions des fractions ou des problèmes.

Piste retenue le 2026-10-15 (maquette « D ») : garder les sections actuelles et ajouter à chaque carte une **bande de
couleur avec le symbole de sa famille**. Les cartes « En ce moment » quittent le rouge pour un contour épais, afin de
ne pas mélanger deux codes couleur.

## Familles

Mathématiques, à partir de l'opération de la compétence :

| Famille | Symbole | Opérations |
|---|---|---|
| Les nombres | `123` | `numeration` (dont pair / impair) |
| Additions et soustractions | `+ −` | `addition`, `subtraction`, `mixed` |
| Multiplier | `×` | `multiplication` |
| Fractions, doubles et moitiés | `½` | `fraction`, `doubleHalf` |
| Problèmes | `?!` | `problem` |

Orthographe, à partir d'un nouveau champ `theme` des compétences :

| Thème | Symbole | Valeur | Compétences CP / CE1 |
|---|---|---|---|
| Les lettres et les sons | `[ou]` | `lettersAndSounds` | voyelles, consonnes, sons [ou], [on], [an], [ch], [in], [oi], [eu], [é], [o], [è], [s] |
| Les petites règles | `gu` | `spellingRules` | m devant m, b, p ; g, gu, ge |
| Les lettres muettes | `…t` | `silentLetters` | lettres muettes (CP, CE1) |
| Les accords | `s/x` | `agreement` | pluriel des noms (plus tard : féminin, accord du verbe) |
| Les mots à connaître | `★` | `wordsToKnow` | petits mots à connaître, mots invariables |

Les deux matières réutilisent les mêmes cinq couleurs (une par rang de famille) : elles ne sont jamais affichées
ensemble.

## Comportement attendu

- Chaque carte d'exercice et de leçon affiche à gauche une bande de la couleur de sa famille, avec son symbole.
- Sous le titre de l'écran, un **filtre** : un bouton « Tout » puis un bouton par famille présente (symbole + nom),
  qui sert aussi de légende. Un clic sur une famille n'affiche plus qu'elle (les sections vides disparaissent) ; un
  second clic ou « Tout » réaffiche tout. Le filtre est gardé dans l'adresse (`?famille=Numbers`) pour qu'un retour
  arrière le conserve ; il n'est pas affiché s'il n'y a qu'une famille.
- Les cartes « En ce moment » ont un contour épais au lieu du fond rouge ; le reste de l'écran ne change pas
  (sections, ordre, révisions avec le niveau d'origine, maîtrise).
- Une leçon prend la famille de sa première compétence (`skillIds[0]`, la principale).

## Critères d'acceptation

- [x] `theme` obligatoire pour une compétence d'orthographe et interdit en maths (schéma JSON et chargeur Rust) ;
      renseigné dans les 22 compétences d'orthographe CP / CE1.
- [x] La famille est calculée dans `educator-core` (`Skill::category`), testée pour chaque opération et chaque thème.
- [x] `category` ajoutée aux contrats des compétences et des leçons ; `web/src/api/types.ts` régénéré.
- [x] Bande de couleur et symbole sur les cartes des écrans Exercices et Leçons.
- [x] Filtre par famille sous le titre (« Tout » + une famille à la fois, état `aria-pressed`), gardé dans l'adresse.
- [x] Cartes « En ce moment » en contour épais, sans fond rouge.
- [x] Couleurs définies par des variables du thème (`--family-1` à `--family-5`, bloc `[data-theme="sport"]`) et
      noms des familles dans `web/src/i18n/fr.ts`.
- [x] Contraste du symbole sur sa couleur conforme WCAG AA.
- [x] La couleur n'est jamais seule : symbole et nom de la famille toujours présents (filtre, libellé accessible).

## Notes techniques

- `SkillParams::theme` (`Option<SpellingTheme>`), `SkillCategory` et `Skill::category` dans `educator-core::skills`.
- Commandes : `SkillResponse.category` et `LessonSummaryResponse.category` (`null` si la compétence de la leçon est
  introuvable).
- Front : prop `category` de `BigButton`, composant `FamilyFilter` et hook `useFamilyFilter`
  (`web/src/components/child/Family.tsx`).
