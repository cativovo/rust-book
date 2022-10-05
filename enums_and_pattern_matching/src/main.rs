fn main() {
    control_flow();
}

fn control_flow() {
    #[derive(Debug)]
    enum UsState {
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(us_state) => {
                println!("\nthe Quarter's state is {:#?}", us_state);
                25
            }
        }
    }

    let penny_value_in_cents = value_in_cents(Coin::Penny);
    println!("Penny's value in cents is {penny_value_in_cents}");

    let nickel_value_in_cents = value_in_cents(Coin::Nickel);
    println!("Nickel's value in cents is {nickel_value_in_cents}");

    let dime_value_in_cents = value_in_cents(Coin::Dime);
    println!("Dime's value in cents is {dime_value_in_cents}");

    let quarter_value_in_cents = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Quarter's value in cents is {quarter_value_in_cents}");
}
