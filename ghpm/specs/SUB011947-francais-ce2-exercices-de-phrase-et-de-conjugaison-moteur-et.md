---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBXomM
url: https://github.com/georgesp/Educator/issues/304
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB011947 — Français CE2 — Exercices de phrase et de conjugaison (moteur et écrans)
status: Backlog
size: XL
estimate:
startDate:
endDate:
parent: "#119"
order: 1249
---

Source : revue du lot 4 du 2026-10-16. Le lot 2 sait dicter un mot, faire compléter un mot à trous et faire choisir la
bonne écriture d'un mot. Le programme du CE2 demande en plus de travailler **dans la phrase** : accords, relation
sujet-verbe, conjugaison, classes de mots, types de phrases. Cette fiche crée une fois les types d'exercices, les
banques de contenu et les règles communes ; les fiches de thèmes du français CE2 (et des lots 5 et 6) les réutilisent.

## User story

En tant qu'enfant de CE2, je veux des exercices où je complète, je choisis, je clique ou je trie des mots dans une
phrase, et où je conjugue des verbes, afin de m'entraîner à la grammaire et à la conjugaison de ma classe.

## Nouveaux formats d'exercices (`skill.schema.json`, `educator-core`)

| Format | L'enfant… | Réponse | Exemple |
|---|---|---|---|
| `sentenceChoice` | choisit le mot qui complète la phrase (2 à 4 propositions) | choix | Le chat … sur le mur. (et / est) |
| `sentenceWrite` | écrit le mot ou la terminaison qui manque, au clavier | texte | Les petit… chats dorment. → s |
| `conjugate` | écrit le verbe conjugué au temps et à la personne demandés | texte | Nous (chanter, futur) … → chanterons |
| `clickWord` | clique sur le ou les mots demandés dans la phrase | ensemble | Clique sur le verbe : « Les enfants jouent au ballon. » |
| `sortWords` | range des étiquettes de mots dans 2 ou 3 colonnes | ensemble par colonne | nom / verbe / adjectif ; singulier / pluriel |
| `chooseSentence` | choisit la phrase juste ou transformée parmi 3 | choix | La phrase à la forme négative ? |
| `sentenceDictation` | écrit une phrase courte (4 à 8 mots) entendue au clic | texte | « Les petites filles jouent dans la cour. » |

- Validation dans `educator-core` (règles du lot 2) : **aucune tolérance sur les accents**, casse tolérée, espaces en
  trop ignorés, « oe » refusé pour « œ ». Une dictée de phrase est corrigée **mot par mot** : les mots faux sont soulignés,
  la bonne écriture est montrée, la réponse compte juste seulement si tous les mots le sont.
- Après une erreur, l'**astuce** de la notion (« Remplace par *était* : si ça marche, c'est *est* ») s'affiche ; elle est
  lisible au clic, jamais lue toute seule (décision du 2026-10-15). Le mot qui commande l'accord (le nom, le sujet) est
  mis en évidence.
- Distracteurs générés depuis les banques (homophone voisin, terminaison d'une autre personne ou d'un autre temps, accord
  oublié), jamais une forme inventée qui n'existe pas en français.

## Banques de contenu (`content/`)

- **Phrases** `content/sentences/*.json` (schéma `sentences.schema.json`) : la phrase découpée en mots, avec pour chaque
  mot sa classe (déterminant, nom commun, nom propre, adjectif, verbe, pronom personnel sujet, adverbe, autre), le groupe
  sujet et le verbe conjugué, le genre et le nombre des mots variables ; les trous possibles (mot, terminaison) et leurs
  distracteurs ; la notion travaillée ; la classe ; la source. Phrases originales, courtes, du quotidien de l'enfant, prénoms
  variés et non stéréotypés, **relues par Georges**.
- **Verbes** `content/verbs/verbs.json` (schéma `verbs.schema.json`) : être, avoir, les verbes du 1er groupe de la liste
  de fréquence (avec les cas de -ger et -cer : nous mangeons, nous commençons) et les verbes irréguliers du programme
  (faire, aller, dire, venir, pouvoir, voir, vouloir, prendre). Les formes des verbes en -er sont **calculées** par
  `educator-core::conjugation` (radical + terminaison) ; celles des verbes irréguliers sont écrites dans le fichier et
  vérifiées par test. Temps : présent, imparfait, futur et passé composé de l'indicatif.
