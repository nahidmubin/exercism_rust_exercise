pub fn actions(n: u8) -> Vec<&'static str> {
    let mut activities: Vec<&'static str> = Vec::new();

    // let bits: Vec<char> = format!("{:b}", n).chars().rev().collect();
    let bits: Vec<u8> = (0..u8::BITS).map(|i| n >> i & 1).collect();

    if bits[0] == 1 {
        activities.push("wink");
    }

    if bits[1] == 1 {
        activities.push("double blink");
    }

    if bits[2] == 1 {
        activities.push("close your eyes");
    }

    if bits[3] == 1 {
        activities.push("jump");
    }

    if bits[4] == 1 {
        activities.reverse();
    }

    activities
}
