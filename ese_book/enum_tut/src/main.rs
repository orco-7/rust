enum UsState {
    Alabama,
    Alaska,
    Nevada,
    Colorado,
    Tennessee,
    Florida,
    Ohio,
    Wyoming,
    Washington,
    SouthDakota,
    NorthDakota,
    Massachute,
    Pensilvanya,
    NewYork,
    NewJersey,
    Kentucky,
    California,
    Texas,
    NewMexico,
    DC,
    Idaho,
    Virginia,
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
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {}", state);
            25
        }
    }
}

fn main() {}