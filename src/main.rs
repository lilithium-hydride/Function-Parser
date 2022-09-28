use std::io::{stdout, Write};

fn evaluate(function: &str, x: f64 ) -> f64 {
    let mut substr: Vec<String> = Vec::new();
    let mut past = false;
    let mut token_list: Vec<Option<usize>> = Vec::new();
    //println!("{}", function);

    token_list.push(function.find("+"));
    token_list.push(function.find("-"));
    token_list.push(function.find("*"));
    token_list.push(function.find("/"));
    token_list.push(function.find("^"));
    let paren = function.find("(");

    for token in token_list{
        if token != None{
            substr.push(function.chars().skip(0).take(token.unwrap()).collect());
            substr.push(function.chars().skip(token.unwrap()).take(1).collect());
            substr.push(function.chars().skip(token.unwrap() + 1).take(function.chars().count() - token.unwrap()).collect());
            past = true;
            break;
        }
    }

    if past {
        match substr[1].as_str(){
            "+" => return evaluate(&substr[0], x) + evaluate(&substr[2], x),
            "-" => return evaluate(&substr[0], x) - evaluate(&substr[2], x),
            "/" => return evaluate(&substr[0], x) / evaluate(&substr[2], x),
            "*" => return evaluate(&substr[0], x) * evaluate(&substr[2], x),
            "^" => return evaluate(&substr[0], x).powf(evaluate(&substr[2],x)),
            _ => return 0.0
        }

    }
    else if function == "x" {
        return x;
    }
    else {
        return function.parse().unwrap();
    }
}
fn main() {
    let mut function = String::from("");
    let mut input_x = String::from("");
    let mut precision = String::from("");

    println!("Hello, this program will solve for approximate tangent line at a point!");
    print!("Please input the function you are working with: ");
    let _ = stdout().flush();
    std::io::stdin().read_line(&mut function).unwrap();
    print!("Please input the x value you are searching for: ");
    let _ = stdout().flush();
    std::io::stdin().read_line(&mut input_x).unwrap();
    print!("How many decimal points of precision do you need? ");
    let _ = stdout().flush();
    std::io::stdin().read_line(&mut precision).unwrap();

    let precision:i32 = precision.trim().parse().unwrap();
    let input_x:f64 = input_x.trim().parse().unwrap();
    function = function.trim().to_string();

    let mut start = String::from("0.");
    for _ in 0..precision + 1 {
        start.push_str("0");
    }
    start.push_str("1");
    let editor:f64 = start.parse().unwrap();
    let lower:f64 = input_x - editor;
    let upper:f64 = input_x + editor;

    println!("eval of f(x) = {}", evaluate(&function, input_x));
    let estimated_derivative = (evaluate(&function, upper) - evaluate(&function, lower)) / (upper-lower);
    println!("eval of f'(x) = {}", estimated_derivative);
}
