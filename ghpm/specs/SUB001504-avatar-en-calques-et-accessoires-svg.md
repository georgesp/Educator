---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5Wm4
url: https://github.com/georgesp/Educator/issues/37
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB001504 — Avatar en calques et accessoires SVG
status: Todo
size: L
estimate:
startDate:
endDate:
parent: "#15"
order: 440
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

- [ ] Au moins 20 accessoires, compatibles avec **tous** les personnages (planche de contrôle visuelle).
- [ ] Rendu identique en 48 px et 256 px.
- [ ] Migration testée.
