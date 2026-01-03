// convert a string to upper case
fn main() {
    println!("\n");

    let str_one = String::from("hello world!");
    let mut str_two = String::new();


    for item in str_one.chars() {
        if let Some(ch) = item.to_uppercase().next() {
            str_two.push(ch);
        }
    }

    println!(" -> {}", str_one);
    println!(" -> {}", str_two);

    println!("\n The End ...\n");
}
