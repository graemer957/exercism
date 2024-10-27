pub fn find<T: Ord, U: AsRef<[T]>>(array: U, key: T) -> Option<usize> {
    let mut array = array.as_ref();
    let mut base_index = 0;
    loop {
        let mid_point = array.len() / 2;
        let value = array.get(mid_point)?;
        if *value == key {
            return Some(base_index + mid_point);
        } else {
            let (left, right) = array.split_at(mid_point);
            if left.is_empty() || right.is_empty() {
                return None;
            }
            if *value > key {
                array = left;
            } else {
                base_index += mid_point;
                array = right;
            }
        }
    }
}
