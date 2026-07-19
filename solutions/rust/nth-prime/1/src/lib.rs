pub fn nth(n: u32) -> u32 {
    //initialize a vector to store prime number
    let mut prime_numbers: Vec<u32> = Vec::new();
    //initialize the first prime number
    let mut num: u32 = 2;

    // iterate untill nth prime is found
    loop {
        //if the number is prime add it to vector
        if is_prime(num){
            prime_numbers.push(num);
        }
        // if the length of vector become n+1 then the last found prime is the nth prime
        if prime_numbers.len() as u32 == n+1 {
            return num;
        }

        // else increase the number by one and check again
        num += 1;

    }
}


fn is_prime(num: u32) -> bool {
    //iterate through the first prime number to the one before the candidate number
    for i in 2..num {
        //if any of them evenly divide the candidate number then its not prime
        if num % i == 0 {
            return false;
        }
    }
    true
}
