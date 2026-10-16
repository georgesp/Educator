# ADR 0007 — Synchronisation locale des données (ordinateur ↔ tablette, Wi-Fi)

- **Statut** : acceptée (validée par Georges le 2026-10-15, sans `LegacyPoints` : voir section 6)
- **Date** : 2026-10-15
- **Fiche** : SUB027604 — Étude : modèle de synchronisation (lot tablette 2 — synchronisation, US0276)
- **Liée à** : [ADR 0003](0003-backend-rust.md) (backend Rust), [ADR 0006](0006-application-tablette.md) (application iPad)

> La fiche parle d'« ADR 0006 » et de la migration « 0004_sync » : ces numéros sont déjà pris (ADR 0006 = application
> tablette, migration 0004 = motivation). Cette décision est donc l'**ADR 0007** et la migration sera **0005_sync**.

## Contexte

Chaque appareil a sa propre base SQLite. Un enfant qui travaille sur l'iPad et sur le Mac a deux historiques : les
statistiques de l'espace parent sont incomplètes et les points ne s'additionnent pas. Georges veut partager les
données **sans cloud** : synchronisation sur le Wi-Fi de la maison, appairage par QR code.

Constats dans le code (2026-10-15) :

- `Profile`, `Session`, `Answer`, `ReviewItem` ont des identifiants Guid v7 créés localement : pas de collision entre
  appareils.
- Une séance n'est modifiée qu'une fois, à sa fermeture (`end_session`, ou `abandon_open_sessions` au démarrage) ;
  ses réponses ne sont jamais modifiées.
- **Les points ne sont pas toujours la somme des séances** : `Profile.Points` = Σ `Session.PointsEarned` +
  Σ `DailyGoalDay.BonusPoints` pour tout ce qui a été fait depuis le lot 2, mais les points gagnés avant le lot 2
  (version .NET, sans table `Session`) ne correspondent à aucune séance.
- `SkillMastery` est recalculée à partir des 20 dernières réponses (`sessions::update_masteries`).
- L'évaluation de niveau « faite une seule fois » (`placement_done`) se lit dans les séances : synchroniser les
  séances suffit.
- La sauvegarde (`educator-data::backup`) copie toutes les tables, dont `AppSetting`.
- L'application tablette est **iPad seulement** (ADR 0006) : tout ce qui concerne Android est hors périmètre.

## Décisions

### 1. Rôles et exposition

- **L'ordinateur est le serveur, l'iPad est le client.** L'iPad lance toujours la synchronisation.
- Le serveur n'écoute que dans deux cas : **l'écran « Synchronisation » de l'espace parent est ouvert** sur
  l'ordinateur (nécessaire pour l'appairage), ou **l'option « Recevoir les synchronisations » est activée** dans cet
  écran (désactivée par défaut, enregistrée sur l'ordinateur) ; dans ce second cas, tant que l'application est ouverte.
  Arrêt propre à la fermeture de l'écran (si l'option est coupée) et de l'application.
- Il écoute sur toutes les interfaces mais **refuse toute connexion qui ne vient pas d'une adresse privée**
  (10/8, 172.16/12, 192.168/16, 169.254/16, fc00::/7, fe80::/10) : rien ne sort du réseau local.
- Port : le dernier port utilisé s'il est libre, sinon un port choisi par le système ; il est donné par le QR code et
  par l'annonce Bonjour.

### 2. Identité des appareils et des enfants

- Chaque base reçoit un **identifiant d'appareil** (Guid v7) au premier démarrage après la migration. Il est rangé
  dans une table propre à la synchronisation (`SyncState`), **jamais dans `AppSetting`** et jamais dans la sauvegarde :
  restaurer la sauvegarde d'un appareil sur un autre ne copie pas son identité ni ses appairages.
- **L'identité d'un enfant est l'identifiant de son profil.** Deux profils créés séparément ne sont jamais fusionnés
  automatiquement.
