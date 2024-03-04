use clap::Parser;
use rand::Rng;

/// Scramble letters in each word of given sentence keeping first and last letter in place
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Sentence to scramble, without quotes
    sentence: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    let mut scrambled = vec![];
    for word in args.sentence {
        scrambled.push(scramble(&word));
    }
    println!("{}", scrambled.join(" "));
}

/// Scramble a single word
fn scramble(word: &String) -> String {
    let mut word = word.chars().collect::<Vec<char>>();
    if word.len() > 3 {
        let mut rng = rand::thread_rng();
        for _ in 0..word.len() {
            let one = rng.gen_range(1..word.len()-1);
            let two = rng.gen_range(1..word.len()-1);
            word.swap(one, two);
        }
    }
    word.iter().collect()
}
