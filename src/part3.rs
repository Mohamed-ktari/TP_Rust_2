use santiago::grammar::Grammar;
use santiago::lexer::LexerRules;


#[derive(Debug, Clone)]
pub enum AST {
    Program(Box<AST>, Box<AST>), 
    Command(Box<AST>, Box<AST>), 
    Order(Direction),
    Number(i32),
    Empty,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}


fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(

        

        "DEFAULT" | "FORWARD"  = string "forward";
        "DEFAULT" | "BACKWARD" = string "backward";
        "DEFAULT" | "LEFT"     = string "left";
        "DEFAULT" | "RIGHT"    = string "right";

       

        "DEFAULT" | "NUMBER" = pattern r"[0-9]+";


        "DEFAULT" | "WS" = pattern r"\s+" => |lexer| lexer.skip();
    )
}

fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        
        "program" => rules "command" "program" => |nodes : Vec<AST>| AST::Program(
            Box::new(nodes[0].clone()),
            Box::new(nodes[1].clone())
        );
        
        
        "program" => empty => |_| AST::Empty;
 
        "command" => rules "order" "number" => |nodes| AST::Command(
            Box::new(nodes[0].clone()),
            Box::new(nodes[1].clone())
        );
 
        "order" => rules "forward"  => |n| n[0].clone();
        "order" => rules "backward" => |n| n[0].clone();
        "order" => rules "left"     => |n| n[0].clone();
        "order" => rules "right"    => |n| n[0].clone();
 
        "forward"  => lexemes "FORWARD"  => |_| AST::Order(Direction::Forward);
        "backward" => lexemes "BACKWARD" => |_| AST::Order(Direction::Backward);
        "left"     => lexemes "LEFT"     => |_| AST::Order(Direction::Left);
        "right"    => lexemes "RIGHT"    => |_| AST::Order(Direction::Right);
        
        // Extraction de la valeur numérique
        "number"   => lexemes "NUMBER"  => |lexemes| {
            let val = lexemes[0].raw.parse::<i32>().unwrap_or(0);
            AST::Number(val)
        };
    )
}

fn eval(ast: &AST) {
    match ast {
        AST::Program(cmd, next) => {
            eval(cmd);
            eval(next);
        }
        AST::Command(order, number) => {
            let n = if let AST::Number(val) = **number { val } else { 0 };
            match **order {
                AST::Order(Direction::Forward)  => println!("Avance de {} unités", n),
                AST::Order(Direction::Backward) => println!("Recule de {} unités", n),
                AST::Order(Direction::Left)     => println!("Tourne à gauche de {} degrés", n),
                AST::Order(Direction::Right)    => println!("Tourne à droite de {} degrés", n),
                _ => {}
            }
        }
        AST::Empty => {} // Fin du programme, on ne fait rien
        _ => {}
    }
}

fn run( input: &str) {
    println!("\n╔══════════════════════════════════════════════════╗");
    println!("║     Forme");
    println!("╚══════════════════════════════════════════════════╝");
    println!("Source : {:?}\n", input);
 
    
    let rules = lexer_rules();
    let lexemes = match santiago::lexer::lex(&rules, input) {
        Ok(l) => {
            println!("── Lexèmes ({} tokens) ──────────────────────────", l.len());
            for lex in &l {
                println!(
                    "   {:<10}  {:<10}  @ ligne {}, col {}",
                    lex.kind, lex.raw, lex.position.line, lex.position.column
                );
            }
            l
        }
        Err(e) => {
            println!("✗ Erreur lexicale : {}", e);
            return;
        }
    };
 
    // ── Étape 2 : Analyse syntaxique ────────────────────────
    println!("\n── Arbre syntaxique (parse tree) ────────────────");
    let grammar = grammar();
    // ... dans la fonction run ...
    match santiago::parser::parse(&grammar, &lexemes) {
        Ok(parse_trees) => {
            let ast = parse_trees[0].as_abstract_syntax_tree();
            println!("── AST ──────────────────────────────────────────");
            println!("{:?}", ast);
            
            println!("\n── Exécution (Interpréteur) ─────────────────────");
            eval(&ast);
            println!("\n✓ Programme exécuté avec succès.");
        }
        Err(e) => println!("✗ Erreur syntaxique : {}", e),
    }
}



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
                    match direction {
                        Direction::Forward => {
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
                        Direction::Backward => {
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
                        Direction::Left => {
                            self.angle -= val; // Rotation anti-horaire
                        }
                        Direction::Right => {
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

pub fn main(){
    
    // Le programme Logo à compiler
    let input = "forward 100 right 90 forward 100 right 90 forward 100 right 90 forward 100";
    run(input);
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
    
    println!("Aperçu du code :\n{}", code_svg_final);

}