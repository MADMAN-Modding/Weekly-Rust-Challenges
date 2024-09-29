use std::{collections::HashMap, io};

fn main() {
    let mut word:String = String::new();
    let mut anagram:String = String::new();

    let mut word_character_count:HashMap<char, i16> = HashMap::new();
    let mut anagram_character_count:HashMap<char, i16> = HashMap::new();

    println!("Input a word");

    io::stdin()
        .read_line(&mut word)
        .expect("unable to read string");

    println!("Input an anagram");

    io::stdin()
        .read_line(&mut anagram)
        .expect("unable to read string");

    // Used for outputting the anagram and word at the end
    let original_word:String = word.trim().to_owned();
    let original_anagram:String = anagram.trim().to_owned();

    // Trim the strings and convert them to lowercaseString 
    word = word.trim().to_lowercase();
    anagram = anagram.trim().to_lowercase();

    // Makes a map of all the characters in the word
    for character in word.chars() {
        let key_value:i16 = word_character_count.get(&character).get_or_insert(&0).to_owned();
        
        word_character_count.insert(character, key_value + 1);
    }

    // Makes a map of all the characters in the anagram
    for character in anagram.chars() {
        let key_value:i16 = anagram_character_count.get(&character).get_or_insert(&0).to_owned();

        anagram_character_count.insert(character, key_value + 1);
    }

    // Prints whether or the word is an anagram
    println!("{}", if check_anagram(word_character_count, anagram_character_count) {format!("{original_anagram} is an anagram of {original_word}")} else {format!("{original_anagram} is not an anagram of {original_word}")});
}

/* For every key (k) in the word_map, check if the value is equal to that of the anagram map
if the value does not exist, set it to 0, if it does exist and the values aren't equal return false
eventually it will return true
 */
fn check_anagram (word_map: HashMap<char, i16>, anagram_map: HashMap<char, i16>) -> bool {
    for k in word_map.clone() {
        if word_map.get(&k.0).get_or_insert(&0).to_owned() == anagram_map.get(&k.0).get_or_insert(&0).to_owned() {
            continue;
        } else {
            return false;
        }
    }
    return true;
}