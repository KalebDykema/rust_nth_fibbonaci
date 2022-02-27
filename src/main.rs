use std::io;

fn main() {
    // Get the input
    println!("Get the input number of the Fibonacci sequence");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse and print the input
    let index: i32 = input.trim().parse().unwrap();
    println!("{}", get_nth_fibonnaci(index));
}

fn get_nth_fibonnaci(i: i32) -> i32 {
    // If the index is 2 or less, can get an accurate result by just substracting 1
    if i <= 2 {
        i - 1
    // Calling the function within itself in this way return an accurate result by looping through itself until it gets to the end
    } else {
        get_nth_fibonnaci(i - 1) + get_nth_fibonnaci(i - 2)
    }
}
