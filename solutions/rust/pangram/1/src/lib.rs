
const ALPHABETS: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn is_pangram(sentence: &str) -> bool {
    ALPHABETS.chars().all(|ch| sentence.to_ascii_lowercase().contains(ch))
}
