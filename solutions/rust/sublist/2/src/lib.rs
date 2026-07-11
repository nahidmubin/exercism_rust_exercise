#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    }
    else if first_list.is_empty() {
        Comparison::Sublist
    }
    else if second_list.is_empty() {
        Comparison::Superlist
    }
    else if first_list.len() < second_list.len() {
        if is_possessed(first_list, second_list) {
            Comparison::Sublist
        }
        else {
            Comparison::Unequal
        }
    }
    else if first_list.len() > second_list.len(){
        if is_possessed(second_list, first_list) {
            Comparison::Superlist
        }
        else {
            Comparison::Unequal
        }
    }
    else {
        Comparison::Unequal
    }
}


fn is_possessed(possessed: &[i32], possessor: &[i32]) -> bool {
    for (i, item) in possessor.iter().enumerate(){
        if item == &possessed[0] && possessed == &possessor[i..i + possessed.len()]{
            return true;
        }
        else {
            continue;
        }
    }
    false
}