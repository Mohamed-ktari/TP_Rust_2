#  Logo Compiler & Interpreter in Rust

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-TP2--Completed-success)]()

Ce projet est un **compilateur et interpréteur** pour le langage **Logo**, développé en Rust. Il permet de transformer des commandes textuelles de mouvement (style "tortue") en visualisations graphiques au format **SVG**.

Le projet a été construit de manière itérative, illustrant la montée en puissance d'un compilateur, de l'analyse manuelle à l'utilisation d'une grammaire formelle robuste.

---

##  Fonctionnalités

Le développement est structuré en quatre étapes clés :

* **Partie 1 : Fondations** – Implémentation manuelle d'un *Lexer* et d'un *Parser* pour les commandes de base : `forward`, `backward`, `left`, `right`.
* **Partie 2 : Grammaire Formelle** – Utilisation de la bibliothèque **Santiago** pour définir une grammaire structurée et générer un Arbre de Syntaxe Abstraite (AST).
* **Partie 3 : Génération SVG** – Intégration de `svg_fmt` pour traduire l'AST en fichiers images vectorielles `.svg`.
* **Partie 4 : Structures Avancées** – Support complet des boucles `repeat`, des blocs de code `[...]` et de la gestion du stylo (`penup`, `pendown`).

---

##  Installation & Utilisation

### Prérequis
* [Rust](https://www.rust-lang.org/tools/install) (cargo)
* Dépendances externes : `santiago` et `svg_fmt`.

### Lancement
Pour compiler et exécuter le projet globalement :
```bash
cargo run
