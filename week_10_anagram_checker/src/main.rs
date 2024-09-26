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

    // Trim the strings and convert them to lowercase
    word = word.trim().to_string().to_lowercase();
    anagram = anagram.trim().to_string().to_lowercase();


    // Used for outputting the anagram and word at the end
    let original_anagram:String = anagram.to_string();
    let original_word:String = word.to_string();

    // Makes a map of all the characters in the word
    for character in word.chars() {
        let default_value:i16 = 0;
        let key_value = word_character_count.get(&character).get_or_insert(&default_value).to_owned();
        
        word_character_count.insert(character, key_value + 1);
    }

    // Makes a map of all the characters in the anagram
    for character in anagram.chars() {
        let default_value:i16 = 0;
        let key_value:i16 = anagram_character_count.get(&character).get_or_insert(&default_value).to_owned();

        anagram_character_count.insert(character, key_value + 1);
    }

    // Prints whether or the word is an anagram
    println!("{}", if check_anagram(word_character_count, anagram_character_count) {format!("{} is an anagram of {}", original_word, original_anagram)} else {format!("{} not an anagram of {}", original_word, original_anagram)});
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