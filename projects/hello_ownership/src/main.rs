fn main() {
    println!("Hello, world!");

    essay();

    let mut s = String::from("hello world");

    let _word = first_word1(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} - {world}");

    let slice1 = &s[0..2];
    let slice2 = &s[..2];

    let len = s.len();
    let slice3 = &s[3..len];
    let slice4 = &s[3..];

    test2();
    test3();
    test4();
}

fn essay()
{
    let _s1 = "hello";
    let _s2 = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`

    println!("Hello, world!");
}

fn first_word1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn test2() {
    let mut s = String::from("hello world");

    let word = first_word2(&s);

    //s.clear(); // error!

    println!("the first word is: {word}");
}
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn test3()
{
    let s = "Hello, world!";

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word3(&my_string[0..6]);
    let word = first_word3(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word3(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word3(&my_string_literal[0..6]);
    let word = first_word3(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word3(my_string_literal);
}

fn test4() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}