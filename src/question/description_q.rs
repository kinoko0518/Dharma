use crate::question;

use super::Question;

#[derive(Clone)]
pub struct DescriptionQ {
    pub question : String,
    pub explanation : String,
    pub correct_ans : Vec<String>
}
impl Question for DescriptionQ {
    fn ask(&self) -> bool {
        println!("{}", self.question);
        self.correct_ans.contains(&question::get_input(""))
    }
    fn explanation(&self) -> String {
        self.explanation.clone()
    }
}