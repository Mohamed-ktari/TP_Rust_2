// TP2 - Compilateur et Interpréteur Logo

use santiago::lexer::{LexerRules, Lexeme};
use santiago::grammar::Grammar;
use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;
use svg_fmt::*;

// Structure de l'Arbre de Syntaxe Abstraite (AST)

#[derive(Debug, Clone)]
pub enum AST {
    Program(Box<AST>, Box<AST>), // Un programme contient une commande et la suite
    Empty,                       // Fin du programme (chaîne vide)
    Command(Box<AST>, Box<AST>), // Une commande contient un ordre et une valeur
    Order(Box<AST>),             // Type de direction
    Forward,
    Backward,
    Left,
    Right,
    Number(i32),                 // Valeur numérique
}

// Analyse Lexicale (Lexer)

fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "WS"       = pattern r"\s+" => |lexer| lexer.skip(); // Ignorer les espaces
        "DEFAULT" | "FORWARD"  = string "forward";
        "DEFAULT" | "BACKWARD" = string "backward";
        "DEFAULT" | "LEFT"     = string "left";
        "DEFAULT" | "RIGHT"    = string "right";
        "DEFAULT" | "NUMBER"   = pattern r"[0-9]+"; // Détection des nombres entiers
    )
}

// Analyse Syntaxique (Grammaire et Parser)

fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        "program" => rules "command" "program" => |nodes: Vec<AST>| AST::Program(
            Box::new(nodes[0].clone()),
            Box::new(nodes[1].clone())
        );
        "program" => empty => |_| AST::Empty;

        "command" => rules "order" "number" => |nodes: Vec<AST>| AST::Command(
            Box::new(nodes[0].clone()),
            Box::new(nodes[1].clone())
        );

        "order" => lexemes "FORWARD"  => |_| AST::Order(Box::new(AST::Forward));
        "order" => lexemes "BACKWARD" => |_| AST::Order(Box::new(AST::Backward));
        "order" => lexemes "LEFT"     => |_| AST::Order(Box::new(AST::Left));
        "order" => lexemes "RIGHT"    => |_| AST::Order(Box::new(AST::Right));

        "number" => lexemes "NUMBER" => |lexemes| {
            AST::Number(lexemes[0].raw.parse::<i32>().unwrap())
        };
    )
}

// Génération du fichier SVG (Compilateur)
// Structure représentant l'état de la tortue
pub struct Logo {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub pen_down: bool,
    pub svg_content: String,
}

impl Logo {
    pub fn new() -> Self {
        Logo {
            x: 100.0,
            y: 100.0,
            angle: 0.0,
            pen_down: true,
            // On initialise le contenu avec l'entête XML et l'ouverture de la balise SVG
            svg_content: format!(
                "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n{}\n",
                svg_fmt::BeginSvg { w: 300.0, h: 300.0 }
            ),
        }
    }

    pub fn compile(&mut self, ast: &AST) -> String {
        match ast {
            AST::Program(command, next_program) => {
                self.compile(command);
                self.compile(next_program);
            }
            AST::Empty => {
                // Quand on atteint la fin du programme (AST::Empty), on ferme la balise SVG.
                // On vérifie qu'elle n'est pas déjà fermée pour éviter les doublons.
                if !self.svg_content.contains("</svg>") {
                    self.svg_content.push_str(&format!("{}\n", svg_fmt::EndSvg));
                }
            }
            AST::Command(order_node, number_node) => {
                let val = if let AST::Number(v) = **number_node { v as f64 } else { 0.0 };

                if let AST::Order(direction) = &**order_node {
                    match **direction {
                        AST::Forward => {
                            let rad = self.angle * std::f64::consts::PI / 180.0;
                            let new_x = self.x + val * rad.cos();
                            let new_y = self.y + val * rad.sin();
                            
                            // On dessine avec svg_fmt uniquement si le stylo est baissé
                            // CORRECTION : On cast les f64 en f32
                            if self.pen_down {
                                let line = svg_fmt::line_segment(self.x as f32, self.y as f32, new_x as f32, new_y as f32).color(svg_fmt::red());
                                self.svg_content.push_str(&format!("  {}\n", line));
                            }
                            
                            self.x = new_x;
                            self.y = new_y;
                        }
                        AST::Backward => {
                            let rad = self.angle * std::f64::consts::PI / 180.0;
                            let new_x = self.x - val * rad.cos();
                            let new_y = self.y - val * rad.sin();
                            
                            // CORRECTION : On cast les f64 en f32
                            if self.pen_down {
                                let line = svg_fmt::line_segment(self.x as f32, self.y as f32, new_x as f32, new_y as f32).color(svg_fmt::red());
                                self.svg_content.push_str(&format!("  {}\n", line));
                            }
                            
                            self.x = new_x;
                            self.y = new_y;
                        }
                        AST::Left => {
                            self.angle -= val; // Rotation anti-horaire
                        }
                        AST::Right => {
                            self.angle += val; // Rotation horaire
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        
        // On retourne la chaîne SVG complète comme exigé par la signature de la fonction
        self.svg_content.clone()
    }
}

fn main() -> std::io::Result<()> {
    // Le programme Logo à compiler
    let input = "forward 100 right 90 forward 100 right 90 forward 100 right 90 forward 100";
    
    // Lexer
    let lex_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lex_rules, input).unwrap();
    
    // Parser
    let grammar = grammar();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes).expect("syntax error")[0];
    let ast = parse_trees.as_abstract_syntax_tree();
    
    // Compilation avec la nouvelle structure Logo (Partie 3)
    let mut compilateur = Logo::new();
    let code_svg_final = compilateur.compile(&ast); // Appel de la nouvelle fonction
    
    // Écriture du résultat dans un fichier
    let mut file = std::fs::File::create("carre.svg")?;
    file.write_all(code_svg_final.as_bytes())?;
    
    println!("Compilation terminée ! Le fichier 'carre.svg' a été généré.");
    println!("Aperçu du code :\n{}", code_svg_final);

    Ok(())
}