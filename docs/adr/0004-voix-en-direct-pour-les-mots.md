# ADR 0004 — Voix en direct pour les mots d'orthographe

- **Statut** : acceptée
- **Date** : 2026-10-15
- **Fiches** : US0267 (lot 2bis — voix en direct pour les mots d'orthographe), SUB026701 à SUB026708
- **Remplace** : la pré-génération de l'audio des mots de SUB001404 (lot 2). Le moteur reste celui de l'ADR 0002.

## Contexte

Au lot 2 (SUB001404), chaque mot de la banque et sa phrase d'exemple étaient pré-générés en WAV avec Piper
(`scripts/generate-word-audio`), dans `content/audio/words/`, embarqués au build ; la voix en direct ne servait que
de secours. Mesures du 2026-10-14 (voix `fr_FR-siwis-medium`) :

| | Fichiers WAV | Voix en direct |
|---|---|---|
| Espace disque CP–CE1 (959 mots + phrases) | 129 Mo | 0 Mo de plus (moteur et voix déjà embarqués pour les consignes) |
| Espace disque projeté CP → CM2 (≈ 2 500 mots, ≈ 1 000 phrases) | ≈ 420 Mo, ≈ 600 Mo avec une version à 0,5 | 0 Mo de plus, cache plafonné |
| Latence | ≈ 10 à 30 ms | mot ≈ 70 à 100 ms, phrase ≈ 130 à 260 ms (Linux x86) ; ≈ 0 ms une fois préparé à l'avance |
| Ralentir à 0,5 | 2e jeu de fichiers, ou déformation du son | natif (`--length_scale`), sans perte de qualité |
| Simplicité | script de 20 min à relancer, deux chemins (fichiers + secours en direct) | un seul chemin, déjà utilisé pour les consignes |

Contraintes vérifiées : Piper fixe la vitesse **pour tout le processus** (`--length_scale`, un champ de vitesse dans
la demande JSON est ignoré) ; changer de vitesse redémarre Piper (≈ 0,4 à 0,5 s) ; un processus chaud occupe
≈ 105 à 110 Mo de mémoire. Piper tire du hasard à chaque lecture (`noise_scale` 0,667, `noise_w` 0,8) : deux lectures
du même mot diffèrent légèrement.

## Décision

Les mots d'orthographe sont lus **par la voix en direct**, comme les consignes. Aucun fichier audio de mot n'est
embarqué : la taille de l'application ne dépend plus du nombre de mots.

### Ce qui est lu (décisions d'écoute de Georges, 2026-10-14)

- **Même vitesse pour le mot et sa phrase : 0,85** (`speech::speed::WORDS`). Les consignes restent à 0,9
  (`speech::speed::INSTRUCTIONS`), l'écoute lente « Plus lentement » est à 0,5 (`speech::speed::SLOW`).
- **Un nom est lu avec son article** : « un stylo. », « une école. ». Seul en début de lecture, un nom perd souvent sa
  première consonne (« stylo » entendu « sylo ») ; l'article règle le problème et donne le genre. Champ `article`
  (« un » ou « une », jamais « le » ni « l' » à cause de l'élision) obligatoire pour la catégorie `nom` ; l'enfant
  n'écrit que le nom, l'article est affiché devant la case (« un [______] »).
- **Les autres mots sont lus suivis d'une virgule** : « beau, », « très, » (décision du 2026-10-15, qui remplace
  le point choisi le 2026-10-14). Suivi d'un point, un mot court est souvent mal rendu par la voix : « beau. » est
  entendu « beu » à chaque lecture (voyelle mesurée : 2e formant ≈ 1 600 Hz, celui d'un [ø], contre ≈ 950 Hz avec la
  virgule, l'exclamation ou les deux-points ; la phonétique d'espeak-ng, `bˈo`, est pourtant juste). Georges a
  choisi la virgule à l'écoute.
- **Les homophones ont toujours leur phrase** : un mot de `content/spelling/homophones.json` (liste tenue à la
  main), ou un mot dont la `pronunciation` est celle d'un autre mot de la banque, sans `example` est refusé au
  chargement.
- Champ facultatif `pronunciation` : le texte lu à la place du mot quand la voix le prononce mal (« fils » →
  « fisse ») ; le mot affiché et attendu ne change pas.

Le texte lu est construit par le Rust (`educator_core::spelling::word_reading`, `SpokenWord::speech`).

### Un processus Piper par vitesse (SUB026708)

