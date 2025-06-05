use std::collections::HashMap;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let die_frequency = dice.iter().fold(HashMap::new(), |mut acc, die| {
        acc.entry(*die)
            .and_modify(|frequency| *frequency += 1)
            .or_insert(1);
        acc
    });

    match category {
        Category::Yacht => is_yacht(&die_frequency),
        Category::Ones => count_singular(&die_frequency, 1),
        Category::Twos => count_singular(&die_frequency, 2),
        Category::Threes => count_singular(&die_frequency, 3),
        Category::Fours => count_singular(&die_frequency, 4),
        Category::Fives => count_singular(&die_frequency, 5),
        Category::Sixes => count_singular(&die_frequency, 6),
        Category::FullHouse => full_house(&die_frequency),
        Category::FourOfAKind => four_of_a_kind(&die_frequency),
        Category::LittleStraight => little_straight(&die_frequency),
        Category::BigStraight => big_straight(&die_frequency),
        Category::Choice => choice(&die_frequency),
    }
}

fn is_yacht(die_frequency: &HashMap<u8, u8>) -> u8 {
    if die_frequency.keys().len() == 1 {
        50
    } else {
        0
    }
}

fn count_singular(die_frequency: &HashMap<u8, u8>, value: u8) -> u8 {
    *die_frequency.get(&value).unwrap_or(&0) * value
}

fn full_house(die_frequency: &HashMap<u8, u8>) -> u8 {
    if die_frequency.keys().len() == 2 && die_frequency.values().any(|frequency| *frequency == 2) {
        die_frequency
            .iter()
            .map(|(value, frequency)| value * frequency)
            .sum()
    } else {
        0
    }
}

fn four_of_a_kind(die_frequency: &HashMap<u8, u8>) -> u8 {
    if die_frequency.values().any(|frequency| *frequency >= 4) {
        die_frequency
            .iter()
            .map(
                |(value, frequency)| {
                    if *frequency >= 4 {
                        value * 4
                    } else {
                        0
                    }
                },
            )
            .sum()
    } else {
        0
    }
}

fn little_straight(die_frequency: &HashMap<u8, u8>) -> u8 {
    if die_frequency.keys().len() == 5 && die_frequency.keys().all(|value| *value < 6) {
        30
    } else {
        0
    }
}

fn big_straight(die_frequency: &HashMap<u8, u8>) -> u8 {
    if die_frequency.keys().len() == 5 && die_frequency.keys().all(|value| *value > 1) {
        30
    } else {
        0
    }
}

fn choice(die_frequency: &HashMap<u8, u8>) -> u8 {
    die_frequency
        .iter()
        .map(|(value, frequency)| value * frequency)
        .sum()
}