- Avant chaque synchronisation, l'iPad compare la liste des profils (identifiant + prénom) avec celle de l'ordinateur
  (sans les profils de l'ordinateur que l'iPad a supprimés entre-temps : leur suppression va partir).
  Pour chaque prénom présent des deux côtés sous deux identifiants différents, l'espace parent de l'iPad demande
  **« Est-ce le même enfant ? »** :
  - **Oui, c'est le même enfant** : le profil de l'iPad est **rattaché** à celui de l'ordinateur (ses séances,
    réponses, badges, accessoires, jours d'objectif, révisions, réglages et progressions passent sous l'identifiant
    de l'ordinateur ; son nom, sa classe et son avatar sont ceux de l'ordinateur). Une pierre tombale « rattaché à »
    est propagée.
  - **Non, ce sont deux enfants** : le profil de l'iPad est renommé « Prénom 2 » (« Prénom 3 »… si besoin, 20 lettres
    au plus), comme un renommage ordinaire ; le parent peut le renommer ensuite.
  - Tant que la question n'a pas de réponse, la synchronisation ne démarre pas (la tentative silencieuse à
    l'ouverture de l'application est simplement reportée, l'espace parent signale qu'une question attend).
- Filet de sécurité (prénom pris entre la comparaison et l'envoi) : la fusion ne plante jamais sur l'index unique
  `NormalizedName` ; le profil dont l'identifiant est le plus grand reçoit le suffixe « 2 ». La règle est la même sur
  les deux appareils, donc le résultat aussi.

### 3. Détecter les changements malgré des horloges différentes

- **Horloge hybride (HLC)** par appareil : `(millisecondes, compteur, appareil)`, écrite en texte de largeur fixe
  (`0001792095649001-000003-<appareil>`) pour se comparer en SQL. Elle avance à chaque écriture locale et se recale sur
  la plus grande valeur reçue : un iPad en retard de dix minutes ne perd pas ses modifications, et deux modifications
  se départagent toujours de la même façon (l'appareil sert d'arbitre en cas d'égalité).
- **Table de suivi `SyncRow`** `(Entity, Key, Field, Hlc, Seq, Deleted, Target)` : une ligne par donnée suivie (par
  champ pour le profil ; `Deleted` = pierre tombale, `Target` = profil « rattaché à »). `Seq` est un numéro local qui ne
  fait que croître (`SyncCounter`, même quand des lignes sont supprimées), donné à chaque écriture locale **dans la
  même transaction** que l'écriture elle-même.
  - *Mise en œuvre* : des **déclencheurs SQLite** (migration 0005) sur chaque table suivie ; l'horloge vient d'une
    fonction SQL `educator_hlc()` enregistrée par `educator-data` sur la connexion. Les écritures existantes
    (stores, sauvegarde) n'ont donc pas changé : rien ne peut être oublié.
  - Une valeur reçue qui l'emporte reçoit aussi un nouveau `Seq` (elle pourra être transmise à un troisième appareil
    plus tard, sans le tester ici).
  - Les réponses (`Answer`) n'ont pas de ligne propre : elles voyagent avec leur séance, qui n'est suivie qu'une
    fois fermée.
- **Curseurs** côté iPad, par ordinateur appairé : `PushedSeq` (le dernier `Seq` de l'iPad accusé par l'ordinateur)
  et `PulledSeq` (le dernier `Seq` de l'ordinateur reçu et enregistré par l'iPad). Un lot n'avance le curseur
  qu'**après** sa validation : une coupure fait renvoyer le dernier lot, sans effet puisque la fusion est idempotente.
- On n'envoie pas à un appareil ce qui vient de lui (HLC dont l'appareil est le destinataire), sauf quand le curseur
  est à 0 (premier échange, ou tout à nouveau après une restauration).
