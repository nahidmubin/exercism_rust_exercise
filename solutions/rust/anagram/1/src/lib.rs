use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    //initialize the empty anagram set that would be filled when anagrams are found subsequently
    let mut anagrams = HashSet::new();

    // check for each word in possible anagram list
    'outer: for anagram in possible_anagrams {

        // reject the candidate words that is same as the target word or if it has misnatched length with the target word
        if anagram.to_lowercase() == word.to_lowercase() || anagram.len() != word.len(){
            continue;
        }

        //create a copy of the target word so that it can be mutated
        let mut word_vec: Vec<char> = word.to_lowercase().chars().collect();
        
        //check each character of the rest of the candidate words
        for ch in anagram.to_lowercase().chars(){
            //if the target word doesn't contain any of the character of the candidate word, reject the candidate
            if !word_vec.contains(&ch){
                continue 'outer;
            }

            // if the target word contains a character of a candidate word, then remove character from the
            //target word to prevent using the same character more than once
            if let Some(pos) = word_vec.iter().position(|w| w == &ch){
                word_vec.remove(pos);
            }
        }
        //found an anagram. add it to the set
        anagrams.insert(*anagram);
    }

    //return the final set
    anagrams
}
