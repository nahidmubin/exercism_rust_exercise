pub fn brackets_are_balanced(string: &str) -> bool {
    //initialize a vector of brackets
    let brackets = vec!['(', ')', '{', '}', '[', ']'];

    //filter the brackets of the string and create a vector
    let mut string: Vec<char> = string.chars().filter(|ch| brackets.contains(ch)).collect();

    //create tuples for the pair of brackets
    let paren = ('(',')');
    let curly = ('{','}');
    let square = ('[',']');

    loop {
        //create flag which will be raised when a immediate pair of brackets found
        let mut found = false;
        //iterate through the string
        for i in 1..string.len() {
            //if consecutive pair of brackets found remove it from the vector and raise the flag that pair found
            if (string[i-1], string[i]) == paren || (string[i-1], string[i]) == curly || (string[i-1], string[i]) == square {
                found = true;
                string.drain(i-1..i+1);
                break;
            }
        }

        //if there is no immediate pair found and the string is empty it is balanced else not balanced
        if !found {
            return string.is_empty();
        }
    }
}
