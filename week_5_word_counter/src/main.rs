use std::io;

fn main() {
    println!("Input a sentence and this will give you the word count.");

    let mut sentence = String::new();

    io::stdin()
        .read_line(&mut sentence)
        .expect("unable to read string");

    let mut word_count:i16 = 0;

    if sentence.trim().eq(" ") || sentence.trim().eq("") || sentence.trim().is_empty() {
        println!("That's not a sentence");
        return;
    } else {
        space_count(&sentence.trim(), &mut word_count);
        
        println!("There are {} words.", word_count+1);
    }
}

fn space_count(string: &str, word_count:&mut i16) {
    for character in string.chars() {
        if character == ' ' {
            *word_count += 1;
        }
    }
}
