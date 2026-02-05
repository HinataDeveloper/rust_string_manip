fn main() {
    println!("\n");

    let message: String = String::from("     an    apple a      day keeps      the doctor away    .      ");

    let word_list: Vec<&str> = words(&message);

    for item in word_list {
        println!(" -> {item}");
    }

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}

fn words(data: &String) -> Vec<&str> {
    let mut from: usize;
    let mut to: usize = 0;
    let mut word_vec: Vec<&str> = Vec::new();
    let bytes = data.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            from = to;
            to = index;
            let result = data[from..to].trim();
            if result.len() > 0 {
                word_vec.push(result);
            }
        }
    }

    if to < data.len() {
        let result = data[to..].trim();
        if result.len() > 0 {
            word_vec.push(result);
        }
    }

    word_vec
}
