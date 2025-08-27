use rand::seq:SliceRandom;
use std:collections::HashMap;

#[derive(Default)]
struct Brain{
    words: HashMap<String, HashMap<String, usize>>,
}

impl Brain  {
    pub fn train(&mut self, text: &str){
        let mut prev_word = None;

        for word in text.split(' '){
            if let Some(prev_word) = prev_word.replace(word){
                *self
                    .words
                    .entry(prev_word.to_string())
                    .or_default()
                    .entry(word.to_string())
                    .or_default() += 1;
            }
        }
    }
}

    pub fn prompt(&self, prompt: &str, length: usize) -> String {
        let mut out: Vec<_> = prompt
            .split(' ')
            .map(|str| str.to_string())
            .collect();

        let mut rng = rand::thread_rng();

        while out.len() < length{
            // here we add context 
            let last_word = out.last().unwrap();

            if let Some(next_words) = self.words.get(last_word){
                let next_words: Vec<_> = next_words.iter().collect();

                let next_word = next_words
                    .choose_weighted(&mut rng, |(_word,frequency)| *frequency)
                    .unwrap();

                out.push(next_word.0.to_string());
            }else{
                // encounter input not part of training
                break;
            }
        }
     
    }out.join(" ")
}

fn main(){
    let mut brain = Brain::default();
    brain.train(include_str!("../content/1984.txt"));
    println!("{}",brain.prompt("It was a",64));
    
}
