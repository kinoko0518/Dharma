mod question;
mod questionset;

use question::AQuestion;

fn main() {
    let q1 = question::choice_q::ChoiseQ {
        question : "ほげほげ？".to_string(),
        explanation : "ほげほげってんだろうが".to_string(),
        choices : vec!["ほげほげ".to_string(), "ふがふが".to_string()],
        correct_number : 1
    };
    let q2 = question::description_q::DescriptionQ {
        question : "ふがふが？".to_string(),
        explanation : "ふがふがってんだろうが".to_string(),
        correct_ans : vec!["ふがふが".to_string()]
    };
    questionset::QuestionSet {
        question_set : vec![AQuestion::Choice(q1), AQuestion::Despription(q2)]
    }.ask();
}