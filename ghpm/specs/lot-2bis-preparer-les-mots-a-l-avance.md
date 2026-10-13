---
type: Issue
title: Préparer les mots d'une série à l'avance
status: Todo
size: M
parent: specs/lot-2bis-voix-en-direct.md
order: 503
---

## User story

En tant qu'enfant, je veux entendre le mot dès qu'il s'affiche, comme avec un fichier enregistré.

## Comportement attendu

- Au début d'une série qui fait lire des mots (dictée, mots à trous, « Choisis la bonne écriture », examen et quiz de leçon d'orthographe), le Rust prépare en arrière-plan l'audio de chaque mot et de sa phrase d'exemple dans le cache de la voix (`SpeechCache`), pendant que la consigne est lue.
- Ordre : le mot 1, puis sa phrase, puis les suivants. Une demande de l'enfant passe toujours avant la préparation.
- `speak_series_word` sert le cache quand le mot est prêt, sinon attend la synthèse en cours de ce mot (pas une deuxième synthèse).
- Le mot n'est jamais envoyé au front avant la réponse (inchangé).
- Une série abandonnée arrête sa préparation.

## Critères d'acceptation

- [ ] Délai entre l'affichage du mot et le début du son < 200 ms sur le Mac de Georges, mesuré sur une dictée de 10 mots (moyenne et pire cas notés).
- [ ] Tests : ordre de préparation, priorité de la demande de l'enfant, pas de double synthèse, arrêt à l'abandon.
