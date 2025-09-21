use std::{fs::File, io::Read};
use rand::{rng, Rng};

struct Letter {
    character: char,
    revealed: bool
}

#[allow(warnings)]
fn main() {
    let selected_word = select_word();

    let mut letters = create_letters(&selected_word);
    display_progress(&letters);
    println!("Selected word was {}", selected_word);
    // println!("The selected word was {}", selected_word);
}

fn select_word() -> String {
    let mut rng = rng();
    /* open file */
    let mut file = File::open("words.txt").expect("Could not open file!");
    
    /* load file contents */
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents)
        .expect("An error has occured while reading the file");

    let available_words: Vec<&str> = file_contents.trim().split(',').collect();

    /* generate random index */
    let random_index = rng.random_range(0..available_words.len());

    String::from(available_words[random_index])

}

fn create_letters(word: &str) -> Vec<Letter> {
    /* create empty vector */
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false
        })
    }

    letters
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:");

    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}
