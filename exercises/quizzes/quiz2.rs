// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    fn to_uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    fn trim(s: &str) -> String {
        s.trim().to_string()
    }

    fn append_bar(s: &String, times: &usize) -> String {
        s.to_owned() + &"bar".repeat(*times)
    }

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

        for (s, command) in input {
            match command {
                Command::Uppercase => output.push(to_uppercase(&s)),
                Command::Trim => output.push(trim(&s)),
                Command::Append(times) => output.push(append_bar(&s, &times)),
            }
        }

        output
    }
}

fn main() {
    // You can optionally experiment here.

    let input: Vec<(String, Command)> = vec![
        ("hello".to_string(), Command::Uppercase),
        (" all roads lead to rome! ".to_string(), Command::Trim),
        ("foo".to_string(), Command::Append(1)),
        ("bar".to_string(), Command::Append(5)),
    ];

    let output: Vec<String> = my_module::transformer(input);
    println!("{:?}", output);
    // Expected output: ["HELLO", "all roads lead to rome!", "foobar", "barbarbarbarbarbar"]
}

#[cfg(test)]
mod tests {
    use super::Command;
    use crate::my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
