use std::{io::{stdout, Write}, str::FromStr};

pub mod choice_q;
pub mod description_q;

pub trait Question {
    fn ask(&self) -> bool;
    fn explanation(&self) -> String;
}
#[derive(Clone)]
pub enum AQuestion {
    Choice(choice_q::ChoiseQ),
    Despription(description_q::DescriptionQ)
}

pub fn get_coloured_8bit(raw:&str, colour:u8) -> String {
    format!("\x1b[38;5;{}m{}\x1b[m", colour, raw)
}
pub fn get_input(description:&str) -> String {
    print!("{}✏️ > ", description);
    stdout().flush().unwrap();

    let mut word = String::new();
    std::io::stdin().read_line(&mut word).unwrap();
    word.trim().to_string()
}
pub fn get_typed_input<T:FromStr>() -> T {
    match get_input("Number").parse::<T>() {
        Ok(o) => o,
        Err(_) => {
            println!("{}{}", get_coloured_8bit("error: ", 0b001001), "Invalid input! Please try again.");
            get_typed_input::<T>()
        }
    }
}