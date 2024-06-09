use std::collections::HashSet;

mod tests;

fn main() {}

pub struct Solution {}
impl Solution {
    pub fn is_valid(mut word: String) -> bool {
        word = word.to_lowercase();
        let rule1: bool = Solution::length_valid(&word);
        let rule2: bool = Solution::is_only_alphanumeric(&word);
        let rule3: bool = Solution::contains_vowel(&word);
        let rule4: bool = Solution::contains_consonant(&word);
        rule1 && rule2 && rule3 && rule4
    }
    fn length_valid(word: &String) -> bool {
        word.len() >= 3
    }

    fn is_only_alphanumeric(word: &String) -> bool {
        word.chars().all(|c| c.is_alphanumeric())
    }

    fn contains_vowel(word: &String) -> bool {
        word.chars().any(|c| Solution::is_vowel(c))
    }

    fn contains_consonant(word: &String) -> bool {
        word.chars().any(|c| Solution::is_consonant(c))
    }

    fn is_vowel(letter: char) -> bool {
        let vowels: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        vowels.contains(&letter)
    }

    fn is_consonant(letter: char) -> bool {
        let consonants: HashSet<char> = HashSet::from([
            'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v',
            'w', 'x', 'y', 'z',
        ]);
        consonants.contains(&letter)
    }
}
