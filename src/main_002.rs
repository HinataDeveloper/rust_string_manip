fn main() {
    println!("\n");

    let str_one = String::from("Hello, World!");
    let mut str_two = String::new();

    for item in str_one.chars() {
        str_two.push(item.to_uppercase().next().unwrap());
    }

    println!(" -> {}", str_one);
    println!(" -> {}", str_two);

    println!("\n The End ... (0.0.1)\n");
}
