use santiago::lexer::{LexerRules, Lexeme};
use santiago::grammar::Grammar;

#[derive(Debug, Clone)]
pub enum AST {
    Program(Box<AST>, Box<AST>),
    Empty,
    Command(Box<AST>, Box<AST>),
    Order(Box<AST>),
    Forward,
    Backward,
    Left,
    Right,
    Number(i32),
}

fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "WS"       = pattern r"\s+" => |lexer| lexer.skip();
        "DEFAULT" | "FORWARD"  = string "forward";
        "DEFAULT" | "BACKWARD" = string "backward";
        "DEFAULT" | "LEFT"     = string "left";
        "DEFAULT" | "RIGHT"    = string "right";
        "DEFAULT" | "NUMBER"   = pattern r"[0-9]+";
    )
}

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

pub fn eval(ast: &AST) {
    match ast {
        AST::Program(command, next_program) => {
            eval(command);
            eval(next_program);
        }
        AST::Empty => {
            println!("Stop");
        }
        AST::Command(order_node, number_node) => {
            let val = if let AST::Number(v) = **number_node { v } else { 0 };

            if let AST::Order(direction) = &**order_node {
                match **direction {
                    AST::Forward  => println!("Avance de {} unités", val),
                    AST::Backward => println!("Recule de {} unités", val),
                    AST::Left     => println!("Tourne à gauche de {} degrés", val),
                    AST::Right    => println!("Tourne à droite de {} degrés", val),
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn main() {
    let input = "forward 100";
    
    let lex_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lex_rules, input).unwrap();
    
    let grammar = grammar();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes).expect("syntax error")[0];
    
    let ast = parse_trees.as_abstract_syntax_tree();
    println!("--- Affichage de l'AST ---");
    println!("{:?}", ast);
    
    println!("\n--- Résultat de l'interpréteur ---");
    eval(&ast);
}