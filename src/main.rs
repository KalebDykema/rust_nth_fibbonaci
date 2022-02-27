use std::io;

fn main() {
    println!("Get the input number of the Fibonacci sequence");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index: i32 = input.trim().parse().unwrap();
    println!("{}", get_nth_fibonnaci(index));
}

fn get_nth_fibonnaci(i: i32) -> i32 {
    if i <= 2 {
        i - 1
    } else {
        get_nth_fibonnaci(i - 1) + get_nth_fibonnaci(i - 2)
    }
}
