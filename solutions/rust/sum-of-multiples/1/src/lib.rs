use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    //initialize an empty set for the multiples of magical item
    let mut multiples: HashSet<u32> = HashSet::new();

    //iterate over each magical item
    for factor in factors{
        //initialize multipliers
        let mut i = 1;

        loop {
            //get the multiplied value if
            let multiplied = factor * i;
            //if the value less than the game level, add it to the set, else break the loop
            if multiplied < limit{
                multiples.insert(multiplied);
                //incase factor is 0 break it, otherwise it will create infinite loop
                if factor == &0{
                    break;
                }
                //increase the muliplier by 1 after adding value
                i += 1;
            }
            else {
                break;
            }
        }
    }
    //sum the values of the set and return
    multiples.iter().sum()
}
