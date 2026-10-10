---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzg_5RoA
url: https://github.com/georgesp/Educator/issues/2
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB000101 — Personnages SVG des profils
status: Todo
size: M
estimate:
startDate:
endDate:
parent: "#1"
order: 20
---

## User story

En tant qu'enfant, je veux un personnage sympathique comme image de profil afin de reconnaître mon profil d'un coup d'œil.

## Contenu

Jeu initial de personnages, **tous dans le même style artistique** :

- Animaux : lion, ours, lapin, renard, cochon.
- Enfants stylisés : au moins 3 garçons et 3 filles (coiffures, teints et couleurs de vêtements variés).

## Charte graphique commune

- Format carré, `viewBox="0 0 256 256"`, personnage cadré tête + buste.
- Formes rondes et simples, contour de même épaisseur pour tous, palette commune limitée (définie dans `content/characters/palette.json`).
- Aucun texte, aucune référence à un personnage existant ou à une marque.
- Lisible de 48 px (liste des profils) à 256 px (écran d'accueil).

## Préparation du lot 3 (accessoires)

- Chaque SVG est découpé en groupes nommés : `<g id="body">`, `<g id="head">`, `<g id="face">`, `<g id="hair">` (ou oreilles / crinière pour les animaux), `<g id="clothes">`.
- Points d'ancrage communs déclarés dans un fichier `content/characters/characters.json` : `headTop`, `eyes`, `neck` (coordonnées dans la viewBox).

## Critères d'acceptation

- [ ] 11 personnages minimum livrés dans `content/characters/*.svg`.
- [ ] `characters.json` liste chaque personnage : `id`, `name` (français), `file`, `kind` (`animal` / `child`), points d'ancrage.
- [ ] Planche de contrôle (une page HTML ou image) montrant tous les personnages côte à côte, validée visuellement pour l'homogénéité du style.
- [ ] Test automatisé : chaque SVG est valide, possède les groupes attendus et chaque personnage de `characters.json` a son fichier.

## Notes techniques

- SVG optimisés (pas de métadonnées d'éditeur), couleurs en variables de palette pour permettre des déclinaisons.
- Fichiers embarqués avec le contenu et servis par le host (`GET /api/ui/characters`, `GET /api/ui/characters/{id}.svg`).
