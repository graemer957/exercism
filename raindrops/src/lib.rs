pub fn raindrops(n: u32) -> String {
    let is_factor = |factor| n % factor == 0;

    let mut sound = String::new();
    if is_factor(3) {
        sound += "Pling";
    }
    if is_factor(5) {
        sound += "Plang";
    }
    if is_factor(7) {
        sound += "Plong";
    }

    if sound.is_empty() {
        sound += &n.to_string();
    }
    sound
}
