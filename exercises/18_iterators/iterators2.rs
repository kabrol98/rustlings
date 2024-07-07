// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let rest: String = chars.collect();
            format!("{}{}", first.to_uppercase(), rest)
        }
    }
}

fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
    // For each word in the input vector, call `capitalize_first` and
    // collect the results into a new vector.
    let mut output_vector: Vec<String> = vec![];
    for word in words {
        output_vector.push(capitalize_first(word));
    }
    output_vector
}

fn capitalize_words_string(words: &[&str]) -> String {
    // ???
    // Apply `capitalize_first` to each word in the input vector and
    // concatenate them into a single string with spaces between them.
    let mut result = String::new();
    for word in words {
        result.push_str(capitalize_first(word).as_str());
    }
    result
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
