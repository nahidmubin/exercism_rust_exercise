pub fn raindrops(n: u32) -> String {
    let mut sounds: String = String::new();

    if n.is_multiple_of(3){
        sounds.push_str("Pling");
    }
    if n.is_multiple_of(5){
        sounds.push_str("Plang");
    }
    if n.is_multiple_of(7){
        sounds.push_str("Plong");
    }
    if sounds.is_empty(){
        sounds.push_str(&n.to_string());
    }
    
    sounds
}
