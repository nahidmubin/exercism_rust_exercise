
pub fn find(array: &[i32], key: i32) -> Option<usize> {

    //store the length of the temporary slice taken from given array. Initially the slice is the full array.
    let mut slice_len = array.len();

    // if the slice length is zero. its an empty array. return immediately.
    if slice_len == 0 {
        return None;
    }

    //store the left index of the temporary array
    let mut left_i = 0;
    //store the right index of the temporary array. which is the temporary array length -1. we need ensure that it can't be negative
    let mut right_i = slice_len.checked_sub(1).unwrap_or(0);

    //get the index of the  middle item
    let mut mid_i = get_mid_index(slice_len);

    loop {
        // if the middle value of the temporary array is equal to the given number, then we found desired index
        if key == array[mid_i] {
            return Some(mid_i);
        }
        
        //if the left and right index becomes equal after mid value checking each loop then the array doesn't contain the given number
        if left_i == right_i {
            return None;
        }

        //if the given number is larger than the mid value of the temporary array then -
        else if key > array[mid_i] {
            //new left value will be the right one from the middle value
            left_i = mid_i + 1;
            //new temporary array will be right side items of the current mid value
            slice_len = array[left_i..(right_i + 1)].len();
            //new absolute index of the new temporary array would be the new left index + new mid index
            mid_i = left_i +  get_mid_index(slice_len);
        }
        //if the given number is smaller than the mid value of the temporary array then -
        else {
            //new right value will be the left one from the middle value. Ensure it shouldn't be negative
            right_i = mid_i.checked_sub(1).unwrap_or(0);
            //new temporary array will be left side items of the current mid value
            slice_len =  array[left_i..mid_i].len();
            //new absolute index of the new temporary array would be the new right index - new mid index. Ensure it can't be negative
            mid_i = right_i.checked_sub(get_mid_index(slice_len)).unwrap_or(0);
            
        }
    }
}

fn get_mid_index(slice_length: usize) -> usize {
        //if the array length even then middle value index is one less than the length/2
        if slice_length.is_multiple_of(2) {
            (slice_length / 2).checked_sub(1).unwrap_or(0)
        }
        // if odd its length / 2
        else {
            slice_length / 2
        }

}