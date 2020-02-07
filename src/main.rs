use rusttest::people;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    //TODO: LEARN how to ideomatically block private construction <--
    //TODO: LEARN how to do constructors (static and member)

    let mut people = people::People::load_from_file().unwrap_or_default();

    println!("We have {} people in our collection", people.count());
    println!("{}", people);

    if people.count() == 1 {
        let person = people::Person::create_with_title("Bob", "Jeffrey", "Mr");
        people.add(person.unwrap());
    }

    //TODO: build a repl for adding people
    //TODO: fancy console output

    people.save_to_file()
}
