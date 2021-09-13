enum Creature {
    Crab(String),
    Lobster(String),
    Person(String),
}

fn multi_match_if_let_example() {
    let state = Creature::Crab("Ferris");

    if let Creature::Crab(name) | Creature::Person(name) = state {
        println!("This creature's name is: {}", name);
    }
}
