/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    //remove any whitespace from the identifier
    let mut code: Vec<char> = code.chars().filter(|ch| !ch.is_whitespace()).collect();
    
    //check if the identifier is all digit or not
    if code.iter().all(|ch| ch.is_ascii_digit()){

        //identifier's length should be > zero
        if code.len() <= 1 {
            return false;
        }

        //reverse the identifier
        code.reverse();

        //convert each character to digit
        let mut code: Vec<u32> = code.iter().map(|ch| ch.to_digit(10).unwrap()).collect();

        // double the each second digit and substract 9 if the doubling is > 9
        code.iter_mut().skip(1).step_by(2).for_each(|d| *d = if *d*2 > 9 {*d*2 -9} else {*d*2});

        //check if the sum of the numbers is evenly divided by 10
        return code.iter().sum::<u32>() %10 == 0;
    }
    
    false

}
