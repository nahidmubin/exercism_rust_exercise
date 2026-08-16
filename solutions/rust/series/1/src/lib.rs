pub fn series(digits: &str, len: usize) -> Vec<String> {
    //Get the digits length
    let digits_len = digits.len();

    //Initialize the vector to store the sub serieses
    let mut series: Vec<String> = Vec::new();

    //loop from start to the end of the digits and push the sub series into the vector
    for i in 0..digits_len{
        if i+len <= digits_len {
            series.push(digits[i..i+len].to_string());
        }
    }
    //Return final result
    series
}
