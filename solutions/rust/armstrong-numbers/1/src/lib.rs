pub fn is_armstrong_number(num: u32) -> bool {
    //create a vector of digits
    let mut num_vec: Vec<u32> = num.to_string().chars().map(|ch| ch.to_digit(10).unwrap()).collect();

    //get the number of digits in number
    let digit_count: u32 = num_vec.len() as u32;

    //powerize each digit with the digit count
    num_vec = num_vec.iter().map(|d| d.pow(digit_count)).collect();

    //check whether the powerized sum same as the original number or not
    //return true or false based on the checkings
    num_vec.iter().sum::<u32>() == num

}
