use hello::greet;

// use std::collections::HashMap;

fn main() {
    let mut bunnies = 2;
    bunnies = bunnies + 1;

    // const WARP_SPEED: f32 = ask_for_speed();
    const WARP_SPEED: f32 = 9.0;
    println!("Warp speed: {}", WARP_SPEED);
    println!("Bunnies: {}", bunnies);
    scope_blocks();
    shadowing();
    greet();
}

// Scope Example
fn scope_blocks() {
    let a = 5;
    {
        let b = 10;
        println!("Inside the block: {}, {}", a, b);
    }
    // b cannot be used here.
    println!("Outside the block: {}", a);
    do_stuff(1.0, 10.5);
}

fn shadowing() {
    let a = 5;
    // a in the block becomes something new and then reverts back to the original value.
    {
        let a = 10;
        println!("Inside the block: {}", a);
    }
    let a = a + 1;
    let a = a * 2;
    println!("The value of a is: {}", a);
}

fn do_stuff(qty: f64, oz: f64) -> f64 {
    let total: f64 = qty * oz;
    // If you leave the last expression without a semicolon, the compiler will return it.
    total
}
