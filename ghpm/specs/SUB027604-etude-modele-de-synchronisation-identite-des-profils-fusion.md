---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhA4-_w
url: https://github.com/georgesp/Educator/issues/280
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: "SUB027604 — Étude — modèle de synchronisation (identité des profils, fusion, suppressions, horloges)"
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#276"
order: 5010
---

## User story

En tant que Georges, je veux que les règles de fusion soient écrites et validées **avant** de coder, parce qu'une erreur de fusion sur des données d'enfants est difficile à rattraper.

## Questions à trancher

- **Identité d'un enfant** : deux profils créés séparément sur chaque appareil, avec le même prénom, sont-ils le même enfant ? Proposition : jamais de fusion automatique entre identifiants différents ; au premier appairage, l'espace parent propose « Est-ce le même enfant ? » (fusionner / garder les deux, le second renommé car `NormalizedName` est unique).
- **Que synchronise-t-on** : `Profile`, `Session`, `Answer`, `LessonProgress` ; `SkillMastery` recalculé ; réglages par profil (chronos `PlacementSeconds`, `ExamSeconds`) ; réglages de l'application (non synchronisés par défaut, à confirmer).
- **Détecter les changements** malgré des horloges qui diffèrent : horloge logique hybride, compteur de version par ligne, ou journal de changements numéroté. Proposition : un numéro de séquence local par ligne modifiée + curseur par appareil appairé, et une horloge hybride pour départager deux modifications du même champ.
- **Résolution des conflits** sur `Profile` : dernier écrivain gagne **champ par champ** (nom, classe, personnage, chronos) ; `LastUsedAt` = maximum ; progression de leçon = la plus avancée ou la plus récente (à trancher).
- **Points** : vérifier dans le code si `Profile.Points` est toujours la somme des `PointsEarned` des séances. Si oui, recalcul après fusion ; sinon, synchroniser des écarts (journal de points) plutôt que la valeur.
- **Suppression d'un profil** : pierres tombales (tombstones) propagées ; la suppression l'emporte-t-elle sur une modification ultérieure de l'autre appareil ? (proposition : oui, avec confirmation parent à la création de la suppression, déjà existante).
- **Séances non terminées** (application fermée en pleine série) : abandonnées ou ignorées à la fusion ?
- **Compétences inconnues** : une réponse enregistrée avec un `SkillId` absent de la version de l'autre appareil est conservée mais exclue des statistiques jusqu'à la mise à jour.
- **Évaluation de niveau** (« disponible une seule fois », SUB001418) : faite sur un appareil, elle doit compter comme faite sur l'autre.
- **Versions** : version du protocole et de la base (`user_version`) ; que fait-on quand elles diffèrent (refus bienveillant et message clair).
- **Serveur actif quand ?** Proposition : uniquement pendant que l'écran « Synchronisation » est ouvert sur l'ordinateur ; alternative : tant que l'application est ouverte, si le parent l'a activé dans les options.
- **Plateformes** : réseau local sur iOS (permission « Réseau local », services Bonjour déclarés) et Android (verrou multicast, configuration de sécurité réseau pour un certificat auto-signé).

## Livrable

- ADR 0006 « Synchronisation locale des données » : décisions ci-dessus, schéma de données, protocole en grandes lignes, modèle de menaces (qui peut se connecter, que se passe-t-il si un QR code est photographié), risques.
- Tableau « donnée → règle de fusion → test » repris par les fiches suivantes.

## Critères d'acceptation

- [ ] ADR 0006 rédigé et validé par Georges.
- [ ] Chaque table de la base a une ligne dans le tableau de fusion (synchronisée ou non, et pourquoi).
- [ ] Les fiches suivantes du lot sont ajustées si l'étude change le périmètre.
