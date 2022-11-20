use std::io;

fn main() {
    start();
    println!("Hello, world!");
}

fn start(){
    let mut input = String::new();
    let stdin = io::stdin();
    println!("welcome to the Simple Rust Calculator");
    println!("Type a number to start: ");
    stdin.read_line(&mut input);
    let mut num1: i32 = input.trim().parse().expect("Didnt parsed");
    input.clear();
    println!("Type a second number: ");
    stdin.read_line(&mut input);
    let mut num2: i32 = input.trim().parse().expect("Didnt parsed");
    input.clear();
    println!("What operation would u like to do ?");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("5. Power");
    stdin.read_line(&mut input).expect("no input");
    let choice: String= input.to_owned();
    let choice_sliced:&str = &choice[..].trim();
    let result: i32;
    println!("{}",choice_sliced=="1");
    match choice_sliced {
        "1"=>{
            result = addition(num1, num2);
            println!("Final Result {}", result);
        },
        "2"=>{
            result = subtraction(num1, num2);
            println!("Final Result {}", result);
        },
        "3"=>{
            result = multiplication(num1, num2);
            println!("Final Result {}", result);
        },
        "4"=>{
            result = division(num1, num2);
            println!("Final Result {}", result)
        },
        "5"=>{
            result = powered(num1, num2);
            println!("Final Result {}", result);
        }
        _ => println!("Wrong Input"),
    }
}


fn addition(num1: i32,num2: i32)->i32{
    return num1 + num2;
}

fn subtraction(num1: i32, num2: i32)->i32{
    return num1 - num2;
}

fn multiplication(num1: i32, num2: i32)->i32{
    return num1 * num2;
}

fn division(num1: i32, num2: i32)->i32{
    return num1 / num2;
}

fn powered(num1: i32, num2: i32)-> i32{
    let convertedNum2:u32 = num2 as u32;
    return i32::pow(num1, convertedNum2);
}
