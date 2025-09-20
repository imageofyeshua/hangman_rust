use std::fs::File;

fn main() {
    let selected_word = select_word();
}

fn selected_word() -> String {
    // open file
    let mut file = File::open("words.txt").expect("Could not open file!");
}
