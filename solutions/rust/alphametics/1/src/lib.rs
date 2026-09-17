use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (input_words, output_word, unique_letters, leading_letters) = parse(input);

    if unique_letters.len() > 10 {
        return None;
    }
    let columns: Vec<String> = build_column(&input_words, &output_word);

    let mut assigned_letters: HashMap<char, u8> = HashMap::new();
    let mut used_digits: Vec<u8> = Vec::new();
    let col_index: usize = 0;
    let carry: u8 = 0;

    let success = assign(&columns, &leading_letters, &mut assigned_letters, &unique_letters, &mut used_digits, col_index, carry);

    if success {
        Some(assigned_letters)
    }
    else {
        None
    }
}

fn parse(input_string: &str) -> (Vec<String>, String, HashSet<char>, HashSet<char>) {
    let binding: String = input_string.replace("==", "+");
    let mut input_words: Vec<String> = binding.split("+").map(|w| w.trim().to_string()).collect();
    let leading_letters: HashSet<char> = input_words.iter().map(|words| words.chars().collect::<Vec<char>>()[0]).collect();

    let output_words: String = input_words.pop().expect("Input string is empty.");

    let unique_letters: HashSet<char> = input_string.chars().filter(|ch| ch.is_alphabetic()).collect();

    (input_words, output_words, unique_letters, leading_letters)
}

fn build_column(input_words: &Vec<String>, output_word: &String) -> Vec<String> {
    
    let max_len = output_word.len();
    let mut columns: Vec<String> = Vec::new();

    for i in 0..max_len {
        let mut column = String::new();
        for word in input_words {
            match word.chars().rev().collect::<Vec<char>>().get(i) {
                Some(ch) => column.push(*ch),
                None => ()
            }
        }
        
        column.push(output_word.chars().collect::<Vec<char>>()[max_len-1-i]);

        columns.push(column);
    }
    columns
}

fn assign(columns: &Vec<String>, leading_letters: &HashSet<char>, assigned_letters: &mut HashMap<char, u8>,
            unique_letters: &HashSet<char>, used_digits: &mut Vec<u8>, col_index: usize, carry: u8) -> bool{
    

    if col_index == columns.len() {
        return carry == 0;
    }

    let column = &columns[col_index];
    let mut unassigned_letters = unique_letters - &assigned_letters.keys().cloned().collect();
    
    if unassigned_letters.len() == 0 {
        let (valid_column, new_carry) = check_column(column, assigned_letters, carry);

        if valid_column {
            return assign(columns, leading_letters, assigned_letters, unique_letters, used_digits, col_index+1, new_carry);
        }

        return false;
    }


    let letter_to_assign = unassigned_letters.iter().next().expect("No unassigned letter found").clone();
    unassigned_letters.remove(&letter_to_assign);

    for digit in 0..10_u8 {
        if used_digits.contains(&digit) || (digit == 0 && leading_letters.contains(&letter_to_assign)){
            continue;
        }

        assigned_letters.insert(letter_to_assign, digit);

        used_digits.push(digit);

        if assign(columns, leading_letters, assigned_letters, unique_letters, used_digits, col_index, carry) {
            return true;
        }
        assigned_letters.remove(&letter_to_assign);

        if let Some(pos) = used_digits.iter().position(|&x| x==digit) {
            used_digits.remove(pos);
        }
    }

    false
}

fn check_column(column: &String, assigned_letters: &HashMap<char, u8>, carry_in: u8) -> (bool, u8) {
    let mut left_side = carry_in as u32;
    let mut right_side: u32 = 0;

    for (i, ch) in column.chars().enumerate() {
        if i < column.len()-1 {
            left_side += *assigned_letters.get(&ch).expect("Left Side Column doesn't contain {ch}") as u32;
        }
        else {
            right_side = *assigned_letters.get(&ch).expect("Right Column doesn't contain {ch}") as u32;
        }
    }

    (left_side % 10 == right_side, (left_side / 10) as u8)
}