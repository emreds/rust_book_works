#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    let penny = Coin::Penny;
    let quarter = Coin::Quarter(UsState::Alabama);
    
    // println!("This is the value in cents {}", value_in_cents(&quarter));
    // println!("This is the value in cents {}", value_in_cents(&quarter));
    // println!("This is the value in cents {}", value_in_cents(&penny));
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let seven = plus_one(six);
    // println!("This is the value of seven {:?}", seven);
    // println!("This is the value of none {:?}", none);

    let c = 4; 
    println!("The result of the dice roll {}", dice_roll(c));


}

fn dice_roll(dice: i8) -> i8 {
    match dice {
        3 => move_three(),
        7 => wrong_dice(),
        _ => reroll(dice)
    }

}

fn move_three() -> i8{3}
fn wrong_dice() -> i8{4}
fn reroll(dice: i8) -> i8 {println!("You roll {}, please reroll", dice); 0}

// I made this function to use borrowed value just for trial.
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1, 
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(number: Option<i32>) -> Option<i32> {
    match number {
        Some(i) => Some(i+1),
        None => None
    }
}
