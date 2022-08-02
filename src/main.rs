use std::io;
//create a fib function that returns the nth fibonacci number
//fib(0) = 0
//fib(1) = 1
//fib(2) = 3
//fib(3) = 6
//fib(4) = 10
//fib(5) = 15

fn fib(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    println!("Fibonacci sequence:");
    println!("Enter a number to see the Fibonacci sequence up to that number.");
    println!("Enter 'quit' to exit the program.");

    let mut input = String::new();
    loop {
        println!("Please input your number.");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if input.trim() == "quit" {
                    break;
                } else {
                    continue;
                }
            }
        };
        println!("The Fibonacci sequence is:");
        for i in 0..input {
            println!("{}", fib(i));
        }
    }
}