- **Mots** : `content/words/` complété des mots du CE2 (`grades: ["CE2"]`) ; catégorie `adverbe` déjà prévue ; nouveaux
  champs facultatifs `feminine` et `plural` pour les noms et adjectifs (accords) et `family` (familles de mots).

## Code couleur, voix et autres écrans

- **Thèmes d'orthographe** (SUB001514) : trois nouvelles valeurs de `theme` — `homophones` (« Les homophones », symbole
  `a/à`), `sentence` (« La phrase et les mots », symbole `GN`), `verbs` (« Les verbes », symbole `-er`) ; huit couleurs en
  orthographe (et six en mathématiques avec les mesures) : chaque thème de `web/src/styles.css` reçoit les couleurs
  manquantes, contraste AA vérifié par `web/src/themes.test.ts`.
- **Voix** (ADR 0004) : une phrase est lue en entier au clic ; une phrase à trou est lue avec « blanc » à la place du mot
  qui manque (sauf dans la dictée) ; la consigne de conjugaison dit l'infinitif et le temps (« Conjugue *chanter* au
  futur avec *nous* »). Préparation en arrière-plan comme les mots (`SpeechPreparer`).
- **Sans voix** : les formats qui ne demandent pas d'écouter restent jouables ; la dictée de phrase est grisée comme la
  dictée de mots (CLAUDE.md, synthèse vocale).
- **Examen et évaluation de niveau** (SUB001414, SUB001418) : les nouveaux formats y entrent ; ceux qui demandent d'écouter
  en sont exclus quand la voix manque.
- **iPad** : saisie sans correction automatique ni majuscule automatique (SUB023810) ; clavier à l'écran avec les lettres
  accentuées et « œ » ; cibles de 44 points au moins pour cliquer et trier.

## Critères d'acceptation

- [ ] Les sept formats dans `skill.schema.json`, générés et validés par `educator-core`, avec tests unitaires (accents, casse, espaces, réponses multiples, dictée mot par mot).
- [ ] `sentences.schema.json` et `verbs.schema.json` ; une phrase mal découpée, un trou sans bonne réponse ou une forme verbale manquante fait échouer `content_validation.rs`.
- [ ] Conjugaison calculée des verbes en -er testée sur toute la liste (4 temps, 6 personnes, -ger et -cer) ; formes des huit verbes irréguliers et d'être et avoir vérifiées par un test de référence.
- [ ] Trois nouveaux thèmes d'orthographe dans le schéma, `Skill::category` et le filtre des écrans Exercices et Leçons ; couleurs ajoutées à tous les thèmes visuels (test de contraste).
- [ ] Astuce affichée après une erreur, lisible au clic ; aucune lecture automatique.
- [ ] Écrans utilisables à la souris, au clavier et au doigt (iPad, ADR 0006).
- [ ] Textes de l'enfant dans `fr.ts`, types TypeScript régénérés, tests Rust, Vitest et Playwright.

## Sources

- [Programme de français du cycle 2 (arrêté du 22 octobre 2024, BO n°41 du 31 octobre 2024, en vigueur à la rentrée 2025)](https://www.education.gouv.fr/sites/default/files/document/Annexe%203%20%E2%80%93%20Programme%20de%20fran%C3%A7ais%20du%20cycle%202-403818.pdf)
- [La grammaire du français du CP à la 6e (eduscol)](https://eduscol.education.gouv.fr/sites/default/files/document/guide-la-grammaire-du-francais-du-cp-la-6e-97251.pdf) et [terminologie grammaticale](https://eduscol.education.gouv.fr/sites/default/files/document/guide-la-grammaire-du-francais-terminologie-grammaticale-67998.pdf)
- [Liste de fréquence lexicale (eduscol)](https://eduscol.education.gouv.fr/6873/liste-de-frequence-lexicale)

## Dépendances

- Lot 2 (formats de mots, validation orthographique SUB001405), lot 2bis (voix en direct, ADR 0004), SUB001514 (code couleur).
- À faire avant les fiches d'exercices de français du CE2 qui ne sont pas seulement des dictées de mots.
