---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhA4-ws
url: https://github.com/georgesp/Educator/issues/276
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: US0276 — Lot tablette 3 — Synchronisation des profils et des statistiques (Wi-Fi local)
status: Todo
size: XL
estimate:
startDate:
endDate:
parent:
order: 5000
---

Source : demande de Georges du 2026-10-14 — partager les statistiques entre l'ordinateur et la tablette, en gardant les données **hébergées en local** (pas de cloud). Solution retenue : **synchronisation sur le réseau local (Wi-Fi), appairage par QR code**.

- **Lot** : tablette 3 — Synchronisation ordinateur ↔ tablette
- **Prérequis** : lot tablette 1 (application tablette qui tourne, données et cycle de vie sur mobile) ; lot 3 pour l'espace parent (barrière « adulte », statistiques) ; lot 2 (séances et réponses enregistrées).
- **Indépendant du** lot tablette 2 (stylet) : les deux peuvent avancer dans n'importe quel ordre.

## Contexte

Chaque appareil garde sa propre base SQLite (`educator.db`). Aujourd'hui, un enfant qui travaille sur la tablette et sur l'ordinateur a deux historiques séparés : les statistiques de l'espace parent sont incomplètes et les points ne s'additionnent pas. Le lot tablette 1 a laissé la synchronisation « hors périmètre » : elle est traitée ici.

Ce qui facilite le travail : les identifiants de `Profile`, `Session` et `Answer` sont des identifiants texte (UUID) créés localement, et les réponses ne sont jamais modifiées après coup. Ce qui le complique : les profils, les points et la progression des leçons *sont* modifiés des deux côtés, les horloges des appareils peuvent différer, un profil supprimé d'un côté doit l'être de l'autre, et `SkillMastery` est une valeur calculée (20 dernières réponses par compétence).

## Décisions de principe

- **Aucun serveur distant, aucun compte, aucun cloud.** Les données ne quittent jamais le réseau local, et seulement entre deux appareils appairés par un parent.
- **L'ordinateur est le serveur, la tablette est le client.** La tablette lance la synchronisation (bouton ou ouverture de l'application) ; iOS et Android ne permettent pas de faire tourner un serveur de façon fiable en arrière-plan.
- **Fusion par union d'événements** : les séances et les réponses sont ajoutées sans jamais être modifiées, dédoublonnées par leur identifiant. Les statistiques (`SkillMastery`, points) sont **recalculées** à partir de l'historique, pas copiées. Les rares données modifiables (profil, progression de leçon) suivent une règle de résolution écrite et testée.
- **Toute la logique de fusion vit dans `educator-core`** (code pur, testable) ; le SQL dans `educator-data` ; le réseau dans une crate dédiée ; `web/src-tauri` et le front restent de fins adaptateurs.
- **Données d'enfants (RGPD)** : échanges chiffrés (TLS), appairage explicite, aucun identifiant superflu, aucune donnée d'enfant dans les logs.

## Hors périmètre

- Synchronisation via Internet ou un service tiers (iCloud, Syncthing, P2P hors réseau local).
- Synchronisation tablette ↔ tablette ou entre plus de deux ordinateurs (le modèle ne doit pas l'interdire, mais on ne le teste pas ici).
- Synchronisation du contenu pédagogique (leçons, banque de mots) : il est embarqué dans chaque application, les deux appareils doivent simplement avoir une version compatible.
- Publication sur les magasins d'applications.

## Critères d'acceptation

- [ ] Toutes les sous-issues sont terminées.
- [ ] Un parent appaire la tablette avec l'ordinateur en moins d'une minute, sans saisir d'adresse réseau.
- [ ] Après synchronisation, les deux appareils affichent les mêmes profils, les mêmes points et les mêmes statistiques dans l'espace parent.
- [ ] Une synchronisation répétée (ou interrompue puis reprise) ne crée aucun doublon et ne perd aucune donnée.
- [ ] Un travail fait hors ligne sur les deux appareils à la fois est fusionné sans perte.
- [ ] Aucune donnée ne circule hors du réseau local ; aucun appareil non appairé ne peut lire ou écrire.
- [ ] Aucune règle métier dans le front ni dans `web/src-tauri` ; test d'architecture à jour avec la nouvelle crate.
- [ ] Le build bureau (macOS, Windows), la CI existante et le build mobile du lot tablette 1 restent verts.
