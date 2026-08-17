pub fn egg_count(display_value: u32) -> usize {
    //convert the decimal value to binary as a string
    let binary_value = format!("{:b}", display_value);
    
    //return the number of '1' in string
    binary_value.chars().filter(|&ch| ch=='1').count()
}
