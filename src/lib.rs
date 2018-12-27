

type LevenshteinAutomatonState = Vec<u32>;


#[derive(Debug)]
pub struct LevenshteinAutomaton{
    max_edits:u32,
    string: String
}


impl LevenshteinAutomaton {
    pub fn new(string: &str, max_edits: u32) -> Self {
        LevenshteinAutomaton{
            max_edits,
            string: string.to_string()
        }
    }
    pub fn start(&self) -> LevenshteinAutomatonState {
        //the first row of the levenshtein matrix
        let row:Vec<u32> = (0..self.string.len() + 1).map(|i|i as u32).collect();
        row
    }
    pub fn step(&self, state:&LevenshteinAutomatonState, cha:char) -> LevenshteinAutomatonState {
        let mut new_state = vec![state[0]+1];
        println!("{:?}", state);
        for (i, val) in state.iter().take(state.len()-1).enumerate() {
            let cost = if self.string.chars().nth(i).map(|c|c == cha).unwrap_or(false) {
                0
            }else{
                1
            };
            let candidates = &[new_state[i]+1, val + cost, state[i + 1] + 1];
            let yop = candidates.iter().min().unwrap();
            new_state.push(*yop);
        }

        new_state
    }
    pub fn is_match(&self, state:&LevenshteinAutomatonState) -> bool {
        false
    }
    pub fn can_match(&self, state:&LevenshteinAutomatonState) -> bool {
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let automaton = LevenshteinAutomaton::new("banana", 1);
        let start = automaton.start();
        let s1 = automaton.step(&start, 'w');
        println!("{:?}", automaton.can_match(&s1));
        let s2 = automaton.step(&s1, 'o');
        println!("{:?}", automaton.can_match(&s2));

    }
}


