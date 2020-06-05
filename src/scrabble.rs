use std::collections::HashMap;

type CharMap = HashMap<char, u32>;

pub struct Scrabble {
    scores: CharMap,
}

impl Scrabble {
    pub fn new() -> Scrabble {
        let scores = Self::build_scores_map();
        Scrabble { scores }
    }

    fn build_scores_map() -> CharMap {
        let strmap = "
            A, E, I, O, U, L, N, R, S, T       1
            D, G                               2
            B, C, M, P                         3
            F, H, V, W, Y                      4
            K                                  5
            J, X                               8
            Q, Z                               10";

        let mut scores = HashMap::new();
        let mut last_score: Option<u32> = None;

        for letter in
            strmap.replace(',', " ").split(' ').map(|e| e.trim()).filter(|e| e.len() > 0).rev()
        {
            match letter.parse::<u32>().ok() {
                Some(val) => 
                    last_score = Some(val),
                _ => {
                    let letter = letter.chars().next().unwrap();
                    scores.insert(letter, last_score.unwrap());
                }
            }
        }
        scores
    }

    pub fn score_word(&self, word: &str) -> u32 {
        word.to_uppercase().chars().map(|letter| self.scores.get(&letter).unwrap()).sum()
    }
}
