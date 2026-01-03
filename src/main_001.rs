// print a string from end to first.

fn main() {
    println!("\n");

    //             0123456789012345678901234
    let message = "I am a Rust Developer ...";

    let lenght: usize = message.len();

    let mut j = 1;

    for item in 0..25 {
        print!("{}", message.get((lenght - j)..(lenght - item)).unwrap());
        j += 1;
    }

    println!("\n The End ... (0.0.1)\n");
}
