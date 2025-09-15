// Person struct with fields for name, age, and occupation
struct Person {
    name: String,
    age: u32,
    occupation: String,
}

fn main() {
    // TODO: Create instances of the Person struct with sample data
    let john = Person {
        name: String::from("John"),
        age: 28,
        occupation: String::from("Engineer"),
    };
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
        occupation: String::from("Doctor"),
    };
    let bob = Person {
        name: String::from("Bob"),
        age: 23,
        occupation: String::from("Artist"),
    };

    // TODO: Print the instance content to verify the implementation
    // Example output: Name: John, Age: 28, Occupation: Engineer
    println!(
        "Name: {}, Age: {}, Occupation: {}",
        john.name, john.age, john.occupation
    );
    // Example output: Name: Alice, Age: 34, Occupation: Doctor
    println!(
        "Name: {}, Age: {}, Occupation: {}",
        alice.name, alice.age, alice.occupation
    );
    // Example output: Name: Bob, Age: 23, Occupation: Artist
    println!(
        "Name: {}, Age: {}, Occupation: {}",
        bob.name, bob.age, bob.occupation
    );
}
