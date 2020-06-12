// Scrabble

use std::collections::HashMap;

type CharMap = HashMap<char, u32>;

pub struct Scrabble {
    scores: CharMap,
}

const LETTER_SCORES: &str = "
    A, E, I, O, U, L, N, R, S, T       1
    D, G                               2
    B, C, M, P                         3
    F, H, V, W, Y                      4
    K                                  5
    J, X                               8
    Q, Z                               10";

impl Scrabble {
    pub fn new() -> Scrabble {
        let scores = Self::build_scores_map();
        Scrabble { scores }
    }

    fn build_scores_map() -> CharMap {
        let mut scores = HashMap::new();
        let mut last_score = 0u32;

        for term in LETTER_SCORES
            .replace(',', " ")
            .split(' ')
            .map(|e| e.trim())
            .filter(|e| e.len() > 0)
            .rev()
        {
            match term.parse::<u32>() {
                Ok(val) => last_score = val,
                _ => {
                    let letter = term.chars().next().unwrap();
                    scores.insert(letter, last_score);
                }
            }
        }
        scores
    }

    pub fn score_word(&self, word: &str) -> u32 {
        word.to_uppercase().chars().map(|letter| self.scores.get(&letter).unwrap()).sum()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_scrabble() {
        let scrabble = super::Scrabble::new();
        assert_eq!(scrabble.score_word("hello"), 8);
        assert_eq!(scrabble.score_word("cabbage"), 14);
    }

    #[test]
    fn test_bad_word() {
        let scrabble = super::Scrabble::new();
        assert_eq!(scrabble.score_word("1world"), 14);
    }
}
