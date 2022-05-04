fn main() {
    let v = vec![1, 2, 3];

    let v_iter = v.iter();

    // the for loop takes ownership of the iterator and then makes it mutable so v_iter is no longer
    // available in main() but the values are borrowed, into_iter() returnes owned values
    for val in v_iter {
        println!("Got: {}", val);
    }

    for val in v.iter() {
        println!("{}", val);
    }


    let v: Vec<i32> = vec![1, 2, 3];
                                    //closure
    let v2: Vec<_> = v.iter().map( |x| x +1 ).collect(); //takes each element of v, augments it by 1
                                                         // and stores it in v2
    assert_eq!(v2, vec![2, 3, 4])
}
