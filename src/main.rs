use santiago::lexer::{LexerRules};
use santiago::grammar::Grammar;


use svg_fmt::*;

#[derive(Debug)]
struct Program{
    list: Vec<Command>,
}

#[derive(Debug)]
struct Command {
    order: Order,
    number: i32
}

#[derive(Debug, Clone)]
enum Order{
    Forward,
    Backward,
    Left,
    Right,
}

fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        // commands
        "DEFAULT" | "FORWARD" = string "forward";
        "DEFAULT" | "BACKWARD" = string "backward";
        "DEFAULT" | "LEFT" = string "left";
        "DEFAULT" | "RIGHT" = string "right";

        // One more sequential digits from 0 to 9 will be mapped to an "NUMBER"
        "DEFAULT" | "NUMBER" = pattern r"[0-9]+";

        // Whitespace " " will be skipped
        "DEFAULT" | "WS" = pattern r"\s+" => |lexer| lexer.skip();

        "DEFAULT" | "REPEAT" = string "repeat";
        "DEFAULT" | "PENUP" = string "penup";
        "DEFAULT" | "PENDOWN" = string "pendown";
        "DEFAULT" | "LBRACKET" = string "[";
        "DEFAULT" | "RBRACKET" = string "]";
    )
}


/*pub fn grammar() -> Grammar<()>{
    santiago::grammar!(

        "program" => rules "command" "program";
        "program" => empty;  

        "command" => rules "order" "number";

        "order" => rules "forward";
        "order" => rules "backward";
        "order" => rules "left";
        "order" => rules "right";

        "forward" => lexemes "FORWARD";
        "backward" => lexemes "BACKWARD";
        "left" => lexemes "LEFT";
        "right" => lexemes "RIGHT";

        "number" => lexemes "NUMBER";
    )
}*/

fn grammar()-> Grammar<AST>{
    santiago::grammar!(
    "program" => rules "command" "program" => |mut nodes: Vec<AST>| {
                let first = nodes.remove(0);
                let mut rest = match nodes.remove(0) {
                    AST::Program(v) => v,
                    AST::Empty => vec![],
                    _ => vec![],
                };
                let mut result = vec![first];
                result.append(&mut rest);
                AST::Program(result)
            };


    "program" => empty => |_| AST::Empty;

    // command simple
    "command" => rules "order" "number"
        => |nodes: Vec<AST>| {
        let order = match &nodes[0] {
            AST::Order(order) => order.clone(),
            _ => panic!("expected order"),
        };

        let number = match &nodes[1] {
            AST::Number(number) => *number,
            _ => panic!("expected number"),
        };

        AST::Command(order, number)
    };

    "order" => rules "forward" => |_| AST::Order(Order::Forward);
    "order" => rules "backward" => |_| AST::Order(Order::Backward);
    "order" => rules "left" => |_| AST::Order(Order::Left);
    "order" => rules "right" => |_| AST::Order(Order::Right);

    "forward" => lexemes "FORWARD" => |_lexemes| AST::Empty;
    "backward" => lexemes "BACKWARD" => |_lexemes| AST::Empty;
    "left" => lexemes "LEFT" => |_lexemes| AST::Empty;
    "right" => lexemes "RIGHT" => |_lexemes| AST::Empty;

    "number" => lexemes "NUMBER" => |lexemes| {
                let value = lexemes[0].raw.parse::<i32>().unwrap();
                AST::Number(value)
            };


    // Partie 4 (Bonus) /////////////////////////////////////////////////////////////////////:
    "command" => rules "action" "number" => |nodes: Vec<AST>| {
        let AST::Order(order) = &nodes[0] else { panic!() };
        let AST::Number(number) = &nodes[1] else { panic!() };
        AST::Command(order.clone(), *number)
    };

    "command" => rules "loop" => |nodes| nodes[0].clone();
    "command" => rules "block" => |nodes| nodes[0].clone();
    "command" => rules "state" => |nodes| nodes[0].clone();

    "action" => rules "forward" => |_| AST::Order(Order::Forward);
    "action" => rules "backward" => |_| AST::Order(Order::Backward);
    "action" => rules "left" => |_| AST::Order(Order::Left);
    "action" => rules "right" => |_| AST::Order(Order::Right);

    "state" => rules "penup" => |_| AST::State(PenState::Up);
    "state" => rules "pendown" => |_| AST::State(PenState::Down);

    "penup" => lexemes "PENUP" => |_| AST::Empty;
    "pendown" => lexemes "PENDOWN" => |_| AST::Empty;

    "loop" => rules "repeat" "number" "command" => |nodes: Vec<AST>| {
        let AST::Number(n) = &nodes[1] else { panic!() };
        AST::Loop(*n, Box::new(nodes[2].clone()))
    };

    "repeat" => lexemes "REPEAT" => |_| AST::Empty;

    "block" => rules "lbracket" "program" "rbracket"
    => |nodes: Vec<AST>| {
        match &nodes[1] {
            AST::Program(cmds) => AST::Block(cmds.clone()),
            _ => AST::Block(vec![])
        }
    };

    "lbracket" => lexemes "LBRACKET" => |_| AST::Empty;
    "rbracket" => lexemes "RBRACKET" => |_| AST::Empty;
    ///////////////////////////////////////////////////////////////////////////////
    )
}

