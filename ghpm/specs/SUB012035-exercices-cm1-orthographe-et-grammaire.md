---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAQvSg
rev: 2026-10-12T19:26:32Z
url: https://github.com/georgesp/Educator/issues/233
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB012035 — Exercices CM1 — Orthographe et grammaire
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#120"
order: 1620
---

Source : ancien lot 4 (SUB001607 à SUB001610), découpé par classe — décision de Georges du 2026-10-12. Contenu
réécrit le 2026-10-16 d'après le programme de français du cycle 3 (arrêté du 10 avril 2025).

## User story

En tant qu'enfant de CM1, je veux m'entraîner à l'orthographe, à la conjugaison et à la grammaire de ma classe afin d'écrire sans erreur.

## Ce que dit le programme

- **Orthographe lexicale** : écrire correctement les mots les plus fréquents en s'appuyant sur les régularités et la formation des mots ; s'appuyer sur la morphologie des mots pour les orthographier.
- **Accords** : repérer le sujet du verbe (notamment le nom noyau d'un groupe nominal) ; repérer le donneur d'accord du groupe nominal (le nom noyau) et systématiser la chaîne d'accords ; accorder le participe passé avec le sujet quand l'auxiliaire est être.
- **Conjugaison** : présent de l'indicatif, imparfait, futur, passé composé des verbes être et avoir ; composition du passé composé (auxiliaire + participe passé) ; verbes du 1er et du 2e groupe et faire, aller, dire, venir, pouvoir, voir, vouloir, prendre.
- **Grammaire** : consolider l'identification du verbe conjugué ; déterminants (articles définis et indéfinis, possessifs, démonstratifs) ; conjonctions de coordination ; adverbes les plus fréquents (leur caractère invariable est souligné) ; pronoms personnels sujets et compléments ; distinguer complément d'objet et complément circonstanciel ; COD et COI dans des phrases prototypiques ; groupes circonstanciels (sans les distinguer) ; trois types de phrases (déclaratif, interrogatif, impératif ou injonctif) et formes négative et exclamative.
- Le programme ne répartit pas ces objectifs par période et ne donne pas de liste d'homophones : périodes et homophones sont des choix d'Educator, signalés dans `source`.

**Pas au CM1** (ne pas en faire d'exercice) : accord de l'attribut, participe passé avec avoir, sujet inversé, phrase complexe (CM2) ; passé simple, plus-que-parfait (CM2) ; conditionnel (6e) ; préposition, complément du nom, attribut du sujet ; distinction des compléments circonstanciels par type (CM2).

## Compétences à créer (`content/skills/spelling/`)

| Compétence (`id`) | Période | Domaine | Contenu | Types d'exercices |
|---|---|---|---|---|
| `cm1-words-frequent` | 1 | Orthographe lexicale | Mots les plus fréquents (liste de fréquence d'eduscol, révision du cycle 2) | Dictée de mots, « choisis la bonne écriture » |
| `cm1-homophones-review` | 1 | Homophones | Révision des paires du CE2 (fiche « Exercices CE2 — Orthographe et grammaire ») | Phrase à trou avec choix |
| `cm1-gram-verb-subject` | 1 | Grammaire | Identifier le verbe conjugué et son sujet, y compris le nom noyau d'un groupe nominal sujet | « Clique sur… » |
| `cm1-gram-sentence-types` | 1 | Grammaire | Types de phrases (déclarative, interrogative, impérative) et formes (négative, exclamative) | « Choisis… », transformer (QCM) |
| `cm1-conj-present` | 1 | Conjugaison | Présent : 1er et 2e groupe, être, avoir, faire, aller, dire, venir, pouvoir, voir, vouloir, prendre | Conjuguer (saisie) |
| `cm1-words-morphology` | 2 | Orthographe lexicale | Familles de mots, préfixes et suffixes pour écrire (lettre finale muette retrouvée par un dérivé : grand → grande) | Mots à trous, dictée de mots |
| `cm1-agree-subject-verb` | 2 | Accords | Accord sujet / verbe aux temps déjà étudiés ; sujet groupe nominal dont le nom noyau commande l'accord (« Les pneus de la voiture crissent ») ; jamais de sujet inversé | Phrase à trou, saisie |
| `cm1-agree-noun-group` | 2 | Accords | Chaîne d'accords dans le groupe nominal (déterminant, nom, adjectif), donneur d'accord = nom noyau | Phrase à compléter, « clique sur… » |
| `cm1-gram-determiners` | 2 | Grammaire | Articles définis et indéfinis, déterminants possessifs et démonstratifs | « Clique sur… », tri |
| `cm1-homophones-ces-ses` | 2 | Homophones | ces / ses, ce / se (lien avec les déterminants démonstratifs et possessifs) | Phrase à trou avec choix |
| `cm1-conj-imparfait` | 2 | Conjugaison | Imparfait des mêmes verbes ; radical et terminaisons | Conjuguer (saisie) |
| `cm1-words-accents` | 3 | Orthographe lexicale | Accents é, è, ê et accent circonflexe (choix d'Educator) | Dictée de mots, mots à trous |
| `cm1-gram-pronouns` | 3 | Grammaire | Pronoms personnels sujets et compléments (le, la, les, lui, leur) | « Clique sur… », tri |
| `cm1-gram-coord-adverbs` | 3 | Grammaire | Conjonctions de coordination ; adverbes les plus fréquents, invariables (toujours, souvent, beaucoup, longtemps…) | « Clique sur… », dictée de mots |
| `cm1-homophones-ou` | 3 | Homophones | ou / où | Phrase à trou avec choix |
| `cm1-conj-futur` | 3 | Conjugaison | Futur des mêmes verbes | Conjuguer (saisie) |
| `cm1-agree-pp-etre` | 4 | Accords | Participe passé employé avec être, accordé avec le sujet (« elles sont parties ») | Phrase à trou, saisie |
| `cm1-gram-complements` | 4 | Grammaire | Complément d'objet ou complément circonstanciel ; groupes circonstanciels déplaçables et supprimables (sans les distinguer) | « Clique sur… », QCM |
| `cm1-homophones-leur` | 4 | Homophones | leur / leurs (pronom invariable ou déterminant) | Phrase à trou avec choix |
| `cm1-conj-passe-compose` | 4 | Conjugaison | Composition auxiliaire + participe passé ; passé composé d'être et d'avoir (j'ai été, j'ai eu) | « Clique sur… », conjuguer (saisie) |
| `cm1-gram-cod-coi` | 5 | Grammaire | COD et COI dans des phrases prototypiques, sans ambiguïté | « Clique sur… », QCM |
| `cm1-conj-mixed` | 5 | Conjugaison | Révision mélangée : présent, imparfait, futur, passé composé (temps reconnu ou demandé) | Conjuguer (saisie), QCM |

Les identifiants suivent la forme `cm1-…` des mathématiques ; à aligner sur la convention des compétences
d'orthographe existantes si elle diffère.

## Comportement attendu

- Dictée de mots, mots à trous et « choisis la bonne écriture » (lot 2) avec les compétences CM1 par période.
- Réutilise les types d'exercices créés au lot 4 (phrase à trou, phrase à compléter, « clique sur… ») ; ajoute les banques de phrases du niveau.
- Conjuguer : le pronom et l'infinitif sont affichés, l'enfant écrit la forme conjuguée ; si le lot 4 n'a pas créé ce type, il est créé ici (validation stricte des accents, comme les mots).
- Les mots et les phrases sont lus en direct par Piper (ADR 0004) ; un homophone a toujours sa phrase.

## Prérequis

Le lot 4 (CE2 : phrase à trou avec choix d'homophone, accords, grammaire, conjugaison).

## Critères d'acceptation

- [ ] Une compétence par ligne du tableau, validée par son schéma JSON, avec sa `source` (programme de français du cycle 3, ou « choix d'Educator » pour la période et les homophones).
- [ ] Au moins 30 mots par compétence d'orthographe lexicale, écoutés avec `scripts/listen-words.sh --skill` (lecture en direct, pas de fichier audio pré-généré).
- [ ] Au moins 20 phrases par compétence d'homophones, d'accords, de grammaire et de conjugaison, relues par Georges.
- [ ] Aucune notion de la liste « Pas au CM1 » dans les exercices (relecture et test sur les verbes et les temps attendus).
- [ ] Validation stricte des accents, casse tolérée (règles du lot 2).

## Sources

- [Programme de français du cycle 3 (arrêté du 10 avril 2025, BO n°16 du 17 avril 2025)](https://www.education.gouv.fr/sites/default/files/programme-de-fran-ais-pour-le-cycle-3-439824.pdf)
