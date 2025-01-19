use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::fs;
use std::env;


#[derive(Clone)]
struct OptionalQuestion {
    question    : String,
    choises     : Vec<String>,
    correct_num : i32
}
#[derive(Clone)]
struct DescriptionalQuestion {
    question        : String,
    correct_answers : Vec<String>
}
#[derive(Clone)]
struct MultipleDescriptionalQuestion {
    question         : String,
    required_answers : Vec<String>
}
impl OptionalQuestion {
    fn is_correct(&self, selected_num:i32) -> bool { selected_num == self.correct_num }
}
impl DescriptionalQuestion {
    fn is_correct(&self, answer:&String) -> bool { self.correct_answers.contains(answer) }
}
impl MultipleDescriptionalQuestion {
    fn is_correct(&self, answers:Vec<String>) -> bool { answers == self.required_answers }
}
#[derive(Clone)]
enum Question {
    Descriptional(DescriptionalQuestion),
    Optional(OptionalQuestion),
    MultiDescriptional(MultipleDescriptionalQuestion)
}

struct QuestionSet {
    questions:Vec<Question>
}

fn load(path:String) -> Option<QuestionSet> {
    let content = fs::read_to_string(path).expect("⚠️ A file isn't exist or no permission to open a file.");
    let mut res:Vec<Option<Question>> = Vec::new();
    let lines:Vec<&str> = content.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    
    for line in lines {
        let word:Vec<&str> = line.split('|').filter(|s| !s.is_empty()).collect();
        let loaded =  if !["D", "O", "MD"].contains(&word[0]) {
            { println!("⚠️ Unknown question type. Please make sure that you are running latest version."); Option::None }
        } else if word.len() != match word[0] { "D" => 3, "O" => 4, "MD" => 3, _ => { panic!("⚠️ Something incorrect.") } } {
            {
                let expected_len = match word[0] { "D" => 3, "O" => 4, "MD" => 3, _ => { panic!("⚠️ Something incorrect.") } };
                if word.len() < expected_len {
                    println!("⚠️ The question, {} have no enough element.", line);
                    Option::None
                } else {
                    println!("⚠️ The question, {} have too enough element.", line);
                    Option::None
                }
            }
        } else {
            let correct_answers = word[2].split(",").map(|s| s.to_string()).collect();
            Some( match word[0] {
                "D" => Question::Descriptional(
                    DescriptionalQuestion {
                        question : word[1].to_string(),
                        correct_answers : correct_answers
                    }
                ),
                "O" => Question::Optional(
                    OptionalQuestion {
                        question : word[1].to_string(),
                        choises : word[2].split(",").map(|s| s.to_string()).collect(),
                        correct_num : word[3].parse::<i32>().expect("⚠️ Could not parsed a correct num.")
                    }
                ),
                "MD" => Question::MultiDescriptional(
                    MultipleDescriptionalQuestion {
                        question : word[1].to_string(),
                        required_answers : word[2].split(",").map(|s| s.to_string()).collect()
                    }
                ),
                _ => { panic!("⚠️ Something incorrect."); }
            })
        };
        if loaded.is_some() { res.push(loaded); }
        else { println!("⚠️ A question, {} is broken. The question was ignored.", line) }
    };
    let mut questions:Vec<Question> = Vec::new();
    for question in res {
        if question.is_some() { questions.push(question.unwrap()); }
        else { return Option::None; }
    }
    Option::Some(QuestionSet { questions: questions })
}
fn get_input() -> String {
    print!("✏️> ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
fn get_number_input() -> i32 {
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
fn get_yn_input(question:&str, is_n_primary:bool) -> bool {
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
fn get_multiple_input(size:i32) -> Vec<String> {
    let mut inputs:Vec<String> = Vec::new();
    for i in 0..size {
        print!("({}/{})", i+1, size);
        inputs.push(get_input());
    }
    inputs
}

impl Question {
    fn ask(&self) -> bool {
        println!();
        match self {
            Question::Descriptional(d) => {
                println!("Q. {}", d.question);
            },
            Question::Optional(o) => {
                println!("Q. {}", o.question);
                for i in 0..o.choises.len() { print!("{}. {}  ", i, o.choises[i]); }
                println!();
            }
            Question::MultiDescriptional(md) => {
                println!("Q. {}", md.question);
            }
        };
        let is_correct = match self {
            Question::Descriptional(d) => d.is_correct(&get_input()),
            Question::Optional(o) => o.is_correct(get_number_input()),
            Question::MultiDescriptional(md) => md.is_correct(get_multiple_input(md.required_answers.len() as i32))
        };
        if is_correct {
            println!("⭕ Correct!");
        } else {
            println!("❌ Incorrect. ✅ Correct Answer -> {}", match self {
                Question::Descriptional(d) => d.correct_answers[0].clone(),
                Question::Optional(o) => o.choises[o.correct_num as usize].clone(),
                Question::MultiDescriptional(md) => md.required_answers.join(",").clone()
            });
        }
        is_correct
    }
    fn save(&self, path:&String) -> String {
        match self {
            Question::Descriptional(d) => format!("D|{}|{}", d.question, d.correct_answers.join(",")),
            Question::Optional(o) => format!("O|{}|{}|{}", o.question, o.choises.join(","), o.correct_num),
            Question::MultiDescriptional(md) => format!("MD|{}|{}", md.question, md.required_answers.join(","))
        }
    }
}
impl QuestionSet {
    fn save(&self, path:&String) {
        let mut file = File::create(&path).expect("Unable to create a file.");
        write!(file, "{}", {
            let mut questions_as_text:Vec<String> = Vec::new();
            for question in &self.questions { questions_as_text.push(question.save(&path)); }
            questions_as_text.join(";\n")
        }).expect("Unable to save a file.");
    }
    fn ask(&self) {
        let mut correct_num = 0;
        let mut wronged_question:Vec<Question> = Vec::new();

        for question in &self.questions {
            if question.ask() { correct_num += 1; }
            else { wronged_question.push( (*question).clone() ); }
        }
        println!("\n📋 Result : ({}/{})", correct_num, self.questions.len());
        if correct_num != self.questions.len() {
            if get_yn_input("\nWould you try again questions you wronged?", false) {
                QuestionSet{ questions : wronged_question }.ask();
            }
        }
    }
}

fn main() {
    println!("🗒️  Enter question set name");
    let path = format!("{}/questions/{}.txt", format!("{}", env::current_dir().unwrap().display()).replace("\\", "/"), get_input()).to_string();
    load(path).unwrap().ask();
}