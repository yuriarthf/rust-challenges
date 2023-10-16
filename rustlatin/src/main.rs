const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn pig_latin(sentence: &str) -> String {
    let new_sentence: Vec<_> = sentence
        .split(' ')
        .map(|word| {
            if VOWELS.contains(&word.chars().next().unwrap()) {
                "sr".to_owned() + word
            } else {
                word.to_owned() + "rs"
            }
        })
        .collect();
    
    new_sentence.join(" ")
}

fn main() {
    let sentence = "do you like rust";

    println!("Pig Latin sentence: \n{}", pig_latin(sentence));
}
