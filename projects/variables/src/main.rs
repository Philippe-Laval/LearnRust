fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("The value of THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

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

    // https://doc.rust-lang.org/book/ch03-02-data-types.html
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

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // array
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let b: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];
    let c = a[0];
    println!("{c}");

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");

    another_function();
    another_function2(5);
    print_labeled_measurement(5, 'h');

    test_control_flow();
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");

    let y = {
        let x = 3;
        // Note the x + 1 line without a semicolon at the end
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn test_control_flow()
{
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


}