fn eval(ast: &AST)->(){
    match ast {
        AST::Program(command) => {
            for cmd in command {
                eval(cmd);
            }
            println!("Stop");
        },

        AST::Command(order, number) => {
            match order {
                Order::Forward => println!("Avande de {} unités", number),
                Order::Backward => println!("Recule de {} unités", number),
                Order::Right => println!("Droite de {} degrés", number),
                Order::Left => println!("Gauche de {} degrés", number),
            }
        }
        AST::Empty => {}
        _ => {}
    }
}

#[derive(Debug, Clone)]
enum AST{
    Program(Vec<AST>),
    Command(Order, i32),
    Order(Order),
    Number(i32),
    Empty,

    Loop(i32, Box<AST>),
    Block(Vec<AST>),
    State(PenState)
}

struct Logo{
    x: f32,
    y: f32,
    angle: f32,
    pen_state: PenState,
    svg: String,
}

#[derive(Debug, Clone)]
enum PenState{
    Up,
    Down,
}

impl Logo {
    fn new() -> Self {
        let mut svg = String::new();

        svg.push_str(&format!("{}", BeginSvg { w: 500.0, h: 500.0 }));
        svg.push_str("\n");

        Self {
            x: 100.0,
            y: 300.0,
            angle: 0.0,
            pen_state: PenState::Down,
            svg,
        }
    }

   fn compile(&mut self, ast: &AST) -> String {
        self.process(ast);
        self.svg.push_str(&format!("{}", EndSvg));

        self.svg.clone()
    }

    fn process(&mut self, ast: &AST) {
        match ast {
            AST::Program(commands) => {
                for cmd in commands {
                    self.process(cmd);
                }
            }

            AST::Command(order, number) => {
                let number = *number as f32;
                match order {
                    Order::Forward => self.movement(number),
                    Order::Backward => self.movement(-number),
                    Order::Right => self.angle -= number,
                    Order::Left => self.angle += number,
                }
            }

            // Partie 4 (Bonus) /////////////////////////////////////////////////////
            AST::Loop(count, cmd) => {
                for _ in 0..*count {
                    self.process(cmd);
                }
            }

            AST::Block(commands) => {
                for cmd in commands {
                    self.process(cmd);
                }
            }

            AST::State(state) => {
                self.pen_state = state.clone();
            }
            /////////////////////////////////////////////////////////////////////////////
            AST::Empty => {}
            _ => {}
        }
}

    fn movement(&mut self, number: f32){
        let red = Color {r: 255, g: 0, b: 0};

        let radian = self.angle.to_radians();

        let new_x = self.x + number * radian.cos();
        let new_y = self.y + number * radian.sin();

        if let PenState::Down = self.pen_state {
            self.svg.push_str(&format!("{}\n",line_segment(self.x, self.y, new_x, new_y).color(red)));
        }

        self.x = new_x;
        self.y = new_y;
    }
}

fn main() {
/*
    //
    // Partie 1
    //

    let square = Program {
        list: vec![
            Command { order: Order::Forward, number: 100 },
            Command { order: Order::Right, number: 90 },
            Command { order: Order::Forward, number: 100 },
            Command { order: Order::Right, number: 90 },
            Command { order: Order::Forward, number: 100 },
            Command { order: Order::Right, number: 90 },
            Command { order: Order::Forward, number: 100 },
            Command { order: Order::Right, number: 90 },
        ],
    };

    println!("{:#?}", square);

    */

    //let input = "forward 100";        
    //let input = "forward 100 right 90 forward 100 right 90 forward 100 right 90 forward 100";

    // Partie 5 
    let args: Vec<String> = env::args().collect();
    let input = args[1..].join(" ");
    //

    // Partie 2.a 
    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();
    //println!("{:#?}", lexemes);

    // Partie 2.b
    let grammar = grammar();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes).expect("syntax error")[0];
    //println!("{:#?}", parse_trees);


    // Partie 2.c 
    //println!("{:?}", parse_trees.as_abstract_syntax_tree());
    let ast = parse_trees.as_abstract_syntax_tree();
    //eval(&ast);

    // Partie 3
    let mut logo = Logo::new();
    let svg_fmt = logo.compile(&ast);
    std::fs::write("output.svg", svg_fmt).unwrap();
    
}