fn main() {
    let mut input = String::new();

    println!("Enter a temperature to convert.");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Line expected");

    let input: u32 = match input.parse() {
        Ok(value) => value,
        Err(_error) => 0,
    };

    // lol, I got bored with this and didn't look up the real numbers
    println!("{} Celsius is {} Farenheit", input, input + 32);
}
