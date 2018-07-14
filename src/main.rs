fn main() {
    let mut v = vec![10, 20, 30];

    for i in &mut v {
        *i = *i + 50;
    }
    println!("Hello, world!, {:?}", &v);
}
