fn main() {
    println!("Hello, world!");

    print_labeled_measurement(five(), 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}
