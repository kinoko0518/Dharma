use crate::question::{self, get_coloured_8bit, AQuestion, Question};

pub struct QuestionSet {
    pub question_set : Vec<AQuestion>
}
impl QuestionSet {
    pub fn ask(self) {
        let mut questions = self.question_set.clone();
        questions.reverse();
        let mut wronged : Vec<AQuestion> = Vec::new();
        while !questions.is_empty() {
            let q = questions.pop().unwrap();
            let was_correct = match &q {
                AQuestion::Choice(c) => c.ask(),
                AQuestion::Despription(d) => d.ask()
            };
            if !was_correct {
                let exp = match &q {
                    AQuestion::Choice(c) => c.explanation(),
                    AQuestion::Despription(d) => d.explanation()
                };
                print!("{}: {}", get_coloured_8bit("wronged!", 9), exp);
                wronged.push(q);
            } else {
                print!("{}", get_coloured_8bit("correct!", 114))
            }
            println!("\n");
        }
        if !wronged.is_empty() {
            QuestionSet {
                question_set : wronged
            }.ask();
        }
    }
}