use std::io; 
use std::process::exit;

//STACK VARIABLES:
static SUM:&str = "sum";
static SUB:&str  = "sub";
static MUL:&str  = "mul";
static DIV:&str  = "div";
static POW:&str  = "pow";
static PROMPT_1:&str  = "Set first number:";
static PROMPT_2:&str  = "Set second number:";

fn main() {
    let choice: u8 = get_operation();
    match choice {
            6 => {
                println!("Live long and prosper! 🖖");
                exit(0); 
            },
            _ => {println!("Scelta non valida, riprova.")},
    }

    let n1: f32= get_input_number(&PROMPT_1);
    let n2: f32= get_input_number(&PROMPT_2);
    
    match choice{
        1 => perform_operation(&SUM, n1, n2),
        2 => perform_operation(&SUB, n1, n2),
        3 => perform_operation(&MUL, n1, n2),
        4 => perform_operation(&DIV, n1, n2),
        5 => perform_operation(&POW, n1, n2),
        _ => {
            println!("Scelta non valida, riprova.");
            exit(0); 
        },
    }

}

fn get_operation() -> u8 {
       // CHOICEs
    println!("Choose an operation:");
    println!("1) Addition");
    println!("2) Subtraction");
    println!("3) Multiplication");
    println!("4) Division");
    println!("5) Exponentiation");
    println!("6) Exit");
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => {
                if  num >= 1 && num <= 6 {
                    return num
                }
            },
            Err(_) => println!("Please enter a valid choice."),
        }
    }
}

fn get_input_number(msg:&str) -> f32 {
    println!("{}",msg);
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num, // return inside loop => exit from loop and retrn the value
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn perform_operation(op:&str, n1:f32, n2:f32 ) {
    match op.trim() {
        "sum" => {
                let res = n1 + n2;
                println!("Result: {}",res);
        },
        "sub" => {
                let res = n1 - n2;
                println!("Result: {}",res);
        },
        "mul" => {
                let res = n1 * n2;
                println!("Result: {}",res);
        },
        "div" => {
                let res = n1 / n2;
                println!("Result: {}",res);
        },
        "pow" => {
            let res = n1.powf(n2);
            println!("Result: {}",res);
        },
        _ => println!("Scelta non valida, riprova."),
    }


}
