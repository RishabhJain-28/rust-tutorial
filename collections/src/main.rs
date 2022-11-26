use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let v: Vec<i32> = vec![1, 3];

    print!("{}", v[0]);
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
