pub mod people {
    use serde::{Deserialize, Serialize};
    use std::fmt;
    use std::io;

    #[derive(Serialize, Deserialize)]
    pub struct Person {
        first_name: String,
        last_name: String,
        title: Option<String>,
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
                title: None,
            };
            Person::validate(person)
        }

        pub fn create_with_title(
            first: &str,
            last: &str,
            title: &str,
        ) -> Result<Person, &'static str> {
            // TODO: is static appropriate as it will only be an error?
            let person = Person {
                first_name: String::from(first),
                last_name: String::from(last),
                title: Some(String::from(title)),
            };
            Person::validate(person)
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct People {
        people_list: Vec<Person>,
    }

    impl People {
        //TODO can we derivedefault instead?
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

        pub fn iter(&self) -> std::slice::Iter<'_, Person> {
            self.people_list.iter()
        }

        pub fn save_to_file(&self) -> io::Result<()> {
            let persist = std::fs::File::create("mydata")?;
            let result = bincode::serialize_into(persist, self);
            match result {
                Ok(_) => Ok(()),
                Err(_) => Err(io::Error::from(io::ErrorKind::InvalidData)),
            }
        }

        pub fn load_from_file() -> io::Result<People> {
            let file = std::fs::File::open("mydata")?;
            let result = bincode::deserialize_from::<std::fs::File, People>(file);
            // we need to do this because there is no error conversion
            match result {
                Ok(p) => Ok(p),
                Err(_) => Err(io::Error::from(io::ErrorKind::InvalidData)),
            }
        }
    }

    impl std::fmt::Display for People {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for per in self.people_list.iter() {
                write!(f, "Person: {}", per)?
            }
            Ok(())
        }
    }

    impl std::default::Default for People {
        fn default() -> Self {
            People::new()
        }
    }
}
