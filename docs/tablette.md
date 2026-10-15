# Educator sur iPad — préparer le Mac et l'iPad

Procédure pas à pas pour construire Educator et l'installer sur l'iPad de test. Décisions et raisons :
[ADR 0006](adr/0006-application-tablette.md).

| | |
|---|---|
| Mac | MacBook Pro M1 Pro (Apple Silicon), macOS Tahoe 26.6 ou plus récent |
| iPad de test | iPad Air 13 pouces (M2), iPadOS 27, paysage seul |
| Compte Apple | Apple ID gratuit (« Personal Team ») : installation valable 7 jours |

> Les étapes 1 à 5 se font une fois. Les étapes 6 et suivantes ne fonctionnent qu'après la fiche SUB023802
> (création du projet iOS) ; les commandes exactes y seront confirmées.

## 1. Xcode

1. Mettre macOS à jour si besoin : Xcode 27 demande **macOS Tahoe 26.6** ou plus récent.
2. Installer **Xcode** depuis l'App Store (l'application complète, ≈ 10 Go ; les Command Line Tools seuls ne
   suffisent pas).
3. Ouvrir Xcode une première fois, accepter la licence, laisser installer les composants.
4. Xcode › Réglages › **Components** : installer la plateforme **iOS 27** (et son simulateur).
5. Vérifier dans le Terminal :

   ```bash
   xcode-select -p        # doit afficher /Applications/Xcode.app/Contents/Developer
   xcodebuild -version    # doit afficher Xcode 27.x
   ```

   Si `xcode-select -p` affiche `/Library/Developer/CommandLineTools` :
   `sudo xcode-select -s /Applications/Xcode.app/Contents/Developer`.

## 2. Homebrew et CocoaPods

Prérequis iOS de Tauri 2.

```bash
brew --version || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install cocoapods
pod --version          # doit afficher une version (1.x)
```

## 3. Cibles Rust pour l'iPad

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
rustup target list --installed   # doit lister aarch64-apple-ios et aarch64-apple-ios-sim
```

`aarch64-apple-ios` : l'iPad ; `aarch64-apple-ios-sim` : le simulateur sur un Mac Apple Silicon.

## 4. Compte Apple dans Xcode

1. Xcode › Réglages › **Accounts** › « + » › Apple ID : se connecter avec son Apple ID.
2. Une équipe **« Georges Portier (Personal Team) »** apparaît : c'est elle qui signe l'application.

Limites du compte gratuit : l'application installée expire au bout de **7 jours** (elle ne s'ouvre plus ; il suffit
de la réinstaller depuis le Mac, les profils et résultats des enfants restent sur l'iPad) ; pas de TestFlight.

## 5. Préparer l'iPad

1. Brancher l'iPad au Mac en USB-C, déverrouiller l'iPad, répondre **« Se fier »** à cet ordinateur.
2. Dans Xcode › Window › **Devices and Simulators**, l'iPad apparaît (la première préparation peut prendre
   quelques minutes).
3. Sur l'iPad : Réglages › Confidentialité et sécurité › **Mode développeur** › activer, puis redémarrer et
   confirmer. (L'option n'apparaît qu'après le branchement à Xcode.)
4. Garder le Mac et l'iPad sur le **même réseau Wi-Fi** : en développement, l'iPad charge le front depuis le Mac.

## 6. Lancer dans le simulateur (après SUB023802)

```bash
cd web
npm install
npm run tauri ios dev "iPad Air 13-inch (M2)"
```

Attendu : le simulateur de l'iPad s'ouvre en paysage et affiche l'écran « Qui joue ? ». La barre d'état indique
que la voix est indisponible tant que les fiches voix ne sont pas faites.

## 7. Lancer sur l'iPad (après SUB023802)

```bash
cd web
npm run tauri ios dev -- --open
```

Dans Xcode, choisir l'iPad comme destination et l'équipe « Personal Team » dans *Signing & Capabilities* (une
seule fois), puis **Run**. Au premier lancement, l'iPad refuse d'ouvrir une application d'un développeur non
approuvé :

- Réglages › Général › **VPN et gestion de l'appareil** › profil de développeur (ton Apple ID) › **Faire confiance**.

Attendu : Educator s'ouvre sur l'iPad, en paysage, sur l'écran « Qui joue ? ».

## 8. Réinstaller chaque semaine (compte gratuit)

Au bout de 7 jours, Educator ne s'ouvre plus sur l'iPad : rebrancher l'iPad et relancer l'étape 7. Les données
(profils, résultats) sont conservées.

## Dépannage

| Symptôme | Piste |
|---|---|
| `error: target not installed` à la compilation | Étape 3 |
| « No signing certificate » / « No profiles for … » | Étape 4, puis choisir l'équipe dans *Signing & Capabilities* |
| L'iPad n'apparaît pas dans Xcode | Câble, « Se fier », iPad déverrouillé ; étape 5 |
| L'application s'ouvre sur un écran blanc en développement | Mac et iPad sur le même Wi-Fi ; pare-feu du Mac (ports 1421 et 1422) |
| « Untrusted Developer » | Étape 7, « Faire confiance » |
| L'application ne s'ouvre plus après une semaine | Étape 8 |
