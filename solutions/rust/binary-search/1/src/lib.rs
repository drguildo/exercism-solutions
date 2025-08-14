pub fn find<T: Ord, U: AsRef<[T]>>(array: U, key: T) -> Option<usize> {
    let array = array.as_ref();

    if array.is_empty() {
        return None;
    }

    if array.len() == 1 {
        if array[0] == key {
            return Some(0);
        } else {
            return None;
        }
    }

    let mid = array.len() / 2;
    let split_array = array.split_at(mid);
    if array[mid] > key {
        return find(split_array.0, key);
    } else {
        return find(split_array.1, key).map(|i| i + mid);
    }
}
