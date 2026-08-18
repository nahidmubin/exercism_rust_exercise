
pub fn abbreviate(phrase: &str) -> String {
    //Initialize a sting to store filtered phrase
    let mut words: String = String::new();
    //initialize a flag to check whether previous letter lower or not
    let mut prev_letter_lower = false;

    //filter the phrase
    for ch in phrase.chars() {
        if ch == '-' || !ch.is_ascii_punctuation(){
            if ch.is_ascii_uppercase() && prev_letter_lower{
                words.push(' ');
                words.push(ch);
            }
            else {
                words.push(ch);
            }
        }
        prev_letter_lower = ch.is_ascii_lowercase();
    }

    //Split the phrase with '-' and space
    let words: Vec<&str> = words.split(|ch| ch == '-' || ch == ' ').collect();
    //Initialize a string for acronym
    let mut acronym = String::new();
    
    //Store the first letter in acronym
    for word in words {
        match word.chars().nth(0){
            Some(letter) => acronym.push(letter.to_ascii_uppercase()),
            None => ()
        }
    }
    
    //Return reult
    acronym
}
