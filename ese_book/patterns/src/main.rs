//fn main() {
    /*   FIRST EXAMPLE

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),   //this will match any Some()
        _ => println!("Default case, x = {:?}", x),    //this is the x from the outer scope because
                                                        // it isn't shadowed
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
*/


    /*  SECOND EXAMPLE
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),     // we can match multiple putterns | is an or
        3 => println!("three"),
        _ => println!("anything"),
    }
*/

    /*  THIRD EXAMPLE
    let x = 5;

    match x {
        1..=5 => println!("one through five"),   // matches an inclusive range
        _ => println!("something else"),
    }

    // also works with chars
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
*/
//}


/* FOURTH EXAMPLE: STRUCT(s)
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    //let Point { x, y } = p;     // it's sufficient to specify the name of the field

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
*/

/*  NESTED
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}
*/


/* we can even nest _
fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    //

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    let s = Some(String::from("Hello!"));

    //if let Some(_s) = s //this variable can ba unused but still binds so takes ownership
    if let Some(_) = s    //this matches but doesn't take ownership
    {
        println!("found a string");
    }

    println!("{:?}", s);


}
*/


fn main() {
    /*   EXPLAINING ..
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    match numbers {
        (_, second, ..) => { // we can't use (.., second, ..), it's ambiguous
            println!("The second number is {}", second);
        }
    }*/

    /* MATCH GUARD: expresses an ulterior condition that must be true in order to match
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
*/

    /* @ operator: */
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}