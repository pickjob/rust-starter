#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn reference_and_move_by_pattern() {
    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    // `name` is moved out of person, but `age` is referenced.
    let Person { name, ref age } = person;
    println!("{} {}", name, age);
}
