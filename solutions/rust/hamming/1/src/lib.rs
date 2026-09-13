pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    //initialize a varibale to track DNA mismatch count
    let mut distance = 0;

    //for valid distance two strands should be same else None should be returned
    if s1.len() == s2.len(){
        for (a, b) in s1.chars().zip(s2.chars()) {
            //if DNA pairs are not same, increase mismatch count
            if a != b {
                distance += 1;
            }
        }

        return Some(distance);
    }
    None
}

