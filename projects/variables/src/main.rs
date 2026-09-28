fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

    // shadowing

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {spaces}");

    // type

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");

    let x = 0b1111_0000;
    println!("The value of x is: {x}");

    // float

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32


    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;


    let t = true;
    let f: bool = false; // with explicit type annotation


    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
