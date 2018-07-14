use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    let new_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let mut v = vec![10, 20, 30];

    for i in &mut v {
        *i = *i + 50;
    }
    println!("Hello, world!, {:?}", &new_scores);
}
