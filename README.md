Logo Compiler & Interpreter in Rust
Ce projet est un compilateur et interpréteur pour le langage Logo, développé en Rust dans le cadre du TP2. 
Il permet de transformer des commandes textuelles de mouvement (style "tortue") en visualisations graphiques au format SVG.Le projet a été construit de manière itérative, passant d'une analyse manuelle à une grammaire formelle robuste capable de gérer des structures de contrôle avancées.
Fonctionnalités :
Le projet est divisé en quatre étapes clés reflétant la montée en complexité :
Partie 1 : Fondations - Implémentation manuelle d'un Lexer et d'un Parser pour les commandes de base (forward, backward, left, right).
Partie 2 : Grammaire Formelle - Utilisation de la bibliothèque Santiago pour définir une grammaire structurée et générer un Arbre de Syntaxe Abstraite (AST).
Partie 3 : Génération SVG - Intégration de svg_fmt pour traduire l'AST en un fichier image vectorielle .svg.
Partie 4 : Structures Avancées - Support des boucles repeat, des blocs de code [...], et de la gestion du stylo (penup, pendown).
Installation & Utilisation
Prérequis
Rust 
Dépendances : santiago (lexer/parser framework) et svg_fmt.
Exécution
Pour lancer le projet global :
cargo run
Pour tester l'interpréteur interactif (Partie 4), vous pouvez saisir des commandes complexes comme :
repeat 5 [ forward 150 right 144 ]
Cela générera automatiquement un fichier mon_dessin.svg représentant une étoile.
Structure du Code
part1.rs : Analyseur lexical et syntaxique écrit de zéro.
part2.rs : Introduction de la grammaire avec Santiago.
part3.rs : Premier compilateur produisant du code SVG.
part4.rs : Version finale avec mode interactif, gestion des erreurs et boucles imbriquées.
