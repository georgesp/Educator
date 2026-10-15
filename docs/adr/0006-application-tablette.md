# ADR 0006 — Application tablette (iPad)

- **Statut** : proposée — à valider par Georges (critère de la fiche SUB023801)
- **Date** : 2026-10-15
- **Fiche** : SUB023801 — Étude : plateformes cibles, versions minimales et prérequis (lot tablette 1, US0238)
- **Liée à** : [ADR 0003](0003-backend-rust.md) (backend Rust), [ADR 0005](0005-voix-sur-tablette.md) (voix sur tablette)

## Contexte

Le lot tablette 1 doit faire tourner Educator sur une tablette avec le même parcours qu'à l'ordinateur, la voix de
la tablette, hors ligne, sans stylet. La fiche 01 tranche la plateforme, l'appareil de test, l'orientation, les
versions minimales et les prérequis, avant d'engager le développement.

L'ADR 0003 a rendu l'application autonome (tout dans le processus Tauri, en Rust) : la coque `educator-shell` est
déjà compilée en `staticlib` / `cdylib`, les plugins propres au bureau sont exclus hors bureau et Piper n'est pas
compilé pour iOS / Android.

## Décisions

### Plateforme : iPad seulement

- **iPadOS** en premier et seul pour ce lot. Android est mis de côté (fiches Android : voir « Conséquences sur les
  fiches »).
- **iPad uniquement**, pas d'iPhone (famille d'appareils « iPad » dans le projet Xcode) : les écrans de moins de
  7 pouces sont hors périmètre du lot.

### Appareil de test

| | |
|---|---|
| Modèle | iPad Air 13 pouces (M2, 2024) |
| Système | iPadOS 27 (dernière version, sortie le 14 septembre 2026) |
| Écran | 2 732 × 2 048 pixels, soit **1 366 × 1 024 points** en paysage |
| Stylet compatible (lot tablette 2) | Apple Pencil Pro, Apple Pencil (USB-C) |

L'écran en paysage dépasse la taille minimale de la fenêtre du bureau (1 024 × 700) : les écrans actuels devraient
s'afficher sans refonte ; la fiche 08 vérifie chaque écran et ajoute les zones sûres.

### Orientation : paysage seul

- Paysage à gauche et à droite ; le portrait n'est pas proposé.
- Raison : l'interface est pensée pour un écran plus large que haut (1 024 × 700 minimum) ; en portrait, la largeur
  est juste à la limite sur 13 pouces (1 024 points, pour 1 366 de haut) et en dessous sur 11 pouces (820 points) :
  tous les écrans seraient à recomposer.
- À vérifier à la fiche 08 : depuis iPadOS 26, une application peut être affichée dans une fenêtre redimensionnable ;
  le comportement d'Educator dans une petite fenêtre (taille minimale, message) reste à définir.

### Versions minimales

- **iPadOS 17** minimum déclaré (proposition : couvre les iPad depuis 2018 ; `AVSpeechSynthesizer.write` existe depuis
  iOS 13). Seule la version testée (iPadOS 27) est garantie.
- Le stylet (lot tablette 2) pourra demander une version plus récente.

### Outils sur le Mac

| Outil | Version / commande |
|---|---|
| Mac | MacBook Pro M1 Pro (Apple Silicon) |
| macOS | Tahoe 26.6 ou plus récent (exigé par Xcode 27) |
| Xcode | 27 complet (App Store), pas seulement les Command Line Tools ; plateforme iOS 27 installée |
| Homebrew + CocoaPods | `brew install cocoapods` (prérequis iOS de Tauri 2) |
| Cibles Rust | `rustup target add aarch64-apple-ios aarch64-apple-ios-sim` (`x86_64-apple-ios` inutile sur Apple Silicon) |
| Tauri CLI | celle du projet (`@tauri-apps/cli` 2.12), via `npm run tauri` |

### Compte Apple : compte gratuit pour commencer

- Pas de compte Apple Developer payant pour l'instant : installation avec un **Apple ID gratuit** (« Personal Team »
  dans Xcode).
- Conséquences : le profil d'installation expire au bout de **7 jours** (l'application ne s'ouvre plus, il faut la
  réinstaller depuis le Mac ; les données de l'application restent en place) ; pas de TestFlight ; nombre
  d'applications installées et d'identifiants limité. Educator n'a besoin d'aucune capacité réservée aux comptes
  payants (iCloud, notifications…).
