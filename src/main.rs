use rand::prelude::IndexedRandom;
// use rand::rng;

#[derive(Default)]
struct Brain {
    words: Vec<String>,
}

impl Brain{
    pub fn train(&mut self, text: &str){
        self.words = text.split(' ').map(|word| word.to_string()).collect();
    }

    pub fn prompt(&self, prompt: &str, length: usize) -> String {
        let mut out: Vec<_> = prompt
            .split(' ')
            .map(|word| word.to_string())
            .collect();

        let mut rng = rand::rng();
    
        while out.len() < length{
            // the prompt is completed by choosing random words 
            // from the input text
            out.push(self.words.choose(&mut rng).unwrap().clone());

        }
    
        out.join(" ")
    }
}

fn main(){
    let mut brain = Brain::default();
    brain.train(include_str!("../content/1984.txt"));
    println!("{}",brain.prompt("It was a",64));
}
