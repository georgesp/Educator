---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wm4
rev: 2026-10-10T19:39:27Z
url: https://github.com/georgesp/Educator/issues/37
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001504 — Avatar en calques et accessoires SVG
status: In Review
size: L
estimate:
startDate:
endDate:
parent: "#15"
order: 614
---

## User story

En tant qu'enfant, je veux habiller mon personnage avec des accessoires que je débloque afin qu'il me ressemble.

## Contenu

- Rendu de l'avatar par composition de calques SVG : personnage (lot 1) + calques d'accessoires positionnés sur les points d'ancrage (`headTop`, `eyes`, `neck`).
- Accessoires générés en SVG **dans le même style** que les personnages : chapeaux (casquette, chapeau de magicien, couronne…), lunettes (soleil, rondes…), écharpes, capes, petits compagnons.
- Déclinaisons de couleurs automatiques par accessoire.
- Conditions de déblocage dans `content/rewards/accessories.json` (niveau atteint ou badge).

## Modèle

- `Profile.CharacterId` devient `Profile.Avatar` (JSON : personnage + accessoires portés + couleurs) ; migration des profils existants.
- Table `UnlockedAccessory` (`ProfileId`, `AccessoryId`, `UnlockedAt`).

## Critères d'acceptation

- [x] Au moins 20 accessoires, compatibles avec **tous** les personnages (planche de contrôle visuelle).
- [x] Rendu identique en 48 px et 256 px.
- [x] Migration testée.

## Réalisation

- 22 accessoires SVG dans `content/accessories/` (schéma `accessories.schema.json`), dans le style des personnages (contour 5 px, couleurs pleines) : tête (casquette, chapeau de magicien, couronne, casque de pilote, bonnet à pompon, chapeau de cow-boy, nœud, chapeau de fête), yeux (lunettes rondes, de soleil, étoiles, masque de héros, lunettes de pilote), cou (écharpe, nœud papillon, médaille, cape, collier de fleurs), compagnons (chaton, chiot, poussin, petit robot).
- Couleurs : 13 couleurs communes (`colors` du fichier) ; chaque accessoire propose les siennes, la première par défaut, appliquées par les variables `--acc` et `--acc-light`.
- Déblocage : par niveau (3 accessoires dès le niveau 1, puis jusqu'au niveau 10) ou par badge (médaille : premier tour de piste ; cape : 7 jours d'entraînement ; couronne : 10 séries sans faute).
- Composition dans le cœur Rust (`educator-core::rewards::accessories::compose_avatar`) : calques derrière (cape), personnage, puis cou, yeux, tête, compagnon, chacun posé par son point `anchor` sur le point d'ancrage du personnage (`headTop`, `eyes`, `neck` de `characters.json`, désormais lus par `CharacterCatalog::anchors`). Zone de dessin fixe `-12 -24 280 280` pour tous les avatars : un chapeau ou un compagnon n'est jamais coupé et le personnage ne change pas de taille. Commande `get_avatar_svg` ; composant `AvatarImage` (front).
- Rendu identique en 48 et 256 px : un seul SVG vectoriel. Tests : `crates/educator-content/tests/accessories.rs` vérifie que chaque accessoire tient dans le cadre pour **chaque** personnage et que l'avatar composé est un SVG valide.
- Planche de contrôle : `content/accessories/planche.html`, générée par le code Rust (`EDUCATOR_WRITE_PLANCHE=1 cargo test -p educator-content --test accessories` ; le test échoue si elle n'est plus à jour).
- Modèle : `Profile.accessories` (accessoires portés, avec leur couleur). Migration `0004_motivation` : colonne `Profile.Avatar` (JSON `{"character": …, "accessories": [{"id", "color"}]}`), remplie pour les profils existants avec leur personnage et aucun accessoire ; `CharacterId` reste renseignée avec le personnage (compatibilité). Table `UnlockedAccessory` (`ProfileId`, `AccessoryId`, `UnlockedAt`). Testée : `profiles_of_lot_2_become_avatars_without_loss`, `the_accessories_of_an_avatar_are_stored`.
