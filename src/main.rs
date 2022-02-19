use std::env::{args, Args};

fn operate(
    first: f32, 
    second: f32, 
    op: &str
) -> f32 {
    match op {
        "+" => first + second,
        "-" => first - second,
        "*" | "x" | "X" => first * second,
        "/" => first / second,
        _ => panic!("unknown operator: {}", op),
    }
}

fn output(
    first_number: f32, 
    second_number: f32, 
    operator: String, 
    result: f32
) -> String {
    format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    )
}

fn main() {
    let mut args: Args = args();
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(first_number, second_number, operator.as_str());

    println!("{}",output(first_number, second_number, operator, result));
}
