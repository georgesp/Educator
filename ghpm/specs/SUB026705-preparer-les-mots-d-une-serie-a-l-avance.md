---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhAzCTo
rev: 2026-10-14T19:30:51Z
url: https://github.com/georgesp/Educator/issues/272
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB026705 — Préparer les mots d'une série à l'avance
status: In Review
size: M
estimate:
startDate:
endDate:
parent: "#267"
order: 530
---

## User story

En tant qu'enfant, je veux entendre le mot dès qu'il s'affiche, comme avec un fichier enregistré.

## Comportement attendu

- Au début d'une série qui fait lire des mots (dictée, mots à trous, « Choisis la bonne écriture », examen et quiz de leçon d'orthographe), le Rust prépare en arrière-plan l'audio de chaque mot et de sa phrase d'exemple dans le cache de la voix (`SpeechCache`), dès l'affichage de la série (la voix ne part qu'au clic depuis le 2026-10-15 : ADR 0004).
- Texte lu pour un mot (décision du 2026-10-14) : un nom avec son article (« un stylo. »), les autres mots suivis d'une virgule (« beau, », décision du 2026-10-15) ; voir SUB026706.
- Ordre : le mot 1, puis sa phrase, puis les suivants. Une demande de l'enfant passe toujours avant la préparation.
- `speak_series_word` sert le cache quand le mot est prêt, sinon attend la synthèse en cours de ce mot (pas une deuxième synthèse).
- Le mot n'est jamais envoyé au front avant la réponse (inchangé).
- Une série abandonnée arrête sa préparation.

## Critères d'acceptation

- [ ] Délai entre le clic sur « Écouter le mot » et le début du son < 200 ms sur le Mac de Georges, mesuré sur une dictée de 10 mots (moyenne et pire cas notés).
- [x] Tests : ordre de préparation, priorité de la demande de l'enfant, pas de double synthèse, arrêt à l'abandon.

## Implémentation (2026-10-15)

- `educator_core::speech::SpeechPreparer` : un fil de préparation par application ; `prepare(série, lectures)` met en
  file chaque mot puis sa phrase ; `read(série, lecture)` sert la lecture préparée depuis la mémoire, attend celle en
  cours, ou synthétise tout de suite celle qui attend en file (la préparation se met en pause pendant ce temps) ;
  `release(série)` arrête la préparation (abandon, fin de série, défi terminé). Une nouvelle série prend la place des
  autres dans la file (une série quittée sans abandon ne retarde pas la suivante) ; au plus 2 séries gardées en
  mémoire. Les 12 premiers mots sont préparés au départ (`WORDS_AHEAD`), les suivants à chaque mot entendu
  (`readings_ahead`, choix fait dans `educator-core`). Une panique du moteur ne bloque pas les demandes suivantes.
- Branché dans `services.rs` : `start_series`, `start_exam`, `start_placement`, `start_timed`, `start_lesson_quiz`
  préparent les mots ; `speak_series_word` lit via le préparateur ; `answer_series` (dernière réponse),
  `abandon_series` et `finish_series` libèrent la série.
- Tests : `crates/educator-core/tests/speech_preparation.rs` (ordre, priorité, pas de double synthèse, arrêt à
  l'abandon, échec repris à la demande, sans voix) ; `services::tests::the_words_of_a_dictation_are_prepared_and_read_with_their_article`.
- Mesure : `cargo test -p educator-content --test speech piper_prepared_dictation_is_heard_at_once -- --nocapture`
  (dictée de 10 mots, vrai Piper) affiche le délai moyen et le pire cas. Environnement de Claude (Linux x64) :
  **0 ms en moyenne et au pire**.
- Reste (Georges) : la même mesure sur le Mac (Rosetta), à noter ici.
