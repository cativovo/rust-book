fn main() {
    immutability();
    mutability();
    constants();
    shadowing();
}

fn immutability() {
    println!("immutability");

    let x = 5;

    println!("The value of x is: {x}");

    // x = 6; // error
}

fn mutability() {
    println!("\nmutability");

    // "mut" keyword is need to make the variable mutable
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");
}

fn constants() {
    println!("\nconstants");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("One minute in seconds is {THREE_HOURS_IN_SECONDS}");
}

fn shadowing() {
    println!("\nshadowing");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
