use unicode_segmentation::UnicodeSegmentation;
use std::str;

fn main() {
    println!("{}",spongebobify("This is Sarcastic Text💖"));
}

/// Make a string into spongebob text
fn spongebobify(text: &str) -> String {
    let capitalized = text.to_uppercase();
    let mut return_string: Vec<char> = Vec::with_capacity(text.len());

    let mut current_char: char;
    // grapheme_indices will create an iterator that is guaranteed to be safe
    for (index, _byte_offset) in UnicodeSegmentation::grapheme_indices(text, true){
        if index % 2 == 0{
            // So we just have to unwrap, instead of checking validity
            current_char = capitalized.chars().nth(index).unwrap();
        }
        else {
            current_char = text.chars().nth(index).unwrap();
        }
        return_string.push(current_char);
    }
    let s = return_string.into_iter().collect();
    return s;
}