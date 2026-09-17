
pub fn is_valid_isbn(isbn: &str) -> bool {
    //Remove the dashes from isbn
    let isbn = isbn.replace("-", "");

    //get the isbn length and it should be equal to 10
    let isbn_len =  isbn.len();
    if isbn_len != 10 {
        return false;
    }

    //if any alphabet in the ISBN other than the last digit Or the last digit is alphabet but
    //other than X then its invalid ISBN
    for (i, ch) in isbn.chars().enumerate() {
        if (ch.is_alphabetic() && i != isbn_len-1) || (ch.is_alphabetic() && i == isbn_len-1 && ch != 'X') {
            return  false;
        }
    }

    //conver the digits to number
    let isbn: Vec<u32> = isbn.chars().map(|ch| if ch == 'X' { 10_u32 } else{ ch.to_digit(10).unwrap() }).collect();

    //get the sum as per ISBN validation rule
    let mut sum = 0;
    for (i, d) in isbn.iter().enumerate() {
        sum += d * (isbn_len - i) as u32;
    }

    // if the sum is not evenly divided by 11, its in valid ISBN else valid
    if !sum.is_multiple_of(11){
        return false;
    }

    true
}
