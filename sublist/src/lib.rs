use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal => {
            if first_list == second_list {
                return Comparison::Equal;
            }
        }
        Ordering::Less => {
            if first_list.is_empty() || contains_sublist(second_list, first_list) {
                return Comparison::Sublist;
            }
        }
        Ordering::Greater => {
            if second_list.is_empty() || contains_sublist(first_list, second_list) {
                return Comparison::Superlist;
            }
        }
    }

    Comparison::Unequal
}

fn contains_sublist<T: PartialEq>(list: &[T], sublist: &[T]) -> bool {
    list.windows(sublist.len()).any(|window| sublist == window)
}
