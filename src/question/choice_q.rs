use crate::question as Q;

#[derive(Clone)]
pub struct ChoiseQ {
    pub question : String,
    pub explanation : String,
    pub choices : Vec<String>,
    pub correct_number : u32
}

impl Q::Question for ChoiseQ {
    fn ask(&self) -> bool {
        println!("{}", self.question);
        let mut counter:u32 = 1;
        for choice in &self.choices {
            if counter != 1 {
                print!(" | ");
            }
            print!("{}. {}", counter, choice);
            counter += 1;
        }
        println!();
        Q::get_typed_input::<u32>() == self.correct_number
    }
    fn explanation(&self) -> String {
        self.explanation.clone()
    }
}