- Le parent qui retire une tablette ou dissocie l'ordinateur pendant un échange garde le dernier mot : les fins
  d'échange ne font que des mises à jour ciblées (curseur, dernière synchronisation), jamais une réécriture complète
  de l'appareil appairé.
- À la migration, tout ce qui existe reçoit une ligne de suivi « à envoyer ».

### 4. Ce qui est synchronisé, et comment (tableau de fusion)

| Table | Synchronisée | Règle de fusion | Tests (fiche 08) |
|---|---|---|---|
| `Profile` — `Name`, `Grade`, avatar (`CharacterId` + `Avatar`), `PlacementSeconds`, `ExamSeconds` | Oui | Dernier écrivain gagne **champ par champ** (HLC du champ). Collision de prénom : section 2 | renommé des deux côtés ; classe changée d'un côté, avatar de l'autre ; deux profils de même nom |
| `Profile.LastUsedAt` | Oui | Maximum | |
| `Profile.CreatedAt` | Oui | Minimum | |
| `Profile.Points` | Non (recalculée) | Σ `PointsEarned` des séances fermées + Σ bonus des jours d'objectif | points identiques des deux côtés après fusion |
| `Session` | Oui, **fermées seulement** | Union par identifiant ; une séance n'est écrite que par l'appareil qui l'a créée. Une séance encore ouverte n'est pas envoyée : elle le sera une fois fermée (fin normale ou abandon au démarrage suivant) | même séance reçue deux fois ; séance non terminée |
| `Answer` | Oui, avec sa séance | Union par identifiant (et `(SessionId, Index)`) ; jamais modifiée | lot de milliers de réponses découpé |
| `SkillMastery` | Non (recalculée) | `sessions::update_masteries` pour chaque profil et compétence touchés par les réponses reçues | statistiques identiques à « tout sur un appareil » |
| `LessonProgress` | Oui | La plus récente gagne (HLC), y compris « leçon terminée » (ligne supprimée = pierre tombale) | |
| `EarnedBadge` | Oui | Union ; date = la plus ancienne | |
| `UnlockedAccessory` | Oui | Union ; date = la plus ancienne | |
| `DailyGoalDay` | Oui | Union par (profil, jour) ; si les deux appareils ont atteint l'objectif le même jour, on garde la plus ancienne et **un seul bonus** | objectif atteint le même jour des deux côtés |
| `ReviewItem` | Oui | Clé (profil, compétence, clé de l'exercice) ; la plus récente gagne (HLC), sortie de la file = pierre tombale | |
| `ProfileSetting` | Oui | Dernier écrivain gagne, par réglage | |
| `AppSetting` (thème de l'appareil, carte des parents…) | **Non** | Réglages de l'appareil, pas de l'enfant | |
| `SyncState`, `SyncRow`, `SyncCounter`, `SyncPeer` (nouvelles) | Non | Propres à chaque appareil, hors sauvegarde | |

- **Évaluation de niveau** : elle se lit dans les séances ; faite sur l'iPad, elle compte comme faite sur l'ordinateur
  dès la synchronisation.
- **Compétences inconnues** (réponse d'une version plus récente) : la réponse est conservée et `SkillMastery` est
  calculée comme pour les autres. Les totaux de l'espace parent la comptent (ils sont ainsi les mêmes sur les deux
  appareils) ; tant que l'appareil n'est pas à jour, la ligne de cet exercice s'affiche sous son identifiant, comme pour
  un exercice retiré du contenu (comportement existant, pas de cas particulier).
- **Badges et accessoires nouveaux grâce aux données fusionnées** : ils sont accordés à la prochaine séance, par le code
  existant (`RewardsService::after_session`), pas pendant la fusion.

### 5. Suppression d'un profil

- La suppression laisse une **pierre tombale** (ligne `Deleted` de `SyncRow` : identifiant du profil, HLC,
  éventuellement « rattaché à » dans `Target`). Reçue, elle supprime le profil et tout ce qui en dépend.
- **La suppression l'emporte** sur toute modification ou séance de l'autre appareil, même plus récente : la confirmation
  existe déjà quand le parent supprime un profil. Une séance reçue pour un profil supprimé est ignorée.
- Rattachement (« même enfant ») : la pierre tombale porte la cible ; un appareil qui connaîtrait encore l'ancien
  profil y déplace ses données avant de le supprimer.

### 6. Points

- **Décision de Georges (2026-10-15)** : les points gagnés avec l'ancienne version .NET, qui ne correspondent à aucune
  séance, sont abandonnés. Pas de colonne « points hérités ».
- Les points d'un profil sont **toujours** Σ `PointsEarned` des séances fermées + Σ `BonusPoints` des jours d'objectif.
  La migration 0005 recalcule `Profile.Points` ainsi pour tous les profils (un profil de la version .NET repart avec
  les seuls points de ses séances).
- Après chaque fusion, les points des profils touchés sont recalculés par une fonction de `educator-core`
  (`sync::profile_points`). Un test vérifie que `add_points` et ce recalcul donnent toujours le même total.
- Conséquence assumée : si l'objectif du jour a été atteint le même jour sur les deux appareils, un des deux bonus
  disparaît à la fusion (le profil a 30 points de moins sur un appareil), comme si tout avait été fait au même endroit.

### 7. Protocole (grandes lignes)

- **HTTPS** avec un **certificat auto-signé** (ECDSA P-256) généré au premier lancement du serveur (`rcgen`),
  certificat et clé privée rangés dans `SyncState` (base locale, dossier de données de l'application, jamais dans la
  sauvegarde). L'iPad **épingle l'empreinte SHA-256** du certificat reçue dans le QR code :
  aucune autorité de certification, une autre machine est refusée.
- HTTP/1.1 minimal écrit dans la crate `educator-sync` (une requête par connexion, `Content-Length` obligatoire, corps
  limité à 4 Mo, délais de lecture et d'écriture) au-dessus de **rustls** (fournisseur `ring`, compilable pour iOS).
  Pas de serveur web généraliste : moins de dépendances, et le code est testé de bout en bout.
- Messages JSON décrits par un schéma versionné (`crates/educator-sync/schemas/sync-messages.schema.json`, exemples
  dans `schemas/examples/`), même démarche que le contenu :
  - `POST /v1/pair` : secret d'appairage → jeton durable ;
  - `POST /v1/hello` : versions, identifiant d'appareil → profils de l'ordinateur (pour la question « même enfant ») ;
  - `POST /v1/push` : un lot de changements de l'iPad → accusé (`Seq` le plus haut appliqué) ;
  - `POST /v1/pull` : curseur → un lot de changements de l'ordinateur, `more` s'il en reste.
- Lots bornés (200 séances ou 2 000 éléments au plus). Un lot contient toujours l'état complet (avec HLC) des profils
  auxquels il fait référence : une séance n'arrive jamais avant son profil.
- Chaque lot est appliqué **dans une seule transaction** SQLite : tout ou rien.
- **Versions** : `protocol` = 1. Une version différente est refusée par un message bienveillant (« Mets à jour
  Educator sur les deux appareils »). La version de la base (`user_version`) n'est pas comparée : le format d'échange
  est indépendant du schéma.
- **Bonjour / mDNS** (`mdns-sd`, bureau) : l'ordinateur s'annonce `_educator-sync._tcp.local.` sous un nom neutre
  « Educator-XXXX » (4 caractères de l'identifiant d'appareil), aucune donnée d'enfant.
- **Retrouver l'ordinateur depuis l'iPad** (écart à la fiche, constaté à l'implémentation) : sur iPadOS, écouter le
  multicast avec une bibliothèque Rust demande l'autorisation `com.apple.developer.networking.multicast`, accordée par
  Apple sur demande et impossible avec le compte gratuit (ADR 0006). La recherche Bonjour passe donc par le framework
  natif (`NWBrowser`), en Swift : **reportée à une fiche de suivi**. En attendant, l'iPad utilise les adresses du QR code
  puis la dernière adresse qui a répondu (essayée en premier). Si l'ordinateur change d'adresse (box redémarrée), un
  nouveau scan du QR code met l'adresse à jour sans rien perdre (même ordinateur, mêmes curseurs).

### 8. Appairage

- « Associer une tablette » (espace parent de l'ordinateur) affiche un QR code :
  `educator://pair?v=1&h=<adresses>&p=<port>&f=<empreinte>&s=<secret>`. Le secret (128 bits) ne vit qu'en mémoire,
  **5 minutes**, **une seule fois**.
- Plan B sans caméra : un **code court de 8 caractères** (`ABCD-EFGH`) affiché sous le QR code avec l'adresse de
  l'ordinateur (`192.168.1.10:47615`), valable aussi 5 minutes, 5 essais au plus ; le parent saisit les deux sur l'iPad,
  qui accepte le certificat à la première connexion puis l'épingle (confiance au premier usage, plus faible que le QR
  code : à n'utiliser qu'à la maison). Plus de 5 échecs en une minute depuis une même adresse : refus pendant une
  minute (`429`).
- L'échange donne un **jeton de 256 bits** propre à l'iPad ; l'ordinateur n'en garde que l'empreinte SHA-256
  (`SyncPeer`). Chaque requête le présente ; un appareil retiré est refusé immédiatement.
- Nom de l'appareil proposé par défaut : « Tablette 1 » (« Tablette 2 »…) côté ordinateur, « Ordinateur » côté iPad,
  modifiable, sans prénom d'enfant.
- **Jeton sur l'iPad** : rangé dans la base de l'application (dossier protégé de l'application, hors sauvegarde
  Educator). Le Keychain demanderait du Swift supplémentaire ou une dépendance à tester sur l'appareil : reporté à une
  fiche de suivi.
- Lecture du QR code : plugin officiel `tauri-plugin-barcode-scanner` (iOS seulement). Permissions déclarées dans
  `Info.plist` avec un texte en français : caméra (`NSCameraUsageDescription`), réseau local
  (`NSLocalNetworkUsageDescription`, `NSBonjourServices`).

### 9. Déclenchement côté iPad

- Bouton « Synchroniser maintenant » dans l'espace parent, et tentative silencieuse à l'ouverture de l'application
  (écran « Qui joue ? ») si un ordinateur est appairé et qu'aucune question « même enfant » n'attend.
- **Jamais pendant une série, un examen, un quiz ou un défi chrono** : la synchronisation est refusée tant qu'une série
  est en cours dans le processus.
- Passage en arrière-plan : l'échange en cours s'arrête au prochain lot ; il reprend plus tard grâce aux curseurs.
- Échecs normaux (ordinateur éteint ou sur un autre réseau, réception coupée, certificat différent, version
  incompatible, appareil retiré) : message simple dans l'espace parent, rien pour l'enfant.

### 10. Organisation du code

| Où | Quoi |
|---|---|
| `educator-core::sync` | HLC, types des changements, moteur de fusion (code pur), recalcul des points, traits `SyncStore`, règles de lot |
| `educator-data::sync` | Migration 0005, suivi dans les écritures existantes (même transaction), application d'un lot en une transaction, lecture des changements après un curseur |
| `educator-sync` (nouvelle crate) | Format d'échange JSON (serde) et schéma, TLS (rustls, rcgen), HTTP minimal, serveur (bureau), client (iPad et tests), appairage, annonce Bonjour (bureau), limites de débit |
| `web/src-tauri` | `synchronisation.rs` : commandes (`get_sync_status`, `open_sync_screen` / `close_sync_screen`, `set_sync_receive`, `start_sync_pairing`, `cancel_sync_pairing`, `rename_sync_device`, `remove_sync_device`, `pair_with_computer`, `sync_now`, `answer_sync_question`, `interrupt_sync`), cycle de vie du serveur, aucune règle |
| `web/src` | Écran « Synchronisation » de l'espace parent (ordinateur et iPad), question « même enfant », textes dans `i18n/fr.ts` |

Règle de dépendance : `educator-sync` → `educator-core` seulement (ni `data`, ni `content`) ; vérifiée par
`tests/architecture.rs`.

## Modèle de menaces

| Situation | Ce qui se passe |
|---|---|
| Un autre appareil du Wi-Fi se connecte au serveur | Sans jeton valide : refus (`401`), après 5 échecs en une minute : `429`. Le serveur n'écoute pas par défaut. |
| Quelqu'un photographie le QR code | Le secret ne sert qu'une fois et expire au bout de 5 minutes ; un QR code rejoué plus tard est refusé. |
| Une machine se fait passer pour l'ordinateur | L'empreinte du certificat ne correspond pas : l'iPad refuse et l'affiche au parent. |
| Écoute du Wi-Fi | Tout est chiffré (TLS 1.3). |
| Connexion depuis Internet (redirection de port de la box) | Refusée : seules les adresses privées sont acceptées. |
| Tablette perdue | « Retirer cette tablette » sur l'ordinateur révoque son jeton immédiatement. |
| Logs | Ni prénom, ni jeton, ni secret ; identifiants tronqués (8 caractères) et nombres d'éléments seulement. |

## Risques

- **Pare-feu de macOS** : l'application de développement non signée déclenche la question « accepter les connexions
  entrantes » ; un refus rend le serveur injoignable. L'écran l'explique quand l'iPad ne trouve pas l'ordinateur.
- **Réseau local sur iPadOS** : sans la permission « Réseau local », Bonjour et les connexions échouent sans erreur
  claire ; l'écran de l'iPad le signale.
- **Profil de 7 jours** (compte Apple gratuit) : réinstaller l'application garde ses données et son appairage.
- **Wi-Fi « invité »** ou isolation des clients sur certaines box : les deux appareils ne se voient pas. Hors de notre
  portée, message dans l'écran.
- Restauration d'une sauvegarde : elle remplace les profils de l'appareil, sans envoyer de suppression. Elle change
  l'**époque** de l'appareil (`SyncState`, renvoyée par `/v1/hello`) et remet ses curseurs à 0 : à la synchronisation
  suivante, les deux appareils s'échangent tout (un curseur à 0 envoie tout, même ce que l'autre appareil a déjà ; la
  fusion ignore ce qu'elle connaît). Les profils de l'autre appareil reviennent donc.
- Un profil supprimé puis ramené par la sauvegarde (le parent veut le retrouver) : si l'appareil est appairé, il
  revient sous un **nouvel identifiant** (ses séances, badges, réglages le suivent), car sa suppression, peut-être
  déjà connue de l'autre appareil, l'emporterait toujours sur l'ancien identifiant. Il revient ainsi sur les deux
  appareils. Sans appareil appairé, rien ne change (même identifiant, comme au lot 3).

## Conséquences sur les fiches

- SUB027607 : migration **0005_sync** (et non 0004).
- SUB027606 : le schéma va dans `crates/educator-sync/schemas/` (ce n'est pas du contenu pédagogique, il n'est pas
  embarqué) ; les types échangés ne sont pas exposés au front (seuls les contrats des commandes le sont).
- SUB027601 / SUB027602 : Android hors périmètre (ADR 0006) ; jeton dans la base de l'iPad, Keychain en fiche de suivi ;
  recherche Bonjour sur l'iPad (framework natif) en fiche de suivi.
- SUB027605 (fichier chiffré, facultatif) : reporté après la recette, comme prévu par la fiche.
- SUB027609 : rapport de recette dans `docs/recette-synchronisation.md`, joué par Georges sur le Mac et l'iPad.
