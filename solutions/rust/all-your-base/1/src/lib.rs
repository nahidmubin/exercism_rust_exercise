#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    //Input or Output base can't be less than 1
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return  Err(Error::InvalidOutputBase);
    }

    //If number list is empty vector of 0 should be returned
    if number.len() <= 0 {
        return Ok(vec![0]);
    }

    //any number in list can't be >= base
    for n in number {
        if n >= &from_base {
            return Err(Error::InvalidDigit(*n));
        }
    }


    //get the max exponent to be used while converting from any base to base 10 
    let mut exponent = number.len() as u32 -1 ;
    
    //convert to base 10 from any base
    let mut base_10_num: u32 = 0;
    for i in number{
        base_10_num += i* from_base.pow(exponent);
        if exponent > 0 {
            exponent -= 1;
        }
    }

    //convert the base 10 number to the vector of its digits
    let mut converted_num: Vec<u32> = base_10_num.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();


    // convert the base 10 number to other base if the required base isn't 10
    if to_base != 10 {
        converted_num.clear();

        let mut rem: u32;
        
        loop {
            rem = base_10_num % to_base;
            converted_num.push(rem);
            base_10_num = base_10_num / to_base;
            if base_10_num == 0 {
                converted_num.reverse();
                break;
            }
        }
    }

    //Return the final result
    Ok(converted_num)
}
