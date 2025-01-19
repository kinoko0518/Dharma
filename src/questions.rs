use crate::input;

#[derive(Clone)]
pub struct OptionalQuestion {
    pub question    : String,
    pub choises     : Vec<String>,
    pub correct_num : i32
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
pub enum Question {
    Descriptional(DescriptionalQuestion),
    Optional(OptionalQuestion),
    MultiDescriptional(MultipleDescriptionalQuestion)
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
        };
        let is_correct = match self {
            Question::Descriptional(d) => d.is_correct(&input::get_input()),
            Question::Optional(o) => o.is_correct(input::get_number_input()),
            Question::MultiDescriptional(md) => md.is_correct(input::get_multiple_input(md.required_answers.len() as i32))
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
    pub fn save(&self) -> String {
        match self {
            Question::Descriptional(d) => format!("D|{}|{}", d.question, d.correct_answers.join(",")),
            Question::Optional(o) => format!("O|{}|{}|{}", o.question, o.choises.join(","), o.correct_num),
            Question::MultiDescriptional(md) => format!("MD|{}|{}", md.question, md.required_answers.join(","))
        }
    }
}