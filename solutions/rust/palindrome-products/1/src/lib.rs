use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome {
    value: u64,
    factor_min: u64,
    factor_max: u64,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        let mut factors: HashSet<(u64, u64)> = HashSet::new();
        let range = self.factor_min..=self.factor_max;
        for i in 1..((self.value+1)/2)+1 {
            if self.value.is_multiple_of(i) {
                if range.contains(&i) && range.contains(&(self.value/i)) && i <= self.value/i {
                    factors.insert((i, self.value/i));
                }
            }
        }
        factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes: Vec<Palindrome> = Vec::new();
    for i in min..=max {
        for j in min..=max {
            if is_palindrome(i * j) {
                palindromes.push(Palindrome { value: i*j, factor_min: min, factor_max: max });
            }
        }
    }

    palindromes.sort();

    if palindromes.len() >= 1 {
        Some((palindromes[0].clone(), palindromes[palindromes.len()-1].clone()))
    }
    else {
        None
    }
}

pub fn is_palindrome(num: u64) -> bool {
    let num_str = num.to_string();
    let rev_str: String = num_str.chars().rev().collect();

    if num_str == rev_str{
        true
    }
    else {
        false
    }
}