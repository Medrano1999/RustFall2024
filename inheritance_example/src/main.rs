trait Talkative {
    fn talk(&self);
}

struct Parrot {
    name: String,
}

impl Talkative for Parrot {
    fn talk(&self) {
        println!("{} the parrot says: Squawk! Polly wants a cracker!", self.name);
    }
}

struct Person {
    name: String,
}

impl Talkative for Person {
    fn talk(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

impl Parrot {
    fn new(name: &str) -> Parrot {
        Parrot {
            name: name.to_string(),
        }
    }
}

impl Person {
    fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }
}

fn main() {
    let parrot = Parrot::new("Polly");
    let person = Person::new("Alice");

    parrot.talk();
    person.talk();
}
