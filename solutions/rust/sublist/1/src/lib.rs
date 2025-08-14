#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() < _second_list.len() {
        if is_sublist(_first_list, _second_list) {
            return Comparison::Sublist
        }
    } else if _first_list.len() > _second_list.len() {
        if is_sublist(_second_list, _first_list) {
            return Comparison::Superlist;
        }
    } else if _first_list == _second_list {
        return Comparison::Equal;
    }

    return Comparison::Unequal;
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() > b.len() {
        return false;
    }

    for i in 0..(b.len() - a.len() + 1) {
        let s = &b[i..i + a.len()];
        if s == a {
            return true;
        }
    }

    return false;
}
