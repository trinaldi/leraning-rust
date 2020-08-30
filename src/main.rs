// Your basic include, import, require, whatever you are familiar with
use rand::Rng; // "Random" Number Generator, I guess.
use std::cmp::Ordering; // ...really?
use std::io;

// Of course, the `main` function
fn main() {
    // Macros, we'll see them later, I hope
    println!("Guess the number...w00t");

    // The magic!
    // "current thread of execution and seeded by the operating system", nice.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Way too easy, right?
    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess:");

        // Create a variable `guess` and initialize it.
        // `mut` means mutable. Variables tend to be immutable!
        let mut guess = String::new();

        // _kinda_ like C++, very _kinda_ way to handle I/O (using references)
        io::stdin()
            .read_line(&mut guess)
            // `expect`is called on `Err` from Result from `read_line`, interesting.
            .expect("Failed to read line from stdin");

        // Thanks to mutability we'll cast String to u32 (default is i32).
        // Nice chain of events here :) It will make sense soon.
        // match is a better way to deal with error handling, here we can see
        // both `Ok` and `Err` from `Result` enum.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed... {}", guess);

        // Using `Ordering` enums in the match expression/pattern.
        // Notice the reference to `secret_number` again.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                // break to, well, stop the game on win.
                break;
            }
        }

    }
}
