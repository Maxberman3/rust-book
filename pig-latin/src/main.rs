use std::io::stdin;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    println!("Input the sentence or phrase that you would like to turn into pig latin");

    let mut sentence = String::new();

    stdin()
        .read_line(&mut sentence)
        .expect("failed to read line");

    let words: Vec<&str> = sentence.trim().split_whitespace().collect::<Vec<&str>>();
    let mut result = String::new();

    for word in words {
        if word.len() > 1 {
            if let Some(first_char) = word.chars().next() {
                if VOWELS.iter().any(|char| *char == first_char) {
                    result.push_str(word);
                    result.push_str("hay ");
                } else {
                    let (second_char_index, _): (usize, char) = word.char_indices().nth(1).unwrap();
                    result.push_str(&word[second_char_index..]);
                    result.push_str(&format!("{first_char}ay "));
                }
            }
        }
    }

    println!("{result}");
}