- Le **mode développeur** doit être activé sur l'iPad (Réglages › Confidentialité et sécurité › Mode développeur).
- Passage au compte payant (99 € par an) à décider quand l'iPad servira tous les jours aux enfants ou pour
  TestFlight ; publication sur l'App Store hors périmètre (lot packaging).

### Même dépôt, même application

- La version iPad vit dans ce dépôt : `tauri ios init` crée le projet Xcode dans `web/src-tauri/gen/apple`, à côté
  de la configuration du bureau. Les crates, le front, `content/`, les contrats et `types.ts` sont partagés.
- Une seule coque Tauri (`educator-shell`), les différences passent par `cfg(desktop)` / `cfg(mobile)` ; aucune
  règle métier ajoutée dans la coque ni dans le front.
- `web/src-tauri/gen/` est aujourd'hui entièrement ignoré par git : la fiche 02 décide ce qui est commité dans
  `gen/apple` (projet Xcode, `Info.plist`, icônes) et ce qui reste ignoré (sorties de build).
- Plugin de voix système (fiche 05) : proposition d'un dossier `plugins/tauri-plugin-system-voice/` membre du
  workspace (Rust + Swift), distinct de `crates/` réservé au domaine.
- Estimation : environ 95 % du code actuel (≈ 46 000 lignes sur 48 500, hors tests) tourne sur l'iPad tel quel ;
  le code propre à l'iPad est estimé entre 1 500 et 3 000 lignes (Swift, pont Rust, styles, cycle de vie).

### Saisie : les claviers de l'application

- Les exercices ont déjà leurs claviers à l'écran : `NumberPad` (nombres) et `LetterKeyboard` (AZERTY, accents, œ).
- Proposition pour la fiche 10 : sur l'iPad, le clavier du système ne s'ouvre pas sur les champs de réponse
  (`inputMode="none"` ou champ en lecture seule) ; l'enfant n'utilise que les claviers de l'application. Plus de
  correction automatique ni de clavier qui masque le champ de réponse.
- Les champs de texte libre (nom d'un profil, espace parent) gardent le clavier du système, sans correction
  automatique ni majuscule automatique.

### Voix

- Voix du système (`AVSpeechSynthesizer`), comme décidé dans US0238 et l'ADR 0005 (option A). La fiche 04 choisit le
  contrat (WAV renvoyé au front comme aujourd'hui, ou lecture directe).

### Taille de l'application

- Contenu pédagogique (`content/`) : ≈ 2,5 Mo, sans fichier audio (les mots sont lus en direct). Pas de voix ni de
  moteur à embarquer. Taille de l'application à mesurer à la fiche 14.

## Risques

| Risque | Conséquence | Parade |
|---|---|---|
| Profil gratuit de 7 jours | L'application ne s'ouvre plus chez les enfants | Réinstaller chaque semaine pendant le développement ; compte payant avant un usage quotidien |
| Lecture audio dans WKWebView après un appel asynchrone | `Audio.play()` refusé si le geste de l'enfant n'est plus « en cours » quand le WAV arrive | À vérifier dès la fiche 04 ; sinon lecture directe par le système (contrat « lire ») |
| Fenêtres redimensionnables d'iPadOS 26+ | Écran trop petit, mise en page cassée | Fiche 08 : taille minimale ou message « Agrandis la fenêtre » |
| Voix française absente ou de qualité basique | Dictée peu claire | Fiche 05 : état de la voix et message au parent (voix « améliorée » à télécharger dans les réglages) |
| Application suspendue ou tuée par iPadOS | Série perdue | Fiche 11 : reprise ou abandon propre |
| Mises à jour de Xcode et d'iPadOS chaque année | Build cassé | Versions notées dans `docs/tablette.md`, CI iOS (fiche 12) |

## Conséquences sur les fiches du lot

- SUB023807 (voix Android) : mise de côté (hors lot, ou futur lot « tablette Android »).
- SUB023804, SUB023812, SUB023813 : partie iOS seulement.
- Lot tablette 2 : SUB023905 (écriture Android, ML Kit) mise de côté de la même façon.
- Procédure de préparation du Mac et de l'iPad : [`docs/tablette.md`](../tablette.md).
