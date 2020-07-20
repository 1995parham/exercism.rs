use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut s: HashSet<&str> = HashSet::new();

    for &p in possible_anagrams.iter() {
        if word.to_lowercase() == p.to_lowercase() {
            continue;
        }
        if is_anagram(word, p) {
            s.insert(p);
        }
    }

    return s;
}

fn is_anagram(word: &str, possible_anagram: &str) -> bool {
    let mut characters: HashMap<char, i32> = HashMap::new();

    for b in word.to_lowercase().chars() {
        characters.insert(b, *characters.get(&b).unwrap_or(&0) + 1);
    }

    for b in possible_anagram.to_lowercase().chars() {
        characters.insert(b, *characters.get(&b).unwrap_or(&0) - 1);
    }

    for &v in characters.values() {
        if v != 0 {
            return false;
        }
    }

    return true;
}
