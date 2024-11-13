#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match (1..=num / 2).filter(|i| num % i == 0).sum::<u64>() {
        _ if num == 0 => None,
        x if x > num => Some(Classification::Abundant),
        x if x < num => Some(Classification::Deficient),
        _ => Some(Classification::Perfect),
    }
}
