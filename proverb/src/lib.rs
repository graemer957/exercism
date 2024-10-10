use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(object_of_desire) => list
            .windows(2)
            .map(|window| format!("For want of a {} the {} was lost.\n", window[0], window[1]))
            .chain(once(format!(
                "And all for the want of a {}.",
                object_of_desire
            )))
            .collect(),
    }
}
