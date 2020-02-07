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

// Todo convert to a test
fn create() {
    let test_instance = people::Person::new("James", "Clarke");
    let bad_instance = people::Person::new("", "");

    let mut people = people::People::new();

    match test_instance {
        Ok(person) => {
            println!("Person is valid {}", person);
            people.add(person); //person is now invalid
        }
        Err(mess) => println!("Person is invalid with reason {}", mess),
    }

    match bad_instance {
        Ok(person) => println!("Person is valid {}", person),
        Err(mess) => println!("Person is invalid with reason {}", mess),
    }
}
