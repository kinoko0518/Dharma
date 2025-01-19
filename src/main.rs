use std::env;

mod questions;
mod question_set;
mod input;

fn main() {
    println!("🗒️  Enter question set name");
    let path = format!("{}/questions/{}.txt", format!("{}", env::current_dir().unwrap().display()).replace("\\", "/"), input::get_input()).to_string();
    question_set::load(path).unwrap().ask();
}