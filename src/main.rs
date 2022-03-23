use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn f_to_c(f: f32) -> f32 {
    return (f - 32.0) / 1.8
}

fn secret_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please enter your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failied to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn fibonacci(n: u32) -> u32 {
    let mut fib = 1;
    if n == 1 || n == 2 {
        return fib;
    } else {
        let mut nm1 = 1;
        let mut nm2 = 1;

        for _ in 2..n {
            fib = nm1 + nm2;
            nm2 = nm1;
            nm1 = fib;
        }
    }
    return fib
}
fn main() {
    
    println!("F° {} is C° {}:", 100, f_to_c(100.0));
    println!("Fibonacci {}", fibonacci(8));
    secret_number();
}
