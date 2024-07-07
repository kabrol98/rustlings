// This is similar to the previous `from_into` exercise. But this time, we'll
// implement `FromStr` and return errors instead of falling back to a default
// value. Additionally, upon implementing `FromStr`, you can use the `parse`
// method on strings to generate an object of the implementor type. You can read
// more about it in the documentation:
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<u8>()
    ParseInt(ParseIntError),
}
fn validate_person(name: &str, age_string: &str) -> Result<Person, ParsePersonError> {
    if name.is_empty() {
        return Err(ParsePersonError::NoName);
    }
    let age_result: Result<u8, ParseIntError> = age_string.trim().parse::<u8>();
    match age_result {
        Ok(age) => Ok(Person {
            name: name.to_string(),
            age,
        }),
        Err(e) => Err(ParsePersonError::ParseInt(e)),
    }
}

impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entity_list: Vec<&str> = s.split(',').collect();
        if entity_list.len() != 2 {
            Err(ParsePersonError::BadLen)
        } else {
            match validate_person(entity_list[0], entity_list[1]) {
                Ok(p) => Ok(p),
                Err(e) => Err(e),
            }
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