`PiperSpeechSynthesizer` garde un processus `piper --json-input` **par vitesse** : passer de la consigne au mot
dicté, puis à l'écoute lente, ne redémarre jamais Piper. Les vitesses des consignes et des mots sont préchauffées au
démarrage et restent prêtes ; un autre processus (l'écoute lente) démarre à la première demande et s'arrête après
**2 minutes** sans demande. Au plus deux processus restent donc chauds en permanence (≈ 220 Mo). Les demandes à une
même vitesse sont servies une à la fois ; deux vitesses travaillent en parallèle. Un processus planté ou bloqué est
relancé à la demande suivante.

### Préparation des mots d'une série (SUB026705)

Au début d'une série qui fait lire des mots (exercices, examen, évaluation, défi, quiz de leçon), le Rust prépare en
arrière-plan (`educator_core::speech::SpeechPreparer`) chaque mot puis sa phrase, dans l'ordre, dès l'affichage de
la série : les 12 premiers mots (`WORDS_AHEAD`), puis les suivants au fur et à mesure
que l'enfant avance (un défi chrono en compte 90). Quand l'enfant demande un mot :

- préparé : servi aussitôt, depuis la mémoire ;
- en cours de synthèse : la demande attend cette synthèse (jamais une deuxième) ;
- encore en file : il est synthétisé tout de suite, et la préparation attend (la demande de l'enfant passe avant).

Un enfant ne joue qu'une série à la fois : une nouvelle série prend la place des autres dans la file. Une série
abandonnée, terminée ou trop ancienne (au-delà de 2 séries) libère sa préparation. Le mot n'est jamais envoyé au
front avant la réponse : le front demande « le mot de l'exercice *n* de la série ». Un échec ou une panique du moteur
ne bloque jamais les demandes suivantes.

### Rien n'est lu tout seul (décision du 2026-10-15)

Dans toute l'application, la voix ne part **qu'au clic** sur un bouton : « Écouter le mot », « Plus lentement »,
« Le mot dans une phrase », et les haut-parleurs de la consigne, de la correction, du bilan, des diapositives de
leçon et de la mascotte. Plus de lecture automatique de la consigne ni du mot à l'ouverture d'un exercice, ni du mot
suivant, ni de la correction, ni du bilan, ni de la phrase de la mascotte, ni des écrans (menu au survol compris).
Constaté par Georges : lue automatiquement à l'ouverture de l'exercice, la consigne perdait son premier mot
(« Écoute »), alors que le fichier de Piper commence déjà par ≈ 200 ms de silence. Le chrono d'une question
(examen ou évaluation minutés) démarre dès l'affichage de la question.

### Cache disque plafonné (SUB026701)

`SpeechCache` (dossier `speech-cache` des données de l'application) est plafonné à **30 Mo** ; au-delà, les fichiers
les moins récemment **lus** sont supprimés (jusqu'à 90 % du plafond). La clé reste voix + vitesse + texte : changer de
voix ou de prononciation ne sert jamais un ancien fichier. Le cache reste jetable : tout problème de lecture ou
d'écriture se rabat sur la synthèse.

### Sans voix (SUB026704)

Sans voix — Piper absent, ou qui ne démarre pas au préchauffage (Rosetta manquante, fichiers abîmés) —, les
exercices où le mot est seulement entendu (dictée, « Choisis la bonne écriture ») sont grisés avec un message
bienveillant ; les mots à trous restent jouables. Un examen, une évaluation ou un défi d'orthographe sans voix ne
compose que des questions jouables sans écoute. Une panne de Piper survenue plus tard, après un préchauffage réussi,
n'est pas détectée : les lectures répondent alors `503` et l'écran reste muet. Les quiz de leçon d'orthographe ne
sont pas grisés (hors périmètre du lot).

### Écoute de contrôle

`scripts/listen-words.sh` / `.ps1` (exemple `listen_words`) écrit dans un dossier temporaire, jamais embarqué, les
WAV d'un échantillon (`--sample 50`) ou d'une compétence (`--skill ce1-son-o-au-eau`), avec la liste à écouter ;
`--slow` ajoute la lecture lente ; `--again stylo` refait la lecture d'un mot et remplace celle du cache de
l'application. Le cache de l'application recompte son dossier toutes les 100 écritures, ce qui prend en compte les
fichiers ajoutés par le script.

## Mesures

Environnement de développement de Claude (Linux x64, 2 cœurs, voix siwis-medium, Piper préchauffé) :

- dictée de 10 mots préparée pendant la consigne : mot entendu **0 ms** après la demande en moyenne et au pire
  (test `piper_prepared_dictation_is_heard_at_once`, qui affiche ses mesures) ;
- synthèse d'un calcul après préchauffage : 100 à 420 ms selon la charge ;
- mémoire d'un processus Piper : ≈ 105 Mo (RSS).

**À mesurer sur le Mac de Georges (Rosetta)** et à reporter ici : `cargo test -p educator-content --test speech
piper_ -- --nocapture` (délais de la dictée), mémoire totale des processus `piper` dans le Moniteur d'activité.

## Conséquences

- Plus d'étape de build à ne pas oublier ni de dossier `content/audio/` ; un seul chemin pour la voix.
- La première écoute lente d'un mot prend le temps d'une synthèse (≈ 0,1 à 0,5 s, plus le démarrage du processus
  lent s'il était arrêté) ; les suivantes viennent du cache.
- Sur tablette, Piper ne peut pas être lancé comme processus annexe : le moteur intégré est étudié dans
  SUB026703 (ADR 0005) et réalisé dans le lot tablette.
- Les tests qui utilisent le vrai moteur ne s'exécutent que si Piper est installé (la CI ne le télécharge pas).
