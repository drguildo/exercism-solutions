use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
    word_chars.sort_by(|a, b| b.cmp(a));

    let mut matches = HashSet::new();
    for anagram in possible_anagrams {
        if *anagram.to_lowercase() == word.to_lowercase() {
            continue;
        }
        let mut anagram_chars: Vec<char> = anagram.to_lowercase().chars().collect();
        anagram_chars.sort_by(|a, b| b.cmp(a));
        if anagram_chars == word_chars {
            matches.insert(*anagram);
        }
    }
    return matches;
}
