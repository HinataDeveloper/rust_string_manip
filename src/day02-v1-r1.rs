fn main() {
    println!("\n");

    let message1 = String::from("hello world ...");

    let message2 = change_to_upper_case(&message1);

    println!("value of message1: {}", message1);

    println!("value of message2: {}", message2);

    println!("\n The End ...\n");
}

fn change_to_upper_case(data: &str) -> String {
    let mut str_upper = String::new();
    for item in data.chars() {
        str_upper.push(item.to_uppercase().next().unwrap());
    }
    str_upper
}
