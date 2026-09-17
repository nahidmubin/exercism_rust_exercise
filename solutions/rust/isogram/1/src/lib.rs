use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let phrase: Vec<char> = candidate.chars().filter(|&ch| ch != '-' && ch != ' ').collect();
    let mut phrase_set: HashSet<char> = HashSet::new();

    for ch in phrase.iter() {
        phrase_set.insert(ch.to_ascii_lowercase());
    }

    dbg!(&phrase);
    dbg!(&phrase_set);
    
    phrase.len() == phrase_set.len()
}
