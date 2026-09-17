
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();

    let mut numbers: Vec<u64> = (2..upper_bound+1).collect();


    while !numbers.is_empty() {
        let candidate_number = numbers[0];
        primes.push(candidate_number);
        numbers = numbers.into_iter().filter(|&n| !n.is_multiple_of(candidate_number)).collect();
    }

    primes
}