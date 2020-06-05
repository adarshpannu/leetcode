// tests

#[cfg(test)]
mod tests {
    #[test]
    fn test_scrabble() {
        let scrabble = Scrabble::new();
        assert_eq!(scrabble.score_word("hello"), 91);
    }
}
