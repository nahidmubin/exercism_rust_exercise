pub fn collatz(n: u64) -> Option<u64> {
    // make n mutable
    let mut n = n;

    // if n < 1 then its not valid. Return None.
    if n == 0 {
        return None;
    }

    // initialize a step counter
    let mut i: u64 = 0;

    // loop as long as n >1
    while n > 1{
        //check if n is divided by 2 or not then update n and add 1 to the step counter
        if n.is_multiple_of(2){
                n /= 2;
                i+=1;
        }
        else {
            n = n*3 + 1;
            i+=1;
        }
    }
    //Return steps wrapping in some
    Some(i)
}
