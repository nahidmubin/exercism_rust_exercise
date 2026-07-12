#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    else if first_list.is_empty() {
        return Comparison::Sublist;
    }
    else if second_list.is_empty() {
        return Comparison::Superlist;
    }
    else if first_list.len() < second_list.len() {
        if is_possessed(first_list, second_list) {
            return Comparison::Sublist;
        }
        else {
            return Comparison::Unequal;
        }
    }
    else if first_list.len() > second_list.len(){
        if is_possessed(second_list, first_list) {
            return Comparison::Superlist;
        }
        else {
            return Comparison::Unequal;
        }
    }
    else {
        return Comparison::Unequal;
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