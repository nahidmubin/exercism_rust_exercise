use crate::Classification::{Abundant, Deficient, Perfect};

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let mut factors: Vec<u64> = Vec::new();

    for i in 1..(num/2 +1) {
        if num.is_multiple_of(i) {
            factors.push(i);
        }
    }

    let factors_sum: u64 = factors.iter().sum();

    if factors_sum > num {
        Some(Abundant)
    }
    else if factors_sum == num {
        Some(Perfect)
    }
    else {
        Some(Deficient)
    }
}
