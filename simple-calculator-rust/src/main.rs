use std::io;

fn main() {
    start();
    println!("Hello, world!");
}

fn start(){
    //Declare A variable to store the input of the user 
    let mut input = String::new();
    //Declare the Input library in a variable 
    let stdin = io::stdin();
    let mut num1: f32;
    loop {
        println!("welcome to the Simple Rust Calculator");
    println!("Type a number to start: ");
    //It will read the line of the input that the user input 
    stdin.read_line(&mut input);
    if input.trim().is_empty() {
        println!("Please type a number !")
    }else{
    //This will take the input of the user and store in this variable but it needs to be parse an trim and handle error using expect 
         num1 = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}",e);
                return;
            }
        };
        break;
    }
    }
    //To reuse the input we will clear the variable data
    input.clear();
    //Now it will take the second input of the user 
    println!("Type a second number: ");
    //It will read the line of the input that the user input
    stdin.read_line(&mut input);
    //This will take the input of the user and store in this variable but it needs to be parse an trim and handle error using expect 
    let mut num2: f32 = input.trim().parse().expect("Didnt parsed");
    //To reuse the input we will clear the variable data
    input.clear();

    println!("What operation would u like to do ?");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");
    println!("5. Power");
    //It will read the line of the input that the user input
    stdin.read_line(&mut input).expect("no input");
    //Store the input data into a choice variable and use .to_owned by borrowing the data 
    let choice: String= input.to_owned();
    //Which then we slice it to make it into a &str type and trim
    let choice_sliced:&str = &choice[..].trim();
    //Declare a result variable 
    let result: f32;
    println!("{}",choice_sliced=="1");
    //Use a match to match the user's input so that it will declare the function that the user input.
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

//Addition function 
fn addition(num1: f32,num2: f32)->f32{
    return num1 + num2;
}

//Subtraction function 
fn subtraction(num1: f32, num2: f32)->f32{
    return num1 - num2;
}

//Multiplication function
fn multiplication(num1: f32, num2: f32)->f32{
    return num1 * num2;
}

//Division function 
fn division(num1: f32, num2: f32)->f32{
    return num1 / num2;
}

//powered function 
fn powered(num1: f32, num2: f32)-> f32{
    //this will convert a int32 byte into a uint32 by using 'as'
    let convertedNum2:u32 = num2 as u32;
    let convertedNum1:i32 = num1 as i32;
    let result: f32 = i32::pow(convertedNum1, convertedNum2) as f32;
    //use a pow function
    return result;
}
