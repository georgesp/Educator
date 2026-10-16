# Recette de la synchronisation ordinateur ↔ iPad (lot tablette 2)

- **Fiche** : SUB027609 — Recette de la synchronisation (US0276)
- **Décisions** : [ADR 0007](adr/0007-synchronisation-locale.md)
- **Appareils** : MacBook Pro M1 Pro (macOS), iPad Air 13 pouces M2 (iPadOS 27), même Wi-Fi
- **Joué par** : Georges — **le** : _à compléter_

Les tests automatiques couvrent déjà la fusion (idempotence, ordre des fusions, renommages, suppressions, même enfant,
horloges décalées, lots de plusieurs milliers de réponses, reprise après coupure), le protocole (QR code rejoué,
jeton retiré, autre certificat, version différente) et l'écran de l'ordinateur. Cette recette vérifie le vrai réseau,
les permissions de l'iPad, le pare-feu du Mac et le confort d'usage.

## Préparer

```bash
# Sur le Mac, à la racine du dépôt
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace                 # doit finir par « test result: ok » pour chaque crate
cd web && npm install                  # ajoute @tauri-apps/plugin-barcode-scanner
npm run build && npm test && npm run test:e2e
npm run tauri dev                      # application de l'ordinateur
npm run tauri ios dev -- --open        # application de l'iPad, via Xcode (docs/tablette.md)
```

- Au premier « Associer une tablette », macOS peut demander d'**accepter les connexions entrantes** pour Educator :
  répondre « Autoriser ». En cas de refus : Réglages Système › Réseau › Pare-feu › Options, autoriser Educator.
- Au premier scan, l'iPad demande l'accès à la **caméra**, puis au **réseau local** : répondre « Autoriser ». En cas de
  refus : Réglages › Confidentialité et sécurité › Caméra / Réseau local › Educator.
- Pour regarder la base pendant la recette : `~/Library/Application Support/Educator/educator.db` (Mac). Les journaux
  sont dans `logs/` à côté : ils ne doivent contenir **aucun prénom** (seulement des identifiants tronqués et des
  nombres).

## Scénarios

Résultat : ✅ conforme · ⚠️ conforme avec remarque · ❌ anomalie (créer une fiche dans `ghpm/specs/`).

| # | Scénario | Attendu | Résultat | Remarque / fiche |
|---|---|---|---|---|
| 1 | Premier appairage : espace parent du Mac › « Associer une tablette », espace parent de l'iPad › « Associer à l'ordinateur », viser le QR code | Associée en moins d'une minute, sans saisir d'adresse ; « Tablette 1 » apparaît sur le Mac | | |
| 2 | Plan B : refuser la caméra, puis « Pas de caméra ? Entrer le code » avec l'adresse et le code affichés | Message bienveillant, appairage réussi | | |
| 3 | QR code photographié, puis réutilisé après l'appairage (ou après 5 minutes) | Refusé : « Ce code n'est plus valable » | | |
| 4 | Profils existants des deux côtés, prénoms différents | Après « Synchroniser maintenant », les deux appareils ont tous les profils | | |
| 5 | Même prénom des deux côtés : répondre « Oui, c'est le même enfant » | Un seul profil ; séances, points et badges réunis sur les deux appareils | | |
| 6 | Même prénom des deux côtés : répondre « Non, ce sont deux enfants » | Deux profils ; celui de l'iPad renommé « Prénom 2 » | | |
| 7 | Travail hors ligne le même jour sur le Mac et l'iPad (séries, leçon commencée, badge), puis synchronisation | Mêmes points, mêmes statistiques dans l'espace parent des deux côtés ; un seul bonus d'objectif du jour | | |
| 8 | Synchroniser deux fois de suite | « Tout est déjà à jour », aucun doublon | | |
| 9 | Wi-Fi coupé en plein échange (gros historique), puis rétabli | Message « s'est arrêtée en route » ou « introuvable » ; la synchronisation suivante complète, sans doublon | | |
| 10 | Application de l'iPad mise en arrière-plan, puis tuée, en plein échange | Reprise à la synchronisation suivante, sans perte ni doublon | | |
| 11 | Mac en veille, ou Educator fermé sur le Mac | iPad : « L'ordinateur est introuvable… » ; l'enfant joue normalement | | |
| 12 | Option « Recevoir les synchronisations » activée, écran fermé sur le Mac ; ouvrir Educator sur l'iPad | Synchronisation silencieuse à « Qui joue ? » | | |
| 13 | Profil renommé des deux côtés | Le dernier nom gagne, le même sur les deux appareils | | |
| 14 | Profil supprimé sur le Mac, utilisé sur l'iPad avant de synchroniser | Le profil disparaît des deux côtés (la suppression l'emporte) | | |
| 15 | Heure de l'iPad décalée de 10 minutes (Réglages › Général › Date et heure), renommage après une synchronisation | Le renommage de l'iPad n'est pas perdu | | |
| 16 | Lancer une série sur l'iPad juste après son ouverture (synchronisation silencieuse en cours ou à venir) | Aucune synchronisation ne démarre pendant une série ; l'enfant n'est jamais interrompu | | |
| 17 | Version différente d'un seul côté (si possible) | Message « pas la même version d'Educator » | | |
| 18 | « Retirer cette tablette » sur le Mac, puis « Synchroniser maintenant » sur l'iPad | « L'ordinateur ne reconnaît plus cette tablette » | | |
| 19 | Volume : plusieurs mois d'historique (au moins 3 000 réponses) ; noter la durée de la première synchronisation | Tout arrive ; durée raisonnable (à noter) | | |
| 20 | Box redémarrée (adresse du Mac changée) | iPad : « introuvable » ; un nouveau scan du QR code rétablit la synchronisation sans rien perdre | | |

## Mesures

| Mesure | Valeur |
|---|---|
| Durée de la première synchronisation (scénario 19) | |
| Nombre de réponses échangées | |
| Durée d'une synchronisation sans changement | |

## Anomalies

| Fiche | Résumé | Gravité |
|---|---|---|
| | | |

## Conclusion

_À compléter : la synchronisation peut-elle servir tous les jours aux enfants ?_
