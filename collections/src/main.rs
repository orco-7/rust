use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let score: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // println!("{}", teams[0]); teams was moved in the hash map

    let team_name = String::from("Blue");
    let scor = score.get(&team_name);
    // println!("{}", scor); this is wrong because the result is a Some/None
    for (key, value) in &score {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let  mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }

    println!("{:?}", map);

}