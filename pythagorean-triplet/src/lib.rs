const TRIPLET_SUM: u32 = 1_000;

pub fn find() -> Option<u32> {
    for a in 1..(TRIPLET_SUM / 3) {
        for b in (a + 1)..(TRIPLET_SUM / 2) {
            let c = TRIPLET_SUM - (a + b);
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some(a * b * c);
            }
        }
    }

    None
}
