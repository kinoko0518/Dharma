use crate::input::{self, get_multiple_input};

#[derive(Clone)]
pub struct OptionalQuestion {
    pub question    : String,
    pub choises     : Vec<String>,
    pub correct_num : Vec<u32>
}
#[derive(Clone)]
pub struct DescriptionalQuestion {
    pub question        : String,
    pub correct_answers : Vec<String>
}
#[derive(Clone)]
pub struct MultipleDescriptionalQuestion {
    pub question         : String,
    pub required_answers : Vec<String>
}
#[derive(Clone)]
pub struct OrderedMultiDescriptionalQuestion {
    pub question         : String,
    pub required_answers : Vec<String>
}
impl OptionalQuestion {
    fn is_correct(&self, selected_num:Vec<u32>) -> bool { selected_num == self.correct_num }
}
impl DescriptionalQuestion {
    fn is_correct(&self, answer:&String) -> bool { self.correct_answers.contains(answer) }
}
impl MultipleDescriptionalQuestion {
    fn is_correct(&self, answers:Vec<String>) -> bool {
        if answers.len() != self.required_answers.len() { return false; }
        for req_answer in &self.required_answers {
            if !answers.contains(&req_answer) { return false; }
        }
        return true;
    }
}
impl OrderedMultiDescriptionalQuestion {
    fn is_correct(&self, answers:Vec<String>) -> bool { self.required_answers == answers }
}
#[derive(Clone)]
pub enum Question {
    Descriptional(DescriptionalQuestion),
    Optional(OptionalQuestion),
    MultiDescriptional(MultipleDescriptionalQuestion),
    OrderedMulDescriptional(OrderedMultiDescriptionalQuestion)
}

impl Question {
    pub fn ask(&self) -> bool {
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
            Question::OrderedMulDescriptional(od) => {
                println!("Q. {}", od.question)
            }
        };
        let is_correct = match self {
            Question::Descriptional(d) => d.is_correct(&input::get_input()),
            Question::Optional(o) => o.is_correct(input::get_multiple_num_input(o.correct_num.len() as u32)),
            Question::MultiDescriptional(md) => md.is_correct(input::get_multiple_input(md.required_answers.len() as u32)),
            Question::OrderedMulDescriptional(od) => od.is_correct(get_multiple_input(od.required_answers.len() as u32))
        };
        if is_correct {
            println!("⭕ Correct!");
        } else {
            println!("❌ Incorrect. ✅ Correct Answer -> {}", match self {
                Question::Descriptional(d) => d.correct_answers[0].clone(),
                Question::Optional(o) => o.correct_num.iter().map(|f| o.choises[*f as usize].clone()).collect::<Vec<String>>().join(",").clone(),
                Question::MultiDescriptional(md) => md.required_answers.join(",").clone(),
                Question::OrderedMulDescriptional(od) => od.required_answers.join(",").clone()
            });
        }
        is_correct
    }
}