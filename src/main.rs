
pub mod scrabble;

fn main() {
    println!("rust101");

    let scrabble = scrabble::Scrabble::new();

    println!("score of 'hello' is {}", scrabble.score_word("hello"));
    //println!("score of '1world' is {}", scrabble.score_word("1world"));

}
