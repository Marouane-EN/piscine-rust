use std::{ collections::HashMap };

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut map = HashMap::new();
    for word in words {
        if map.contains_key(*word) {
            if let Some(v) = map.get_mut(*word) {
                *v += 1;
            }
        } else {
            map.insert(*word, (1).to_owned());
        }
    }
    map
}
pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}

const SENTENCE: &str =
    "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests";

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
        let frequency_count = word_frequency_counter(&words);

        // precise expected map construction
        let expected = HashMap::from([
            ("this", 2),
            ("is", 2),
            ("a", 2),
            ("very", 2),
            ("basic", 3),
            ("sentence", 1),
            ("with", 1),
            ("only", 1),
            ("few", 1),
            ("repetitions.", 1),
            ("once", 1),
            ("again", 1),
            ("but", 1),
            ("it", 1),
            ("should", 1),
            ("be", 1),
            ("enough", 1),
            ("for", 1),
            ("tests", 1),
        ]);

        assert_eq!(frequency_count, expected);
    }
    #[test]
    fn test1() {
        let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
        let frequency_count = word_frequency_counter(&words);
        assert_eq!(nb_distinct_words(&frequency_count), 19)
    }
}
