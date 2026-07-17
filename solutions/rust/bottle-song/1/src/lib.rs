
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut lyrics = String::new();
    let mut start_bottles = start_bottles;
    for _ in 0..take_down{
        lyrics.push_str(&format!(
            "{first} green bottle{s1} hanging on the wall,\n\
            {first} green bottle{s1} hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be {second} green bottle{s2} hanging on the wall.\n\
            \n",
            first = numtoword(start_bottles),
            second = numtoword(start_bottles-1).to_lowercase(),
            s1 = if start_bottles ==1 {""} else {"s"},
            s2 = if (start_bottles-1) ==1 {""} else {"s"},
        ));
        start_bottles -= 1;
    }
    lyrics
}

fn numtoword(num:u32) -> &'static str{
    match num {
        0 => "No",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _=> "_"
    }
}