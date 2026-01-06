fn main() {
    println!("\n");

    let mut word_vec: Vec<&str> = Vec::new();
    let message = String::from("Hello World and Hello Rust");

    let (word1, len1) = get_first_word(&message);
    word_vec.push(word1);

    let (word2, len2) = get_first_word(&message[len1 + 1..]);
    word_vec.push(word2);

    let (word3, len3) = get_first_word(&message[len1 + len2 + 2..]);
    word_vec.push(word3);

    let (word4, len4) = get_first_word(&message[len1 + len2 + len3 + 3..]);
    word_vec.push(word4);

    let (word5, _len5) = get_first_word(&message[len1 + len2 + len3 + len4 + 4..]);
    word_vec.push(word5);

    for item in word_vec {
        println!(" -> {}", item);
    }

    println!("\n The End ...\n")
}

fn get_first_word(data: &str) -> (&str, usize) {
    let bytes = data.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&data[0..index], index);
        }
    }
    (&data[0..], data.len())
}
