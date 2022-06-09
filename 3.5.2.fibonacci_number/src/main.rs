use std::io;

fn main() {
    const GOLDEN_RATIO: f64 = 1.61803398874989484820;
    let mut number = String::new();

    loop {
        println!("Enter fibonacci sequence index");

        io::stdin().read_line(&mut number)
            .expect("Failed to read line");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let square_root= 5.0_f64.sqrt();

        let fibonacci_number = (GOLDEN_RATIO.powi(number) - (1.0 - GOLDEN_RATIO).powi(number)) / square_root;

        println!("Index {} of Fibonacci number is {}", number, fibonacci_number);

        break;
    }
}
