// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut available: HashMap<&str, u32> = HashMap::new();

    for word in magazine {
        if let Some(&word_count) = available.get(word) {
            available.insert(word, word_count + 1);
        } else {
            available.insert(word, 1);
        }
    }

    for word in note {
        if let Some(&word_count) = available.get(word) {
            if (word_count == 0) {
                return false;
            } else {
                available.insert(word, word_count - 1);
            }
        } else {
            return false;
        }
    }

    true
}
