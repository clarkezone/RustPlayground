use rusttest::people;

fn main() {
    //TODO: LEARN how to ideomatically block private construction <--
    //TODO: LEARN how to do constructors (static and member)
    // let test_instance = people::Person {
    //     first_name: String::from("james"),
    //     last_name: String::from("clarke"),
    // };

    let test_instance = people::Person::create("James", "Clarke");
    let bad_instance = people::Person::create("", "");

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

    println!("We have {} people in our collection", people.count());

    let result = people.save_to_file();
}