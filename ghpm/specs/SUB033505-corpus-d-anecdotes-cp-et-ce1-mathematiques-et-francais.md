---
# Managed by GHPM: do not edit these keys
id: PVTI_lAHOACppGM4BmYInzhBdqRU
url: https://github.com/georgesp/Educator/issues/340
type: Issue
repository: georgesp/Educator
labels: []
# Editable
title: SUB033505 — Corpus d'anecdotes CP et CE1 (mathématiques et français)
status: Backlog
size: L
estimate:
startDate:
endDate:
parent: "#335"
order: 655
---

## User story

En tant que parent, je veux que les anecdotes racontées à mon enfant soient vraies, adaptées à son âge et liées à ce qu'il apprend.

## Contenu

- Au moins **une anecdote par leçon CP–CE1** existante (26 leçons de mathématiques, 22 d'orthographe), et quelques-unes de plus pour les notions les plus travaillées : environ 60 anecdotes.
- Rédaction par Claude, relecture par Georges avant passage en `Done`.
- Idées de thèmes : histoire des chiffres et du zéro, compter sur ses doigts et en base 10, le signe = et le signe +, l'invention du mètre et du kilogramme, l'histoire de l'euro et des pièces, les fractions chez les Égyptiens ; origine des lettres muettes, du « œ », de la cédille, des accents, des mots venus d'autres langues, la rectification de 1990.

## Sources

Règle : **au moins deux sources indépendantes** par anecdote, dont au moins une institutionnelle ou académique ; Wikipédia ne compte que par les sources qu'elle cite.

- Histoire des mathématiques : *Images des mathématiques* (CNRS), https://images.math.cnrs.fr/ ; APMEP, https://www.apmep.fr/ ; réseau des IREM, https://www.univ-irem.fr/.
- Langue et orthographe : CNRTL (étymologie), https://www.cnrtl.fr/etymologie/ ; Académie française, « Dire, ne pas dire », https://www.academie-francaise.fr/dire-ne-pas-dire.
- Sciences et mesures : Universcience, https://www.universcience.fr/ ; Musée des Arts et Métiers, https://www.arts-et-metiers.net/.
- Contenus pour enfants, pour les idées et le ton, à recouper : Lumni, https://www.lumni.fr/ ; Vikidia, https://fr.vikidia.org/ ; Wikimini, https://fr.wikimini.org/.

## Vérification

- Script de contrôle (comme pour les leçons) : nombre d'anecdotes par leçon, longueur, règle des sources, URL qui répondent.
- Relecture des faits par un agent séparé, qui n'a pas rédigé les anecdotes, en repartant des sources.
- Les anecdotes douteuses ou invérifiables sont retirées plutôt que nuancées.

## Critères d'acceptation

- [ ] Chaque leçon CP–CE1 a au moins une anecdote liée.
- [ ] Toutes les anecdotes passent le chargeur et le script de contrôle.
- [ ] Relecture de Georges faite.
