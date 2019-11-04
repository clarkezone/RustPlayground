pub mod people {
    use serde::{Deserialize, Serialize};
    use std::fmt;

    #[derive(Serialize, Deserialize)]
    pub struct Person {
        first_name: String,
        // TODO: make last name optional
        last_name: String,
        // TODO: add an image as png
    }

    //TODO: learn: what is '_ lifetime
    impl fmt::Display for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "First: {} Last:{}", self.first_name, self.last_name)
        }
    }

    impl Person {
        fn validate(is_valid: Person) -> Result<Person, &'static str> {
            if is_valid.first_name.len() == 0 || is_valid.last_name.len() == 0 {
                return Err("names need to be longer than 0");
            } else {
                return Ok(is_valid);
            }
        }

        // fn create_first(first: &str) -> Result<Person, &str> {
        //     // LEARNING <- no lifetime specifier since only one param
        //     let p = Person {
        //         first_name: String::from(first),
        //         last_name: String::from(""),
        //     };
        //     Ok(p)
        // }

        pub fn create(first: &str, last: &str) -> Result<Person, &'static str> {
            // TODO: is static appropriate as it will only be an error?
            let person = Person {
                first_name: String::from(first),
                last_name: String::from(last),
            };
            Person::validate(person)
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct People {
        people_list: Vec<Person>,
    }

    impl People {
        pub fn new() -> People {
            let people = People {
                people_list: Vec::new(),
            };
            people
        }

        pub fn count(&self) -> usize {
            self.people_list.len()
        }

        pub fn add(&mut self, per: Person) {
            self.people_list.push(per);
        }

        pub fn save_to_file(&self) -> std::io::Result<()> {
            let persist = std::fs::File::create("mydata")?;
            let result = bincode::serialize_into(persist, self);
            //result?
            Ok(())
        }
    }

    // todo add people collection
    // todo make people displayable
    // todo make it serializable
}
