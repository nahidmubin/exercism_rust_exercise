use std::collections::{HashSet};

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets: HashSet<[u32; 3]> = HashSet::new();

    for a in 1..sum+1 {
        for b in 1..sum+1 {
            if a < b {
                let c = sum.saturating_sub(a+b);
                if b < c && (a.pow(2) + b.pow(2)) == c.pow(2) {
                    triplets.insert([a, b, c]);
                }
            }
        }
    }

    triplets
}
