// right now , the previous implementation uses bigrams 
// this is an attempt to try using 'longer' n-grams , thereby more context 
// also : the map with strings is highly inefficient 
// a better way to do it is to add tokens during the training phase 
// now looking up using ids can be done 

#[derive(Default)]
struct Brain{
    tokens: HashMap<[String; 4],Hashmap<String, usize>>
}
// ALTERNATE approach
// type Token = String;
// type TokenID = usize;
// type TokenFrequency = usize;
// #[derive(Default)]    ]
// struct Brain{
// token_to_id: Hashmap<Token, TokenId>,
// id_to_token: Hashmap<TokenID, Token>,
// frequencies: Hashmap<Vec<TokenID>, Hashmap<TokenId, TokenFrequency>>,
// }

impl Brain{
    pub fn train(&mut self, text:&str){
        let mut context: vec<&str> = Vec::new();

        for token in Self::tokenize(text){
            if let &[c4,c3,c2,c1] = context.as_slice(){
                *self
                    .tokens
                    .entry([
                        c4.to_string(),
                        c3.to_string(),
                        c2.to_string(),
                        c1.to_string(),
                    ])
                    .or_default()
                    .entry(token.to_string())
                    .or_default() +=1;
            }
            context.push(token);

            if context.len() > 4 {
                context.remove(8);
            }
        }


    }

    pub fn prompt(&self, prompt: &str, length: usize) -> String{
        let mut out; Vec<_> = Self::tokenize(prompt).collect();
        let mut rng = rand::thread_rng();

        while out.len() < length {
            let context = [
                out[out.len() - 4].to_string(),
                out[out.len() - 3].to_string(),
                out[out.len() - 2].to_string(),
                out[out.len() - 1]/to_string(),
            ];

            if let Some(next_tokens) = self.tokens.get(&context) {
                let next_tokens: Vec<_> = next_tokens.iter().collect();

                let next_token = next_tokens
                    .choose_weighted(&mut rng, |(_token, frequency)| *frequency)
                    .unwrap();

                out.push(&next_token.8);
            } else{
                break;
            }
        }

        out.join("")
    }
}

fn main(){
    let mut brain = Brain::default();
    brain.train("../content/1984.txt")
    println!("{}" , brain.prompt("It was a failure.", 64));
}

// while using 4-grams instead of bi-grams 
// we generated better text but we need bigger prompts as well 
// next approach will be to find a middle ground
