---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAzCY4
rev: 2026-10-14T19:31:03Z
url: https://github.com/georgesp/Educator/issues/273
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB026706 — Article, prononciation et homophones dans la banque de mots"
status: In Review
size: S
estimate:
startDate:
endDate:
parent: "#267"
order: 570
---

## User story

En tant que concepteur du contenu, je veux corriger un mot mal prononcé par la voix sans enregistrer de fichier.

## Comportement attendu

- Champ facultatif `pronunciation` dans le format des mots (`content/words/*.json`, `words.schema.json`) : le texte donné à Piper à la place du mot (« fils » → « fisse », « os » au pluriel → « ô »). Le mot affiché et attendu ne change pas.
- Le champ `audio` du format (nom d'un fichier pré-généré) est retiré.
- **Article des noms** (décision du 2026-10-14) : champ `article` (« un » ou « une », jamais « le » ni « l' » pour éviter l'élision), obligatoire pour la catégorie `nom`. Le nom est lu avec son article (« un stylo. ») ; l'enfant n'écrit que le nom, l'article est affiché devant la case (« un [______] »). Les 642 noms de la banque sont complétés par Claude, puis relus par Georges.
- **Autres mots** : lus suivis d'une virgule (« beau, », décision du 2026-10-15 : avec un point, « beau. » est entendu « beu »), texte construit par le Rust à partir du mot.
- **Homophones** : la phrase d'exemple (`example`) devient obligatoire pour un mot qui a un homophone dans la banque ou dans une liste d'homophones courants (et / est, a / à, ou / où, on / ont, prêt / près, ver / verre / vert…) ; le chargement refuse un homophone sans phrase.
- **Refaire une lecture** : Piper tirant du hasard à chaque lecture, le script d'écoute permet de refaire la lecture d'un mot (`--again stylo`), qui remplace celle du cache.
- Script d'écoute de contrôle `scripts/listen-words.sh` / `.ps1` : génère dans un dossier temporaire (jamais embarqué) les WAV d'un échantillon de mots (`--sample 50`) ou d'une compétence (`--skill ce1-son-o-au-eau`), avec la liste à écouter.
- Reprend l'écoute de contrôle des 50 mots de SUB001404.

## Critères d'acceptation

- [x] Un mot avec `pronunciation` est lu avec ce texte ; le mot attendu reste le mot (test).
- [x] Schéma documenté ; un `pronunciation` vide est refusé.
- [x] `article` obligatoire pour un nom (test : un nom sans article est refusé) ; l'article est affiché devant la case de la dictée.
- [x] Un homophone sans phrase est refusé (test).
- [ ] Écoute de contrôle de 50 mots faite par Georges ; les corrections trouvées sont ajoutées à la banque.

## Implémentation (2026-10-15)

- Format des mots : champs `article` (« un » / « une », obligatoire pour un `nom`, interdit sinon) et
  `pronunciation` ; champ `audio` retiré. Schéma `words.schema.json` mis à jour.
- **Les 642 noms ont reçu leur article** (déduit de la phrase d'exemple quand elle le donnait, choisi à la main pour
  les 88 autres) : **à relire par Georges**. Choix à vérifier en priorité : noms épicènes lus au masculin
  (« un artiste », « un élève », « un enfant », « un parent », « un astronaute », « un membre ») ; mois et jours
  (« un avril », « un mai », « un samedi ») ; noms abstraits ou de matière (« un temps », « un air », « une eau »,
  « une soif ») ; « une madame » (correct mais familier : mot à garder ?). Relu le 2026-10-15 : « auto » corrigé en
  « une ».
- Texte lu construit en Rust : `educator_core::spelling::word_reading` (« un stylo. », « beau, », prononciation à la
  place du mot) ; `SpokenWord::speech(example, slow)`.
- Homophones : nouvelle liste `content/spelling/homophones.json` (64 groupes tenus à la main, schéma
  `homophones.schema.json`) ; un mot d'un groupe, ou un mot dont la `pronunciation` est celle d'un autre mot de la
  banque, sans `example` est refusé au chargement. Tous les mots de la banque ont déjà leur phrase.
- Les mots à écrire des quiz de leçon prennent leur mot de la banque (article, prononciation, phrase) au chargement
  du contenu (`attach_words`).
- Article affiché devant la case (dictée, mots à trous, quiz de leçon) : contrat `WordPromptResponse.article`.
- Script `scripts/listen-words.sh` / `.ps1` (exemple `listen_words`) : `--sample 50`, `--skill <id>`, `--slow`,
  `--again <mot>` (remplace la lecture dans le cache de la voix de l'application), dossier temporaire jamais embarqué,
  liste `ecoute.txt`.
- Tests : `a_word_is_read_with_its_article_or_its_pronunciation`, `articles_and_pronunciations_are_checked`,
  `a_homophone_without_its_sentence_is_refused` (`content_validation.rs`),
  `a_noun_is_read_with_its_article_any_other_word_with_a_period` (`educator-core`).
- Reste (Georges) : relecture des articles, écoute de contrôle de 50 mots (`scripts/listen-words.sh --sample 50`),
  corrections ajoutées à la banque (`pronunciation`).
