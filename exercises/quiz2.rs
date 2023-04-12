// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in the form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}



pub mod my_module {
    use super::Command;
    const BAR: &str = "bar"; 

    fn trim_me(input: &str) -> String {
        input.trim().to_string()
    }

    fn append_me(input: &str, n: &usize) -> String {
        let mut newStr = input.to_string();
        for i in 0..*n {
            newStr.push_str(BAR);
        }
        newStr
    }

    fn uppercase_me(input: &str) -> String {
        input.to_string().to_ascii_uppercase()
    }

    pub fn transformer(input: Vec<(&str, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            output.push(match command {
                Command::Trim => trim_me(string),
                Command::Uppercase => uppercase_me(string),
                Command::Append(n) => append_me(string, n)
            })
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
