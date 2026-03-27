// ─────────────────────────────────────────────
//  1. TOKENS  (sortie du Lexer)
// ─────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Order(String), 
    Number(u32), 
}

// ─────────────────────────────────────────────
//  2. AST  (Arbre de Syntaxe Abstraite)
// ─────────────────────────────────────────────


#[derive(Debug)]
struct Command {
    order: String,
    value: u32,
}

type Program = Vec<Command>;

// ─────────────────────────────────────────────
//  3. LEXER  (analyse lexicale)
// ─────────────────────────────────────────────

fn lex(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    for word in source.split_whitespace() {
        let token = match word {
            "forward" | "backward" | "left" | "right" => {
                Token::Order(word.to_string())
            }
            _ => {
                match word.parse::<u32>() {
                    Ok(n) => Token::Number(n),
                    Err(_) => {
                        return Err(format!("Lexer error: mot inconnu '{}'", word));
                    }
                }
            }
        };
        tokens.push(token);
    }

    Ok(tokens)
}

// ─────────────────────────────────────────────
//  4. PARSER  (analyse syntaxique)
// ─────────────────────────────────────────────

struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    /// Renvoie le token courant sans avancer.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    /// Consomme et retourne le token courant.
    fn advance(&mut self) -> Option<Token> {
        if self.cursor < self.tokens.len() {
            let tok = self.tokens[self.cursor].clone();
            self.cursor += 1;
            Some(tok)
        } else {
            None
        }
    }

    // ── règle : <program> ──────────────────────────────────
    fn parse_program(&mut self) -> Result<Program, String> {
        let mut commands = Vec::new();

        
        while self.peek().is_some() {
            let cmd = self.parse_command()?;
            commands.push(cmd);
        }

        Ok(commands)
    }

    // ── règle : <command> ::= <order> <number> ────────────
    fn parse_command(&mut self) -> Result<Command, String> {
        let order = self.parse_order()?;
        let value = self.parse_number()?;
        Ok(Command { order, value })
    }

    // ── règle : <order> ───────────────────────────────────
    fn parse_order(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token::Order(o)) => Ok(o),
            Some(other) => Err(format!(
                "Parser error: ordre attendu, trouvé {:?}",
                other
            )),
            None => Err("Parser error: ordre attendu, fin de fichier".to_string()),
        }
    }

    // ── règle : <number> ::= [0-9]+ ───────────────────────
    fn parse_number(&mut self) -> Result<u32, String> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(n),
            Some(other) => Err(format!(
                "Parser error: nombre attendu, trouvé {:?}",
                other
            )),
            None => Err("Parser error: nombre attendu, fin de fichier".to_string()),
        }
    }
}

// ─────────────────────────────────────────────
//  5. INTERPRÉTEUR  (exécution de l'AST)
// ─────────────────────────────────────────────

/// État de la tortue Logo.
struct Turtle {
    x: f64,
    y: f64,
    angle: f64,
    pen_down: bool,
}

impl Turtle {
    fn new() -> Self {
        Turtle {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
            pen_down: true,
        }
    }

    /// Exécute une liste de commandes et affiche les mouvements.
    fn run(&mut self, program: &Program) {
        for cmd in program {
            self.execute(cmd);
        }
    }

    fn execute(&mut self, cmd: &Command) {
        match cmd.order.as_str() {
            "forward" => {
                let dist = cmd.value as f64;
                let rad = self.angle.to_radians();
                let nx = self.x + dist * rad.sin();
                let ny = self.y + dist * rad.cos();
                if self.pen_down {
                    println!(
                        "  Tracé : ({:.1}, {:.1}) → ({:.1}, {:.1})",
                        self.x, self.y, nx, ny
                    );
                }
                self.x = nx;
                self.y = ny;
            }
            "backward" => {
                let dist = cmd.value as f64;
                let rad = self.angle.to_radians();
                let nx = self.x - dist * rad.sin();
                let ny = self.y - dist * rad.cos();
                if self.pen_down {
                    println!(
                        "  Tracé : ({:.1}, {:.1}) → ({:.1}, {:.1})",
                        self.x, self.y, nx, ny
                    );
                }
                self.x = nx;
                self.y = ny;
            }
            "left" => {
                self.angle -= cmd.value as f64;
                println!("  Rotation gauche : {}° (angle = {}°)", cmd.value, self.angle);
            }
            "right" => {
                self.angle += cmd.value as f64;
                println!("  Rotation droite : {}° (angle = {}°)", cmd.value, self.angle);
            }
            _ => eprintln!("Interpréteur : ordre inconnu '{}'", cmd.order),
        }
    }
}

// ─────────────────────────────────────────────
//  6. POINT D'ENTRÉE
// ─────────────────────────────────────────────

fn compile_and_run(source: &str) {
    println!("═══════════════════════════════════════");
    println!("Source : {}", source);
    println!("───────────────────────────────────────");

    // Étape 1 : Lexer
    let tokens = match lex(source) {
        Ok(t) => {
            println!("Tokens : {:?}", t);
            t
        }
        Err(e) => {
            eprintln!("Erreur lexicale : {}", e);
            return;
        }
    };

    // Étape 2 : Parser
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse_program() {
        Ok(a) => {
            println!("AST    : {:?}", a);
            a
        }
        Err(e) => {
            eprintln!("Erreur syntaxique : {}", e);
            return;
        }
    };

    // Étape 3 : Interpréteur
    println!("Exécution :");
    let mut turtle = Turtle::new();
    turtle.run(&ast);
    println!("Position finale : ({:.1}, {:.1})", turtle.x, turtle.y);
}

fn main() {
    
    compile_and_run("forward 100 right 90 forward 100 right 90 forward 100 right 90 forward 100 right 90");

   
    compile_and_run("forward 50 jump 10");

    
    compile_and_run("forward");
}