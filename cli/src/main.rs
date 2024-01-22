use clap::Parser;

/// Simple calculator program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// First operand
    #[arg(short = 'o', long)]
    operand1: f64,

    /// Arithmetic operation
    #[arg(short = 'p', long)]
    operation: String,

    /// Second operand
    #[arg(short = 'n', long)]
    operand2: f64,
}

fn main() {
    let args = Args::parse();

    let result = match args.operation.as_str() {
        "add" => args.operand1 + args.operand2,
        "subtract" => args.operand1 - args.operand2,
        "multiply" => args.operand1 * args.operand2,
        "divide" => {
            if args.operand2 != 0.0 {
                args.operand1 / args.operand2
            } else {
                println!("Error: Cannot divide by zero!");
                return;
            }
        }
        _ => {
            println!("Error: Invalid operation specified.");
            return;
        }
    };

    println!("Result: {}", result);
}
