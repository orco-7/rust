struct Color(i32, i32, i32);   // a function that takes Color as argument can't
struct Point(i32, i32, i32);   // take a Point even though they have the same structure
struct AlwaysEqual;            // implement a type without any data
fn main() {
    let black = Color(0, 0, 0);

    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    println!("{}", subject);

}