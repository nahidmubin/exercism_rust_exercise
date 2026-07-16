use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let grapheme: Vec<&str> = input.graphemes(true).rev().collect();
    let mut rev_string = String::new();

    for ch in grapheme{
        rev_string.push_str(ch);
    }

    rev_string
}
