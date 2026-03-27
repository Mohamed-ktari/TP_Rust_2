// TP2 - Compilateur et Interpréteur Logo

use santiago::lexer::LexerRules;
use santiago::grammar::Grammar;
use std::f64::consts::PI;
use std::io::Write;

// Structure de l'Arbre de Syntaxe Abstraite (AST) étendue
#[derive(Debug, Clone)]
pub enum AST {
    Program(Box<AST>, Box<AST>),
    Empty,
    Command(Box<AST>),           
    Action(Box<AST>, Box<AST>),  
    Order(Box<AST>),
    Forward,
    Backward,
    Left,
    Right,
    Number(i32),
    Repeat(i32, Box<AST>),       
    PenUp,
    PenDown,
    Block(Box<AST>),             
}

// Analyse Lexicale (Lexer)
fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "WS"       = pattern r"\s+" => |lexer| lexer.skip();
        "DEFAULT" | "FORWARD"  = string "forward";
        "DEFAULT" | "BACKWARD" = string "backward";
        "DEFAULT" | "LEFT"     = string "left";
        "DEFAULT" | "RIGHT"    = string "right";
        "DEFAULT" | "REPEAT"   = string "repeat";
        "DEFAULT" | "PENUP"    = string "penup";
        "DEFAULT" | "PENDOWN"  = string "pendown";
        "DEFAULT" | "LBRACK"   = string "[";
        "DEFAULT" | "RBRACK"   = string "]";
        "DEFAULT" | "NUMBER"   = pattern r"[0-9]+";
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

        "command" => rules "action_cmd" => |nodes: Vec<AST>| nodes[0].clone();
        "command" => rules "loop"       => |nodes: Vec<AST>| nodes[0].clone();
        "command" => rules "state"      => |nodes: Vec<AST>| nodes[0].clone();
        "command" => rules "block"      => |nodes: Vec<AST>| nodes[0].clone();

        "action_cmd" => rules "order" "number" => |nodes: Vec<AST>| AST::Action(
            Box::new(nodes[0].clone()),
            Box::new(nodes[1].clone())
        );

        "LBRACK" => lexemes "LBRACK" => |_| AST::Empty;
        "RBRACK" => lexemes "RBRACK" => |_| AST::Empty;
        "REPEAT" => lexemes "REPEAT" => |_| AST::Empty;

        "block" => rules "LBRACK" "program" "RBRACK" => |nodes: Vec<AST>| AST::Block(Box::new(nodes[1].clone()));

        "loop" => rules "REPEAT" "number" "command" => |nodes: Vec<AST>| {
            let n = if let AST::Number(val) = nodes[1] { val } else { 0 };
            AST::Repeat(n, Box::new(nodes[2].clone()))
        };

        "state" => lexemes "PENUP"   => |_| AST::PenUp;
        "state" => lexemes "PENDOWN" => |_| AST::PenDown;

        "order" => lexemes "FORWARD"  => |_| AST::Order(Box::new(AST::Forward));
        "order" => lexemes "BACKWARD" => |_| AST::Order(Box::new(AST::Backward));
        "order" => lexemes "LEFT"     => |_| AST::Order(Box::new(AST::Left));
        "order" => lexemes "RIGHT"    => |_| AST::Order(Box::new(AST::Right));

        "number" => lexemes "NUMBER" => |lexemes| {
            AST::Number(lexemes[0].raw.parse::<i32>().unwrap())
        };
    )
}

// Structure Logo (Compilateur + Interpréteur)
pub struct Logo {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub pen_down: bool,
    pub svg_content: String,
}

impl Default for Logo {
    fn default() -> Self {
        Self::new()
    }
}

impl Logo {
    pub fn new() -> Self {
        Logo {
            x: 200.0,
            y: 200.0,
            angle: 0.0,
            pen_down: true,
            svg_content: String::new(), 
        }
    }

