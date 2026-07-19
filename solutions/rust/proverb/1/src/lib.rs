pub fn build_proverb(list: &[&str]) -> String {
    let list_len = list.len();
    let mut rhyme: String = String::new();

    if list_len >= 2{
        for i in 0..list_len-1{
            rhyme.push_str(&format!("For want of a {} the {} was lost.\n", list[i], if i+1 < list_len {list[i+1]} else{list[0]}));
        }
    }
    if list_len >= 1 {
        rhyme.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    
    rhyme
}