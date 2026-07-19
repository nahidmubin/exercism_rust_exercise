pub fn factors(n: u64) -> Vec<u64> {
    //make the candidate number mutable
    let mut n = n;
    //create an empty vector to store prime factors
    let mut factors: Vec<u64> = Vec::new();
    //initialize the first prime factor
    let mut i: u64 = 2;

    //if n = 1 return empty factors
    if n !=1 {

        loop {
            //if number is evenly divided store the factor
            if n % i == 0{
                factors.push(i);
                //update the number with remainder
                n = n / i;
                // if remainder is 1 factorization ends. Break the loop
                if n == 1 {
                    break ;
                }
            }
            //if number isn't divided evenly increase the factor by 1
            else {
                i += 1;
            }
        }
    }
    factors
}
