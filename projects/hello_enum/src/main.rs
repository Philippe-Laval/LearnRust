fn main() {
    println!("Hello, enum!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four {:?}", four);
    println!("six {:?}", six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home {:?}", home);
    println!("loopback {:?}", loopback);

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("home {:?}", home);
    println!("loopback {:?}", loopback);

    let home = IpAddr2::V4(127, 0, 0, 1);

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    value_in_cents2(Coin2::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five {:?}", five);
    println!("six {:?}", six);
    println!("none {:?}", none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

     match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }


    let coin = Coin2::Quarter(UsState::Alaska);

    if let Some(description) = describe_state_quarter(coin) {
        println!("{description}");
    }

    let coin = Coin2::Quarter(UsState::Alaska);
    
    let mut state1 : UsState;
    let mut count = 0;
    match coin {
        Coin2::Quarter(state1) => println!("State quarter from {state1:?}!"),
        _ => count += 1,
    }

    
    println!("count {:?}", count);

    let mut state2 : UsState;

    let coin2 = Coin2::Quarter(UsState::Alabama);

    if let Some(description) = describe_state_quarter(coin2) {
        println!("{description}");
    }

    let coin2 = Coin2::Quarter(UsState::Alabama);

    if let Coin2::Quarter(state2) = coin2 {
        println!("State quarter from {state2:?}!");
    } else {
        count += 1;
    }


    println!("count {:?}", count);


}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr3 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn route(ip_kind: IpAddrKind) {
    println!("four {:?}", ip_kind);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn describe_state_quarter(coin: Coin2) -> Option<String> {
    if let Coin2::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

#[derive(Debug)] 
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("Adding a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing a fancy hat!");
}

fn move_player(num_spaces: u8) {
    println!("Moving player by {num_spaces} spaces!");
}

