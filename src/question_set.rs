use std::fs::File;
use std::io::Write;
use std::fs;

use crate::questions;
use crate::input;

pub struct QuestionSet {
    questions:Vec<questions::Question>
}

impl QuestionSet {
    pub fn save(&self, path:&String) {
        let mut file = File::create(&path).expect("Unable to create a file.");
        write!(file, "{}", {
            let mut questions_as_text:Vec<String> = Vec::new();
            for question in &self.questions { questions_as_text.push(question.save()); }
            questions_as_text.join(";\n")
        }).expect("Unable to save a file.");
    }
    pub fn ask(&self) {
        let mut correct_num = 0;
        let mut wronged_question:Vec<questions::Question> = Vec::new();

        for question in &self.questions {
            if question.ask() { correct_num += 1; }
            else { wronged_question.push( (*question).clone() ); }
        }
        println!("\n📋 Result : ({}/{})", correct_num, self.questions.len());
        if correct_num != self.questions.len() {
            if input::get_yn_input("\nWould you try again questions you wronged?", false) {
                QuestionSet{ questions : wronged_question }.ask();
            }
        }
    }
}

pub fn load(path:String) -> Option<QuestionSet> {
    let content = fs::read_to_string(path).expect("⚠️ A file isn't exist or no permission to open a file.");
    let mut res:Vec<Option<questions::Question>> = Vec::new();
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
                "D" => questions::Question::Descriptional(
                    questions::DescriptionalQuestion {
                        question : word[1].to_string(),
                        correct_answers : correct_answers
                    }
                ),
                "O" => questions::Question::Optional(
                    questions::OptionalQuestion {
                        question : word[1].to_string(),
                        choises : word[2].split(",").map(|s| s.to_string()).collect(),
                        correct_num : word[3].parse::<i32>().expect("⚠️ Could not parsed a correct num.")
                    }
                ),
                "MD" => questions::Question::MultiDescriptional(
                    questions::MultipleDescriptionalQuestion {
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
    let mut questions:Vec<questions::Question> = Vec::new();
    for question in res {
        if question.is_some() { questions.push(question.unwrap()); }
        else { return Option::None; }
    }
    Option::Some(QuestionSet { questions: questions })
}