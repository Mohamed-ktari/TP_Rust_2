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
        AST::Empty => {} 
        _ => {}
    }
}

fn run(label: &str, input: &str) {
    println!("\n╔══════════════════════════════════════════════════╗");
    println!("║  {}", label);
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

fn main() {
    // ── Test 1 : commande simple ─────────────────────────────
    run("Test 1 — commande simple", "forward 100");
 
    // ── Test 2 : carré complet (8 commandes) ─────────────────
    run(
        "Test 2 — carré (4 × right+forward)",
        "forward 100 right 90 forward 100 right 90 \
         forward 100 right 90 forward 100 right 90",
    );
 
    // ── Test 3 : programme vide ───────────────────────────────
    run("Test 3 — programme vide", "");
 
    // ── Test 4 : erreur lexicale (mot inconnu) ────────────────
    run("Test 4 — erreur lexicale", "forward 50 jump 10");
 
    // ── Test 5 : erreur syntaxique (nombre manquant) ──────────
    run("Test 5 — erreur syntaxique (nombre manquant)", "forward");
 
    // ── Test 6 : erreur syntaxique (ordre manquant) ───────────
    run("Test 6 — erreur syntaxique (deux nombres)", "100 forward");
}