    // INTERPRÉTEUR 
    // exécute les commandes et affiche les étapes 
    pub fn interpret(&mut self, ast: &AST) {
        match ast {
            AST::Program(command, next_program) => {
                self.interpret(command);
                self.interpret(next_program);
            }
            AST::Empty => {}
            AST::Action(order_node, number_node) => {
                let val = if let AST::Number(v) = **number_node { v as f64 } else { 0.0 };
                if let AST::Order(direction) = &**order_node {
                    let rad = self.angle * PI / 180.0;
                    match **direction {
                        AST::Forward => {
                            let new_x = self.x + val * rad.cos();
                            let new_y = self.y + val * rad.sin();
                            let mode = if self.pen_down { "(Stylo BAS)" } else { "(Stylo HAUT)" };
                            println!("Avancer de {}: ({:.1}, {:.1}) -> ({:.1}, {:.1}) {}", val, self.x, self.y, new_x, new_y, mode);
                            self.x = new_x;
                            self.y = new_y;
                        }
                        AST::Backward => {
                            let new_x = self.x - val * rad.cos();
                            let new_y = self.y - val * rad.sin();
                            let mode = if self.pen_down { "(Stylo BAS)" } else { "(Stylo HAUT)" };
                            println!("Reculer de {}: ({:.1}, {:.1}) -> ({:.1}, {:.1}) {}", val, self.x, self.y, new_x, new_y, mode);
                            self.x = new_x;
                            self.y = new_y;
                        }
                        AST::Left => {
                            self.angle -= val;
                            println!("Tourner à gauche de {}° (Nouvel angle: {}°)", val, self.angle);
                        }
                        AST::Right => {
                            self.angle += val;
                            println!("Tourner à droite de {}° (Nouvel angle: {}°)", val, self.angle);
                        }
                        _ => {}
                    }
                }
            }
            AST::Repeat(n, body) => {
                println!("--- Début Boucle REPEAT {} fois ---", n);
                for i in 1..=*n {
                    println!("Itération {}/{}", i, n);
                    self.interpret(body);
                }
                println!("--- Fin de Boucle ---");
            }
            AST::Block(inner_program) => {
                self.interpret(inner_program);
            }
            AST::PenUp => {
                self.pen_down = false;
                println!("Action : Lever le stylo");
            }
            AST::PenDown => {
                self.pen_down = true;
                println!("Action : Baisser le stylo");
            }
            _ => {}
        }
    }

    // LE COMPILATEUR SVG 
    pub fn compile(&mut self, ast: &AST) -> String {
        // (La logique de compile reste la même que précédemment pour générer le SVG)
        self.compile_recursive(ast);
        
        format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n{}\n{} \n{}",
            svg_fmt::BeginSvg { w: 400.0, h: 400.0 },
            self.svg_content,
            svg_fmt::EndSvg
        )
    }

    fn compile_recursive(&mut self, ast: &AST) {
        match ast {
            AST::Program(command, next_program) => {
                self.compile_recursive(command);
                self.compile_recursive(next_program);
            }
            AST::Action(order_node, number_node) => {
                let val = if let AST::Number(v) = **number_node { v as f64 } else { 0.0 };
                if let AST::Order(direction) = &**order_node {
                    let rad = self.angle * PI / 180.0;
                    let (new_x, new_y) = match **direction {
                        AST::Forward  => (self.x + val * rad.cos(), self.y + val * rad.sin()),
                        AST::Backward => (self.x - val * rad.cos(), self.y - val * rad.sin()),
                        AST::Left  => { self.angle -= val; (self.x, self.y) },
                        AST::Right => { self.angle += val; (self.x, self.y) },
                        _ => (self.x, self.y),
                    };
                    if self.pen_down && (new_x != self.x || new_y != self.y) {
                        let line = svg_fmt::line_segment(self.x as f32, self.y as f32, new_x as f32, new_y as f32).color(svg_fmt::red());
                        self.svg_content.push_str(&format!("  {}\n", line));
                    }
                    self.x = new_x;
                    self.y = new_y;
                }
            }
            AST::Repeat(n, body) => {
                for _ in 0..*n { self.compile_recursive(body); }
            }
            AST::Block(inner_program) => { self.compile_recursive(inner_program); }
            AST::PenUp => { self.pen_down = false; }
            AST::PenDown => { self.pen_down = true; }
            _ => {}
        }
    }
}

fn main() -> std::io::Result<()> {
    // Programme de test 
    let input = "penup forward 50 pendown repeat 4 [ forward 100 right 90 ]";
    
    let lex_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lex_rules, input).unwrap();
    let grammar = grammar();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes).expect("syntax error")[0];
    let ast = parse_trees.as_abstract_syntax_tree();
    
    // MODE INTERPRÉTEUR
    println!("=== DÉBUT DE L'INTERPRÉTATION ===");
    let mut interpreter = Logo::new();
    interpreter.interpret(&ast);
    
    // MODE COMPILATEUR
    println!("\n=== DÉBUT DE LA COMPILATION SVG ===");
    let mut compiler = Logo::new();
    let code_svg = compiler.compile(&ast);
    
    let mut file = std::fs::File::create("logo_final.svg")?;
    file.write_all(code_svg.as_bytes())?;
    
    println!("Fichier 'logo_final.svg' généré.");
    Ok(())
}