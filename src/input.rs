use std::io::{stdin, stdout, Write};

pub fn get_input() -> String {
    print!("✏️> ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
pub fn get_number_input() -> i32 {
    let mut answer:i32 = 0;
    loop {
        let input = &get_input();
        let input = input.trim();

        if input.parse::<i32>().is_ok() {
            answer = input.parse::<i32>().unwrap();
            break;
        } else {
            println!("⚠️ '{}' is not a number.", input);
        }
    }
    answer
}
pub fn get_yn_input(question:&str, is_n_primary:bool) -> bool {
    let mut value = false;
    loop {
        println!("{} ({}/{})", question, if is_n_primary { "y" } else { "Y" }, if is_n_primary { "N" } else { "n" } );

        let input = get_input();
        let input = input.as_str();

        let interpreted = match input {
            "y"|"Y" => { Option::Some(true) },
            "n"|"N" => { Option::Some(false) },
            _ => { println!("'{}' is invalid input. Please try again.", &input); Option::None }
        };
        if interpreted.is_some() {
            value = interpreted.unwrap();
            break;
        }
    }
    value
}
pub fn get_multiple_input(size:i32) -> Vec<String> {
    let mut inputs:Vec<String> = Vec::new();
    for i in 0..size {
        print!("({}/{})", i+1, size);
        inputs.push(get_input());
    }
    inputs
}