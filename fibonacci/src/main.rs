use std::io;

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    loop {
        println!("Enter the position in fibonacci sequence");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read the input");

        let n: i32 = match n.trim().parse() {
            Ok(num) => {
                if num <= 0 {
                    println!("Input must be greater than 0");
                    continue;
                }

                num
            }
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        println!("Fibonacci sequence: {}", fibonacci(n));

        break;
    }
}
