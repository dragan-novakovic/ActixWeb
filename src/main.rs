use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);

    let mut v = vec![10, 20, 30];

    for i in &mut v {
        *i = *i + 50;
    }
    println!("Hello, world!, {:?}", &v);
}
